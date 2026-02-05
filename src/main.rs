use clap::{Parser, Subcommand};
use terminal_pet::{Event, PetState, Mood};
mod renderer;
mod storage;
use std::path::PathBuf;
use storage::PersistedPet;

#[derive(Parser)]
#[command(name = "terminal-pet")]
#[command(about = "A tiny terminal pet (Milestone 1)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show pet status
    Status,
    /// Run pet continuously, reading persisted state and rendering
    Pet {
        /// Poll interval in seconds
        #[arg(long, default_value_t = 5)]
        poll_interval: u64,
    },
    /// Install the terminal-pet binary to a default location (or provide --dest)
    Install {
        /// Destination path for the installed binary
        #[arg(long)]
        dest: Option<std::path::PathBuf>,
    },
    /// Simulate event (for development/testing)
    Event { name: String },
    /// Install git hooks (post-commit) in the current repo (writes Unix and Windows variants)
    HookInstall,
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Status) => {
            let pet = PetState::new();
            println!("Pet status:\nMood: {:?}\nEnergy: {}\nXP: {}\nLevel: {}", pet.mood, pet.energy, pet.xp, pet.level);
            // try to load persisted state
            let persisted = storage::load(None);
            if let Some(ps) = persisted {
                println!("Persisted: mood={:?} energy={} xp={} level={}", ps.mood, ps.energy, ps.xp, ps.level);
            }
            renderer::render_pet_once(pet.mood);
        }

        Some(Commands::Pet { poll_interval }) => {
            println!("Starting pet mode (press Ctrl+C to quit). Reading state and rendering...");
            // set up signal handler and shared running flag
            let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
            let r = running.clone();
            let poll_ms = poll_interval.saturating_mul(1000);
            let _ = ctrlc::set_handler(move || {
                // clear the running flag to stop the loop
                r.store(false, std::sync::atomic::Ordering::SeqCst);
            });

            renderer::run_loop(poll_ms, running);
        }

        Some(Commands::Install { dest }) => {
            // determine destination: user-provided or default
            let dest_path = if let Some(d) = dest {
                d.clone()
            } else {
                let home = dirs::home_dir().expect("failed to find home dir");
                if cfg!(windows) {
                    home.join("AppData").join("Local").join("Programs").join("terminal-pet").join("terminal-pet.exe")
                } else {
                    home.join(".local").join("bin").join("terminal-pet")
                }
            };

            // ensure parent dir exists
            if let Some(parent) = dest_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }

            // try to locate current executable to copy
            match std::env::current_exe() {
                Ok(current) => {
                    match std::fs::copy(&current, &dest_path) {
                        Ok(_) => {
                            #[cfg(unix)]
                            let _ = std::fs::set_permissions(&dest_path, std::os::unix::fs::PermissionsExt::from_mode(0o755));
                            println!("Installed terminal-pet to {}", dest_path.display());
                        }
                        Err(e) => eprintln!("Failed to install binary: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to locate current executable: {}", e),
            }
        }

        Some(Commands::HookInstall) => {
            // write .git/hooks/post-commit (Unix) and post-commit.bat (Windows)
            let repo_hook_dir = PathBuf::from(".git/hooks");
            let _ = std::fs::create_dir_all(&repo_hook_dir);

            // prefer an installed absolute path if present
            let installed = {
                let home = dirs::home_dir();
                home.map(|h| {
                    if cfg!(windows) {
                        h.join("AppData").join("Local").join("Programs").join("terminal-pet").join("terminal-pet.exe")
                    } else {
                        h.join(".local").join("bin").join("terminal-pet")
                    }
                })
            };

            let unix_cmd = if let Some(ref p) = installed {
                if p.exists() { format!("{} event commit || true\n", p.display()) } else { "terminal-pet event commit || true\n".to_string() }
            } else { "terminal-pet event commit || true\n".to_string() };

            let unix_script = format!("#!/bin/sh\n{}", unix_cmd);
            let hook_path = repo_hook_dir.join("post-commit");
            match std::fs::write(&hook_path, unix_script) {
                Ok(()) => {
                    #[cfg(unix)]
                    let _ = std::fs::set_permissions(&hook_path, std::os::unix::fs::PermissionsExt::from_mode(0o755));
                    println!("Installed post-commit hook at {}", hook_path.display());
                }
                Err(e) => {
                    eprintln!("Failed to install hook: {}", e);
                }
            }

            // Windows variant
            let windows_cmd = if let Some(ref p) = installed {
                if p.exists() { format!("\"{}\" event commit || exit /b 0\r\n", p.display()) } else { "terminal-pet.exe event commit || exit /b 0\r\n".to_string() }
            } else { "terminal-pet.exe event commit || exit /b 0\r\n".to_string() };
            let windows_script = format!("@echo off\r\n{}", windows_cmd);
            let win_hook_path = repo_hook_dir.join("post-commit.bat");
            let _ = std::fs::write(&win_hook_path, windows_script);
            println!("Installed Windows post-commit hook at {}", win_hook_path.display());
        }
        Some(Commands::Event { name }) => {
            // load persisted state if present, otherwise start fresh
            let mut pet_state = if let Some(ps) = storage::load(None) {
                PetState { mood: ps.mood, energy: ps.energy, xp: ps.xp, level: ps.level }
            } else {
                PetState::new()
            };

            let event = match name.as_str() {
                "commit" => Event::Commit,
                "test-pass" => Event::TestPass,
                "test-fail" => Event::TestFail,
                "merge-conflict" => Event::MergeConflict,
                "inactivity" => Event::Inactivity,
                "npm-install" => Event::NpmInstall,
                "force-push-main" => Event::ForcePushMain,
                "friday-deploy" => Event::FridayDeploy,
                "bug-fix" => Event::BugFix,
                _ => {
                    eprintln!("Unknown event: {}", name);
                    std::process::exit(2);
                }
            };

            pet_state.apply_event(event);

            // persist new state
            let persisted = PersistedPet { mood: pet_state.mood, energy: pet_state.energy, xp: pet_state.xp, level: pet_state.level };
            if let Err(e) = storage::save(None, &persisted) {
                eprintln!("Failed to persist pet state: {}", e);
            }

            println!("Applied event: {}\nNew state: {:?}", name, pet_state);
        }
        None => {
            // default to status
            let pet = PetState::new();
            println!("Pet status:\nMood: {:?}\nEnergy: {}\nXP: {}\nLevel: {}", pet.mood, pet.energy, pet.xp, pet.level);
        }
    }
}
