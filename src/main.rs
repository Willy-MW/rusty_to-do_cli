use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::fmt::Display;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

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

fn create_task(description: &str) -> Option<Task> {
    if description.is_empty() {
        None
    } else {
        Some(Task::new(description))
    }
}

fn complete_task(number: isize) {
    println!("Completed task #{}", number);
}

fn undo_task(number: isize) {
    println!("Undo task #{}", number);
}

fn delete_task(number: isize) {
    println!("Deleted task #{}", number);
}

#[derive(PartialEq, Debug)]
enum Action {
    ProcessInput,
    Exit,
}

#[derive(PartialEq, Debug)]
enum Command {
    Complete,
    Undo,
    Delete,
}
#[derive(PartialEq, Debug)]
struct Task {
    id: usize,
    description: String,
}

impl Task {
    fn new(description: &str) -> Self {
        Self {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            description: description.to_string(),
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}. {}", self.id, self.description)
    }
}
