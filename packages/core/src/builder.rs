use anyhow::{Result, anyhow};
use boards::Board;
use bollard::container::{Config, CreateContainerOptions, WaitContainerOptions, LogOutput, LogsOptions};
use bollard::models::HostConfig;
use bollard::image::CreateImageOptions;
use bollard::Docker;
use futures_util::stream::StreamExt;
use std::path::{Path, PathBuf};

pub async fn build_project(board: &Board, project_path: &Path) -> Result<PathBuf> {
    let docker = Docker::connect_with_local_defaults()?;
    
    let abs_project_path = std::fs::canonicalize(project_path)
        .map_err(|e| anyhow!("Failed to resolve project path {}: {}", project_path.display(), e))?;
    
    println!("Using Docker image: {}", board.docker_image);

    let mut stream = docker.create_image(
        Some(CreateImageOptions {
            from_image: board.docker_image.clone(),
            ..Default::default()
        }),
        None,
        None,
    );

    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(info) => {
                if let Some(status) = info.status {
                    println!("Docker Pull: {}", status);
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    let container_name = format!("fork-build-{}", board.name);
    let _ = docker.remove_container(&container_name, None).await;

    let host_config = HostConfig {
        binds: Some(vec![format!("{}:/project", abs_project_path.display())]),
        ..Default::default()
    };

    let config = Config {
        image: Some(board.docker_image.clone()),
        cmd: Some(board.build_command.clone()),
        working_dir: Some("/project".to_string()),
        host_config: Some(host_config),
        ..Default::default()
    };

    docker.create_container(
        Some(CreateContainerOptions {
            name: container_name.clone(),
            ..Default::default()
        }),
        config,
    ).await?;

    println!("Starting build for {}...", board.name);
    docker.start_container::<String>(&container_name, None).await?;

    let mut logs = docker.logs(
        &container_name,
        Some(LogsOptions::<String> {
            stdout: true,
            stderr: true,
            follow: true,
            ..Default::default()
        }),
    );

    while let Some(log) = logs.next().await {
        match log {
            Ok(LogOutput::StdOut { message }) => print!("{}", String::from_utf8_lossy(&message)),
            Ok(LogOutput::StdErr { message }) => eprintln!("{}", String::from_utf8_lossy(&message)),
            Err(e) => eprintln!("Error reading logs: {}", e),
            _ => {}
        }
    }

    let mut wait_stream = docker.wait_container(&container_name, Some(WaitContainerOptions {
        condition: "not-running",
    }));

    if let Some(Ok(wait_response)) = wait_stream.next().await {
        if wait_response.status_code != 0 {
            return Err(anyhow!("Build failed with exit code: {}", wait_response.status_code));
        }
    }

    let artifact_path = match board.name.as_str() {
        "rp2040" => abs_project_path.join("build/firmware.uf2"),
        "esp32c3" => abs_project_path.join("build/partition-table/partition-table.bin"),
        _ => abs_project_path.join("target/release/firmware.bin"),
    };

    Ok(artifact_path)
}
