use anyhow::{Result, anyhow};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn detect_runtime() -> Result<String> {
    if which::which("podman").is_ok() {
        return Ok("podman".to_owned());
    }
    if which::which("docker").is_ok() {
        return Ok("docker".to_owned());
    }
    Err(anyhow!(
        "Neither podman nor docker found. Please install one to use fork."
    ))
}

fn normalize_mount_path(path: &Path) -> String {
    #[cfg(windows)]
    {
        path.to_string_lossy().replace('\\', "/")
    }
    #[cfg(not(windows))]
    {
        path.to_string_lossy().into_owned()
    }
}

fn is_cargo_cmd(cmd: &str) -> bool {
    cmd.trim_start().starts_with("cargo ")
}

/// Pull `tag` if it exists in the registry (cache hit), otherwise build
/// from the given Dockerfile content and push it.
pub fn ensure_image(runtime: &str, tag: &str, dockerfile: &str) -> Result<()> {
    let pull_ok = Command::new(runtime)
        .args(["pull", tag])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if pull_ok {
        return Ok(());
    }

    build_and_push(runtime, tag, dockerfile)
}

/// Always rebuild the composed image from the given Dockerfile content and push.
/// Used by `fork bake`.
pub fn bake_image(runtime: &str, tag: &str, dockerfile: &str) -> Result<()> {
    build_and_push(runtime, tag, dockerfile)
}

/// Build an image from `dockerfile` and tag it locally. Does not push.
pub fn build_local_image(runtime: &str, tag: &str, dockerfile: &str) -> Result<()> {
    build_image(runtime, tag, dockerfile)
}

fn build_and_push(runtime: &str, tag: &str, dockerfile: &str) -> Result<()> {
    build_image(runtime, tag, dockerfile)?;

    let push_status = Command::new(runtime)
        .args(["push", tag])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| anyhow!("Failed to push {}: {}", tag, e))?;

    if !push_status.success() {
        return Err(anyhow!(
            "Push failed (exit {})",
            push_status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}

fn build_image(runtime: &str, tag: &str, dockerfile: &str) -> Result<()> {
    let mut child = Command::new(runtime)
        .args(["build", "--tag", tag, "-f", "-", "."])
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| anyhow!("Failed to start {}: {}", runtime, e))?;

    child
        .stdin
        .take()
        .unwrap()
        .write_all(dockerfile.as_bytes())
        .map_err(|e| anyhow!("Failed to write Dockerfile: {}", e))?;

    let status = child.wait()?;
    if !status.success() {
        return Err(anyhow!(
            "Build failed (exit {})",
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}

/// Build inside a container
/// stdout/err is inherited
pub fn build_project(
    project_path: &Path,
    image: &str,
    cmd: &str,
    extra_args: &[String],
    runtime: &str,
) -> Result<()> {
    let abs = std::fs::canonicalize(project_path)
        .map_err(|e| anyhow!("Cannot resolve project path: {}", e))?;

    let full_cmd = if extra_args.is_empty() {
        cmd.to_owned()
    } else {
        format!("{} {}", cmd, extra_args.join(" "))
    };

    build_in_container(&abs, image, &full_cmd, runtime)
}

fn build_in_container(abs: &Path, image: &str, exec_cmd: &str, runtime: &str) -> Result<()> {
    let bind = format!("{}:/project", normalize_mount_path(abs));

    let mut child = Command::new(runtime);
    child.args(["run", "--rm"]);

    #[cfg(unix)]
    {
        let uid = std::process::Command::new("id")
            .args(["-u"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "1000".into());

        let gid = std::process::Command::new("id")
            .args(["-g"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "1000".into());

        child.args(["--user", &format!("{}:{}", uid, gid)]);
    }

    child.args(["-v", &bind, "-w", "/project", image, "sh", "-c", exec_cmd]);
    let mut child = child
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| anyhow!("Failed to start {}: {}", runtime, e))?;

    let status = child.wait()?;
    if !status.success() {
        return Err(anyhow!(
            "Build failed (exit {})",
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}
