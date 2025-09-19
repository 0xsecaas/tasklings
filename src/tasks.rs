//! Task management.

use crate::persistence;
use serde::{Deserialize, Serialize};
use std::io;

/// A single task.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

/// A list of tasks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    #[serde(default)]
    pub current_index: usize,
    #[serde(default = "default_goal")]
    pub the_goal: String,
}

fn default_goal() -> String {
    "Tasklings".to_string()
}

/// Manages task state.
#[derive(Clone)]
pub struct TaskManager {
    pub tasks: Vec<Task>,
    pub current_index: usize,
    pub undone_indexes: Vec<usize>,
    pub undone_pos: usize,
    pub the_goal: String,
}

impl TaskManager {
    /// Creates a new `TaskManager`.
    pub fn new(task_list: TaskList) -> io::Result<Self> {
        let undone_indexes = persistence::load_undone_indexes(&task_list.tasks)?;
        let undone_pos = task_list.tasks.iter().position(|t| !t.done).unwrap_or(0);
        let current_index = undone_indexes.get(undone_pos).copied().unwrap_or(0);
        Ok(Self {
            tasks: task_list.tasks,
            current_index,
            undone_indexes,
            undone_pos,
            the_goal: task_list.the_goal,
        })
    }

    /// Returns the current task.
    pub fn current_task(&self) -> &Task {
        &self.tasks[self.current_index]
    }

    /// Returns a mutable reference to the current task.
    pub fn current_task_mut(&mut self) -> &mut Task {
        &mut self.tasks[self.current_index]
    }

    /// Moves to the next undone task.
    pub fn next_undone(&mut self) {
        if self.is_done() {
            return;
        }

        if let Some((pos, &idx)) = self
            .undone_indexes
            .iter()
            .enumerate()
            .find(|&(_, i)| i > &self.current_index)
        {
            self.undone_pos = pos;
            self.current_index = idx;
        }
    }

    /// Moves to the first undone task.
    pub fn first_undone(&mut self) {
        if self.is_done() {
            return;
        }
        self.undone_pos = 0;
        self.current_index = self.undone_indexes[0];
    }

    /// Moves to the last task.
    pub fn last(&mut self) {
        self.current_index = self.tasks.len() - 1;
    }

    /// Moves to the next task.
    pub fn next(&mut self) {
        if self.current_index + 1 < self.tasks.len() {
            self.current_index += 1;
        }
    }

    /// Moves to the previous task.
    pub fn previous(&mut self) {
        if self.current_index > 0 {
            self.current_index = self.current_index.saturating_sub(1);
        }
    }

    /// Marks the current task as done.
    pub fn mark_done(&mut self) {
        self.current_task_mut().done = true;
        self.undone_indexes.remove(self.undone_pos);

        if self.undone_pos >= self.undone_indexes.len() && !self.undone_indexes.is_empty() {
            self.undone_pos = self.undone_indexes.len() - 1;
        }
        self.persist();
    }

    /// Marks the current task as not done.
    pub fn mark_undone(&mut self) {
        let index = self.current_index;
        self.tasks[index].done = false;

        match self.undone_indexes.binary_search(&index) {
            Ok(_) => {} // already exists
            Err(pos) => self.undone_indexes.insert(pos, index),
        }
        self.undone_pos = self
            .undone_indexes
            .iter()
            .position(|&i| i == index)
            .unwrap();

        self.persist();
    }

    /// Returns `true` if all tasks are done.
    pub fn is_done(&self) -> bool {
        self.undone_indexes.is_empty()
    }

    /// Persists the task state to disk.
    fn persist(&self) {
        let current_index = self
            .undone_indexes
            .get(self.undone_pos)
            .copied()
            .unwrap_or(0);
        let task_list = TaskList {
            tasks: self.tasks.clone(),
            current_index,
            the_goal: self.the_goal.clone(),
        };
        if let Err(e) = persistence::persist_tasks(&task_list) {
            eprintln!("Failed to persist tasks: {}", e);
        }
        if let Err(e) = persistence::persist_undone_indexes(&self.undone_indexes) {
            eprintln!("Failed to persist undone indexes: {}", e);
        }
    }
}
