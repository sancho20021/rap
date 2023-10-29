use std::time::Instant;

use crossterm::event::{Event, KeyCode};

pub fn record_beat() -> Vec<Instant> {
    crossterm::terminal::enable_raw_mode().unwrap();
    let mut timestamps = Vec::new();
    loop {
        if let Ok(event) = crossterm::event::read() {
            if let Event::Key(key_event) = event {
                if key_event.code != KeyCode::Enter {
                    timestamps.push(Instant::now());
                } else {
                    break;
                }
            }
        }
    }
    crossterm::terminal::disable_raw_mode().unwrap();
    timestamps
}