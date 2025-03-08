use crate::task_list::TaskList;
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{event, execute};
use std::io::stdout;

#[derive(PartialEq, Debug)]
pub enum Action {
    ProcessInput,
    Exit,
}

#[derive(PartialEq, Debug)]
enum Command {
    Complete,
    Undo,
    Delete,
}

impl TryFrom<&str> for Command {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "/complete" => Ok(Command::Complete),
            "/undo" => Ok(Command::Undo),
            "/delete" => Ok(Command::Delete),
            _ => Err("Unknown command"),
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
                KeyCode::Enter => action = Some(Action::ProcessInput),
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
