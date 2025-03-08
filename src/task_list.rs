use crate::task::Task;

#[derive(Default, PartialEq, Debug)]
pub struct TaskList {
    todo: Vec<Task>,
    completed: Vec<Task>,
}

impl TaskList {
    pub fn create_task(&mut self, description: &str) -> usize {
        if description.is_empty() {
            return 0;
        }

        let task = Task::new(description);
        let task_id = task.id();

        self.add_task(task);

        task_id
    }

    pub fn add_task(&mut self, task: Task) {
        if self.todo.len() < task.id() {
            self.todo.push(task);
        } else {
            self.todo.insert(task.id() - 1, task);
        }
    }

    pub fn complete_task(&mut self, task_id: usize) {
        if let Some(task) = TaskList::remove_task(&mut self.todo, task_id) {
            self.completed.push(task);
        }
    }

    pub fn undo_task(&mut self, task_id: usize) {
        if let Some(task) = TaskList::remove_task(&mut self.completed, task_id) {
            self.add_task(task);
        }
    }

    pub fn delete_task(&mut self, task_id: usize) {
        if TaskList::remove_task(&mut self.todo, task_id).is_some() {
            return;
        }
        TaskList::remove_task(&mut self.completed, task_id);
    }

    pub fn remove_task(tasks: &mut Vec<Task>, task_id: usize) -> Option<Task> {
        let index = tasks.iter().position(|task| task.id() == task_id)?;

        Some(tasks.remove(index))
    }
}
