use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::fmt::Display;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn main() -> Result<()> {
    let mut input = String::new();
    let mut task_list = TaskList::new();

    'main: loop {
        match handle_user_input(&mut input)? {
            Some(Action::ProcessInput) => {
                process_input(&input, &mut task_list)?;
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

fn process_input(input: &str, task_list: &mut TaskList) -> Result<()> {
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

impl TryFrom<&str> for Command {
    type Error = &'static str;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "/complete" => Ok(Command::Complete),
            "/undo" => Ok(Command::Undo),
            "/delete" => Ok(Command::Delete),
            _ => Err("Unknown command"),
        }
    }
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

#[derive(PartialEq, Debug)]
struct TaskList {
    todo: Vec<Task>,
    completed: Vec<Task>,
}

impl TaskList {
    fn new() -> Self {
        Self {
            todo: Vec::new(),
            completed: Vec::new(),
        }
    }

    fn create_task(&mut self, description: &str) -> usize {
        if description.is_empty() {
            return 0;
        }

        let task = Task::new(description);
        let task_id = task.id;

        self.add_task(task);

        task_id
    }

    fn add_task(&mut self, task: Task) {
        if self.todo.len() < task.id {
            self.todo.push(task);
        } else {
            self.todo.insert(task.id - 1, task);
        }
    }

    fn complete_task(&mut self, task_id: usize) {
        if let Some(task) = TaskList::remove_task(&mut self.todo, task_id) {
            self.completed.push(task);
        }
    }

    fn undo_task(&mut self, task_id: usize) {
        if let Some(task) = TaskList::remove_task(&mut self.completed, task_id) {
            self.add_task(task);
        }
    }

    fn delete_task(&mut self, task_id: usize) {
        if TaskList::remove_task(&mut self.todo, task_id).is_some() {
            return;
        }
        TaskList::remove_task(&mut self.completed, task_id);
    }

    fn remove_task(tasks: &mut Vec<Task>, task_id: usize) -> Option<Task> {
        let index = tasks.iter().position(|task| task.id == task_id)?;

        Some(tasks.remove(index))
    }
}
