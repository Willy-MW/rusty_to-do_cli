use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

fn main() -> Result<()> {
    let mut input = String::new();

    'main: loop {
        match handle_user_input(&mut input)? {
            Some(Action::ProcessInput) => {
                process_input(&input.trim())?;
                input.clear();
            }
            Some(Action::Exit) => break 'main,
            None => {}
        }
    }

    Ok(())
}

// Function to handle user input
fn handle_user_input(input: &mut String) -> Result<Option<Action>> {
    let mut action: Option<Action> = None;

    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Esc => action = Some(Action::Exit),
                KeyCode::Char(c) => {
                    eprint!("{}", c);
                    (*input).push(c);
                }
                KeyCode::Backspace => {
                    eprint!("\x1b[D \x1b[D");
                    (*input).pop();
                }
                KeyCode::Enter => {
                    println!();
                    action = Some(Action::ProcessInput)
                }
                _ => {}
            }
        }
    }

    Ok(action)
}

fn process_input(input: &str) -> Result<()> {
    let command = get_command(input);
    let argument = get_argument(input, command.is_some());

    Ok(())
}

fn get_command(input: &str) -> Option<Command> {
    let mut input_iter = input.split_whitespace();

    let Some(command) = input_iter.next() else {
        return None;
    };

    match command.to_lowercase().as_str() {
        "/complete" => Some(Command::Complete),
        "/undo" => Some(Command::Undo),
        "/delete" => Some(Command::Delete),
        _ => None,
    }
}

fn get_argument(input: &str, is_command: bool) -> &str {
    if is_command {
        input.split_whitespace().nth(1).unwrap_or("")
    } else {
        input
    }
}

fn complete_task(number: isize) {
    println!("Completed task #{}", number);
}

fn delete_task(number: isize) {
    println!("Deleted task #{}", number);
}

enum Action {
    ProcessInput,
    Exit,
}

enum Command {
    Complete,
    Undo,
    Delete,
}
