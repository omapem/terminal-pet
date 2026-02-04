use std::{thread, time::Duration, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use crate::{Mood};

pub fn render_pet_once(mood: Mood) {
    let frames = match mood {
        Mood::Happy => vec![
            "\n ∧＿∧\n ( ◕‿◕)    ♥\n /つ🍪⊂\\\n しーーーJ\n",
            "\n ∧＿∧\n ( ◕‿◕)    ♥\n /つ  ⊂\\\n しーーーJ\n",
        ],
        Mood::Sad => vec![
            "\n ∧＿∧\n ( ；‿；)    ☁\n /つ   ⊂\\\n しーーーJ\n",
        ],
        _ => vec![
            "\n ∧＿∧\n ( ◕‿◕)\n /つ   ⊂\\\n しーーーJ\n",
        ],
    };

    for frame in frames.iter().cycle().take(4) {
        // clear screen simple
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", frame);
        thread::sleep(Duration::from_millis(350));
    }
}

pub fn run_loop(poll_interval_ms: u64, running: Arc<AtomicBool>) {
    // run until the running flag is cleared by a signal handler
    while running.load(Ordering::SeqCst) {
        // load persisted state
        let path = crate::storage::default_path();
        if let Some(state) = crate::storage::load(Some(&path)) {
            render_pet_once(state.mood);
        } else {
            render_pet_once(Mood::Neutral);
        }
        // check again periodically
        for _ in 0..(poll_interval_ms / 200).max(1) {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }

    // play a short exit animation
    let exit_frames = [
        "\n ∧＿∧\n ( ◡‿◡)    Bye\n /つ   ⊂\\\n しーーーJ\n",
        "\n  ∧＿∧\n ( ；_；)    Bye\n /つ   ⊂\\\n しーーーJ\n",
    ];
    for frame in exit_frames.iter().cycle().take(4) {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", frame);
        thread::sleep(Duration::from_millis(250));
    }
}
