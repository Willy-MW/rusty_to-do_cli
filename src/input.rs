use crate::task_list::TaskList;
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{event, execute};
use std::fmt::Display;
use std::io::stdout;

#[derive(PartialEq, Debug)]
pub enum Action {
    ProcessInput,
    Exit,
}

#[derive(PartialEq, Debug, Clone)]
enum Command {
    Complete,
    Undo,
    Delete,
    Help,
}

impl TryFrom<&str> for Command {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "/help" => Ok(Command::Help),
            "/do" => Ok(Command::Complete),
            "/undo" => Ok(Command::Undo),
            "/delete" => Ok(Command::Delete),
            _ => Err("Unknown command"),
        }
    }
}

impl From<Command> for &'static str {
    fn from(command: Command) -> Self {
        match command {
            Command::Help => "/help",
            Command::Complete => "/complete",
            Command::Undo => "/undo",
            Command::Delete => "/delete",
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let as_str: &str = self.clone().into();
        write!(f, "{}", as_str)
    }
}

impl Command {
    fn get_all() -> Vec<Command> {
        Vec::from([
            Command::Help,
            Command::Complete,
            Command::Delete,
            Command::Undo,
        ])
    }
    pub fn get_description(&self) -> &str {
        match self {
            Command::Help => "Lists all commands.",
            Command::Complete => "Marks task as completed and moves it into completed tasks list.",
            Command::Undo => "Moves task back from completed tasks list into todo list.",
            Command::Delete => "Deletes the task.",
        }
    }
}

pub fn handle_user_input(input: &mut String) -> Result<Option<Action>> {
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

pub fn process_input(input: &str, task_list: &mut TaskList) -> Result<()> {
    let command = get_command(input);
    let argument = get_argument(input, command.is_some());

    if command.is_none() {
        task_list.create_task(argument);
        return Ok(());
    }

    let task_id = argument.parse().unwrap_or(0);

    match command.unwrap() {
        Command::Help => print_help(),
        Command::Complete => task_list.complete_task(task_id),
        Command::Undo => task_list.undo_task(task_id),
        Command::Delete => task_list.delete_task(task_id),
    }

    Ok(())
}

fn get_command(input: &str) -> Option<Command> {
    let command = input.split_whitespace().next()?;

    Command::try_from(command).ok()
}

fn get_argument(input: &str, is_command: bool) -> &str {
    if is_command {
        input.split_whitespace().nth(1).unwrap_or("")
    } else {
        input.trim()
    }
}

pub fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

pub fn print_header() {
    println!("Rust TODO list. Type something to add new task. Type /help for a list of available commands.");
}

pub fn print_help() {
    for command in Command::get_all() {
        print!("{}", command);
        print!(" - ");
        println!("{}", command.get_description());
    }
    println!("Press enter to continue...");
}
