terminal-pet

A terminal-based virtual pet (MVP). This repository is being bootstrapped with a Rust library implementing the pet engine for Milestone 1.

Usage
- Show status: cargo run -- status
- Run pet mode (continuous): cargo run -- pet --poll-interval 5

Notes
- Pet mode listens for SIGINT/SIGTERM and will play a short exit animation before exiting.
- The default poll interval is 5 seconds; customize with --poll-interval <seconds>.
- CI excludes signal-based integration tests on non-Unix runners to avoid flakiness.
