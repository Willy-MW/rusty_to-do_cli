use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

fn main() -> Result<()> {
    let mut command = String::new();

    'main: loop {
        match handle_user_input(&mut command)? {
            Some(Action::ProcessCommand) => {
                process_command(&command)?;
                command.clear();
            }
            Some(Action::Exit) => break 'main,
            None => {}
        }
    }

    Ok(())
}

// Function to handle user input
fn handle_user_input(command: &mut String) -> Result<Option<Action>> {
    let mut action: Option<Action> = None;

    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Release {
            match key.code {
                KeyCode::Esc => action = Some(Action::Exit),
                KeyCode::Char(c) => {
                    eprint!("{}", c);
                    (*command).push(c);
                }
                KeyCode::Backspace => {
                    eprint!("\x1b[D \x1b[D");
                    (*command).pop();
                }
                KeyCode::Enter => action = Some(Action::ProcessCommand),
                _ => {}
            }
        }
    }

    Ok(action)
}

fn process_command(command: &str) -> Result<()> {
    println!("Processing command: {}", command);

    let command  = command.split(' ').next();

    if command.is_none() {
        return Ok(());
    }

    match command {
        So
        "complete" => println!("Complete"),
        "delete" => println!("Delete"),
        _ => println!("Unknown command: {}", command),
    }

    Ok(())
}

enum Action {
    ProcessCommand,
    Exit,
}
