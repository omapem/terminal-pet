use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::{Command, Stdio};
use tempfile::tempdir;
use std::fs;
use std::time::Duration;

#[test]
fn installs_hook_in_repo_and_triggers_event() {
    // create temp git repo
    let dir = tempdir().unwrap();
    let repo_dir = dir.path();
    Command::new("git").arg("init").current_dir(&repo_dir).output().unwrap();

    // run our binary's hook-install
    let mut cmd = Command::cargo_bin("terminal_pet").unwrap();
    cmd.arg("hook-install").current_dir(&repo_dir);
    cmd.assert().success();

    // check hook exists
    let hook_path = repo_dir.join(".git/hooks/post-commit");
    assert!(hook_path.exists());

    // simulate a commit by running the hook script directly
    let output = Command::new(hook_path).current_dir(&repo_dir).output().unwrap();
    // script exits 0
    assert!(output.status.success());
}

#[test]
#[cfg(unix)]
fn pet_exits_on_sigint() {
    // spawn the binary in pet mode
    let mut child = Command::cargo_bin("terminal_pet").unwrap()
        .arg("pet")
        .arg("--poll-interval")
        .arg("1")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn pet binary");

    // give it a moment to start
    std::thread::sleep(Duration::from_secs(1));

    // send SIGINT
    unsafe { libc::kill(child.id() as i32, libc::SIGINT); }

    // wait for process to exit with timeout
    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) => {
                if start.elapsed() > Duration::from_secs(5) {
                    let _ = child.kill();
                    panic!("pet did not exit after SIGINT");
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => panic!("error waiting for child: {}", e),
        }
    }
}
