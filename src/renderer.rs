use std::{thread, time::Duration};
use crate::{Mood};

pub fn render_pet(mood: Mood) {
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
