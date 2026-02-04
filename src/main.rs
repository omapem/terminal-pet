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
    /// Simulate event (for development/testing)
    Event { name: String },
    /// Install git hook (post-commit) in the current repo
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
            renderer::render_pet(pet.mood);
        }

        Some(Commands::HookInstall) => {
            // write .git/hooks/post-commit
            let hook_path = PathBuf::from(".git/hooks/post-commit");
            let script = "#!/bin/sh\nterminal-pet event commit || true\n";
            if let Some(parent) = hook_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            match std::fs::write(&hook_path, script) {
                Ok(()) => {
                    let _ = std::fs::set_permissions(&hook_path, std::os::unix::fs::PermissionsExt::from_mode(0o755));
                    println!("Installed post-commit hook at {}", hook_path.display());
                }
                Err(e) => {
                    eprintln!("Failed to install hook: {}", e);
                }
            }
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
