use assert_cmd::Command;
use std::path::Path;

const EXAMPLES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../examples");

fn fork() -> Command {
    Command::cargo_bin("fork").unwrap()
}

fn require_e2e() -> bool {
    std::env::var("FORK_E2E").is_err()
}

#[test]
fn build_rp2040_rp2040_hal() {
    if require_e2e() { return; }
    let path = Path::new(EXAMPLES).join("rp2040-rp2040-hal");
    fork()
        .args(["build", "-c", "rp2040"])
        .arg(&path)
        .timeout(std::time::Duration::from_secs(1200))
        .assert()
        .success();
}

#[test]
fn build_stm32f405_cortex_m() {
    if require_e2e() { return; }
    let path = Path::new(EXAMPLES).join("stm32f405-cortex-m");
    fork()
        .args(["build", "-c", "stm32f405"])
        .arg(&path)
        .timeout(std::time::Duration::from_secs(1200))
        .assert()
        .success();
}
