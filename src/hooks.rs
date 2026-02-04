use std::fs;
use std::path::PathBuf;

pub fn install_post_commit(hook_path: &PathBuf) -> std::io::Result<()> {
    if let Some(parent) = hook_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let script = "#!/bin/sh\nterminal-pet event commit || true\n";
    fs::write(hook_path, script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(hook_path, PermissionsExt::from_mode(0o755))?;
    }
    Ok(())
}
