//! Application state and logic.

use crate::persistence;
use crate::tasks::TaskManager;
use std::io;

/// Main application struct.
pub struct App {
    /// Task manager.
    pub task_manager: TaskManager,
    /// If true, the application should quit.
    pub should_quit: bool,
}

impl App {
    /// Creates a new `App`.
    pub fn new() -> io::Result<App> {
        let tasks_list = persistence::load_tasks()?;
        let task_manager = TaskManager::new(tasks_list.tasks)?;
        Ok(App {
            task_manager,
            should_quit: false,
        })
    }

    /// Signals the application to quit.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Marks the current task as done.
    pub fn mark_done(&mut self) {
        self.task_manager.mark_done();
    }

    /// Marks the current task as not done.
    pub fn mark_undone(&mut self) {
        self.task_manager.mark_undone();
    }

    /// Moves to the next task.
    pub fn next_task(&mut self) {
        self.task_manager.next();
    }

    /// Moves to the previous task.
    pub fn previous_task(&mut self) {
        self.task_manager.previous();
    }

    /// Moves to the next undone task.
    pub fn next_undone_task(&mut self) {
        self.task_manager.next_undone();
    }

    /// Moves to the first undone task.
    pub fn first_undone_task(&mut self) {
        self.task_manager.first_undone();
    }

    /// Moves to the last task.
    pub fn last_task(&mut self) {
        self.task_manager.last();
    }

}
