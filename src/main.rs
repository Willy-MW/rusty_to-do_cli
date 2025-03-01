use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

fn main() -> Result<()> {
    let mut command = String::new();
    let mut action: Option<Action> = None;

    'main: loop {
        handle_user_input(&mut command, &mut action)?;

        match action {
            Some(Action::ProcessCommand) => {
                println!();
                command.clear();
            }
            Some(Action::Exit) => break 'main,
            None => {}
        }

        action = None;
    }

    Ok(())
}

// Function to handle user input
fn handle_user_input(command: &mut String, action: &mut Option<Action>) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Release {
            match key.code {
                KeyCode::Esc => *action = Some(Action::Exit),
                KeyCode::Char(c) => {
                    eprint!("{}", c);
                    (*command).push(c);
                }
                KeyCode::Backspace => {
                    eprint!("\x1b[D \x1b[D");
                    (*command).pop();
                }
                KeyCode::Enter => *action = Some(Action::ProcessCommand),
                _ => {}
            }
        }
    }

    Ok(())
}

enum Action {
    ProcessCommand,
    Exit,
}
