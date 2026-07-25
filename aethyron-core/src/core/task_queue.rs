use std::collections::VecDeque;

use crate::agents::Task;

pub struct TaskQueue {
    tasks: VecDeque<Task>,
}

impl TaskQueue {

    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }


    pub fn add(&mut self, task: Task) {
        self.tasks.push_back(task);
    }


    pub fn next(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }


    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}