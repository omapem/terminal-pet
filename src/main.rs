use clap::{Parser, Subcommand};
use terminal_pet::{Event, PetState};

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
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Status) => {
            let pet = PetState::new();
            println!("Pet status:\nMood: {:?}\nEnergy: {}\nXP: {}\nLevel: {}", pet.mood, pet.energy, pet.xp, pet.level);
        }
        Some(Commands::Event { name }) => {
            let mut pet = PetState::new();
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
            pet.apply_event(event);
            println!("Applied event: {}\nNew state: {:?}", name, pet);
        }
        None => {
            // default to status
            let pet = PetState::new();
            println!("Pet status:\nMood: {:?}\nEnergy: {}\nXP: {}\nLevel: {}", pet.mood, pet.energy, pet.xp, pet.level);
        }
    }
}
