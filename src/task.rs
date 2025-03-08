use std::fmt::Display;
use std::sync::atomic::{AtomicUsize, Ordering};

pub static COUNTER: AtomicUsize = AtomicUsize::new(1);
#[derive(PartialEq, Debug)]
pub struct Task {
    id: usize,
    description: String,
}

impl Task {
    pub fn new(description: &str) -> Self {
        Self {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            description: description.to_string(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}. {}", self.id, self.description)
    }
}
