use anyhow::{anyhow, Result};
use boards::Board;
use bollard::container::{Config, CreateContainerOptions, LogOutput, LogsOptions, WaitContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::models::HostConfig;
use bollard::Docker;
use futures_util::stream::StreamExt;
use std::path::{Path, PathBuf};

pub async fn build_project(
    board: &Board,
    project_path: &Path,
    tool_name: Option<&str>,
) -> Result<PathBuf> {
    let docker = Docker::connect_with_local_defaults()?;

    let abs_project_path = std::fs::canonicalize(project_path)
        .map_err(|e| anyhow!("Failed to resolve project path {}: {}", project_path.display(), e))?;

    // Resolve build tool: CLI override > auto-detect
    let tool = match tool_name {
        Some(name) => board.get_build_tool(name)?,
        None => board
            .detect_build_tool(&abs_project_path)
            .ok_or_else(|| anyhow!(
                "No compatible build tool detected for {}. Available: {}",
                board.name,
                board.build_tools.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(", ")
            ))?,
    };

    println!("Using build tool '{}' for {}", tool.name, board.name);
    println!("Docker image: {}", tool.docker_image);

    let mut stream = docker.create_image(
        Some(CreateImageOptions {
            from_image: tool.docker_image.clone(),
            ..Default::default()
        }),
        None,
        None,
    );

    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(info) => {
                if let Some(status) = info.status {
                    println!("Docker: {}", status);
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    let container_name = format!("fork-build-{}-{}", board.name, tool.name);
    let _ = docker.remove_container(&container_name, None).await;

    let host_config = HostConfig {
        binds: Some(vec![format!("{}:/project", abs_project_path.display())]),
        ..Default::default()
    };

    let container_config = Config {
        image: Some(tool.docker_image.clone()),
        cmd: Some(tool.build_command.clone()),
        working_dir: Some("/project".to_string()),
        host_config: Some(host_config),
        ..Default::default()
    };

    docker
        .create_container(
            Some(CreateContainerOptions {
                name: container_name.clone(),
                ..Default::default()
            }),
            container_config,
        )
        .await?;

    println!("Building...");
    docker
        .start_container::<String>(&container_name, None)
        .await?;

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
            Ok(LogOutput::StdErr { message }) => eprint!("{}", String::from_utf8_lossy(&message)),
            Err(e) => eprintln!("Error reading logs: {}", e),
            _ => {}
        }
    }

    let mut wait_stream = docker.wait_container(
        &container_name,
        Some(WaitContainerOptions {
            condition: "not-running",
        }),
    );

    if let Some(Ok(wait_response)) = wait_stream.next().await {
        if wait_response.status_code != 0 {
            return Err(anyhow!(
                "Build failed with exit code: {}",
                wait_response.status_code
            ));
        }
    }

    let artifact_path = abs_project_path.join(&tool.artifact_path);
    Ok(artifact_path)
}
