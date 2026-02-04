use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;
use std::fs;

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
