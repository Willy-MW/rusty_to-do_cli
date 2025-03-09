use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum Command {
    Complete,
    Undo,
    Delete,
    Help,
}

impl TryFrom<&str> for Command {
    type Error = &'static str;
    fn try_from(value: &str) -> anyhow::Result<Self, Self::Error> {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> anyhow::Result<(), std::fmt::Error> {
        let as_str: &str = self.clone().into();
        write!(f, "{}", as_str)
    }
}

impl Command {
    pub fn get_all() -> Vec<Command> {
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
