use crate::task::Task;
use std::fmt::Display;

#[derive(Default, PartialEq, Debug)]
pub struct TaskList {
    todo: Vec<Task>,
    completed: Vec<Task>,
}

impl Display for TaskList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for task in &self.todo {
            writeln!(f, "{}", task)?;
        }

        writeln!(f, "----------------------------------")?;

        for task in &self.completed {
            writeln!(f, "{}", task)?;
        }

        Ok(())
    }
}

impl TaskList {
    fn add_task_todo(&mut self, task: Task) {
        if self.todo.len() < task.id() {
            self.todo.push(task);
        } else {
            self.todo.insert(task.id() - 1, task);
        }
    }

    fn remove_task_from_list(list: &mut Vec<Task>, task_id: usize) -> Option<Task> {
        let index = list.iter().position(|task| task.id() == task_id)?;

        Some(list.remove(index))
    }

    pub fn create_task(&mut self, description: &str) -> usize {
        if description.is_empty() {
            return 0;
        }

        let task = Task::new(description);
        let task_id = task.id();

        self.add_task_todo(task);

        task_id
    }

    pub fn complete_task(&mut self, task_id: usize) {
        if let Some(task) = TaskList::remove_task_from_list(&mut self.todo, task_id) {
            self.completed.push(task);
        }
    }

    pub fn undo_task(&mut self, task_id: usize) {
        if let Some(task) = TaskList::remove_task_from_list(&mut self.completed, task_id) {
            self.add_task_todo(task);
        }
    }

    pub fn delete_task(&mut self, task_id: usize) {
        if TaskList::remove_task_from_list(&mut self.todo, task_id).is_some() {
            return;
        }
        TaskList::remove_task_from_list(&mut self.completed, task_id);
    }
}
