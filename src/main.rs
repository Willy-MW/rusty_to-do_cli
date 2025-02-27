use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

fn main() -> Result<()> {
    'main: loop {
        if !handle_user_input()? {
            break 'main;
        }
    }

    Ok(())
}

// Function to handle user input
fn handle_user_input() -> Result<bool> {
    let mut result = Ok(true);

    if event::poll(Duration::default())? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                match key.code {
                    KeyCode::Esc => result = Ok(false),   // Exit the loop
                    KeyCode::Char(c) => eprint!("{}", c), // Print input
                    _ => {}
                };
            }
        }
    }

    result
}
