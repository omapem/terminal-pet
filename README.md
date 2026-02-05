terminal-pet

A terminal-based virtual pet (MVP). This repository is being bootstrapped with a Rust library implementing the pet engine for Milestone 1.

Usage
- Show status: cargo run -- status
- Run pet mode (continuous): cargo run -- pet --poll-interval 5

Notes
- Pet mode listens for SIGINT/SIGTERM and will play a short exit animation before exiting.
- The default poll interval is 5 seconds; customize with --poll-interval <seconds>.
- CI excludes signal-based integration tests on non-Unix runners to avoid flakiness.

Installation
- To install the running binary to a user location: cargo run -- install
- To specify a destination: cargo run -- install --dest /path/to/terminal-pet
- Installer will copy the currently running executable to ~/.local/bin/terminal-pet on Unix or %LocalAppData%\\Programs\\terminal-pet\\terminal-pet.exe on Windows.

Hooks
- Use cargo run -- hook-install in a repository to write post-commit hooks. If the installer has been used, hooks will call the installed binary by absolute path; otherwise they fall back to invoking 'terminal-pet' or 'terminal-pet.exe' on PATH.

