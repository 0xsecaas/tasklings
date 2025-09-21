//! Application state and logic.

use crate::git;
use crate::persistence;
use crate::tasks::{Task, TaskManager};
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use tempfile::NamedTempFile;

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
        let mut task_manager = TaskManager::new(tasks_list)?;
        task_manager.first_undone();
        Ok(App {
            task_manager,
            should_quit: false,
        })
    }

    /// Reloads tasks from disk.
    pub fn reload_tasks(&mut self) -> io::Result<()> {
        let tasks_list = persistence::load_tasks()?;
        self.task_manager = TaskManager::new(tasks_list)?;
        self.task_manager.first_undone();
        Ok(())
    }

    /// Signals the application to quit.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Marks the current task as done.
    pub fn mark_done(&mut self) {
        self.task_manager.mark_done();
        self.task_manager.next_undone();
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

    /// Opens the current task in an editor for modification.
    pub fn edit_task(&mut self) -> io::Result<()> {
        let task = self.task_manager.current_task().clone();
        let edited_task = self.get_task_from_editor(Some(task))?;
        if let Some(edited_task) = edited_task {
            self.task_manager.update_task(edited_task);
        }
        Ok(())
    }

    /// Creates a new task.
    pub fn new_task(&mut self) -> io::Result<()> {
        let new_task = self.get_task_from_editor(None)?;
        if let Some(new_task) = new_task {
            self.task_manager.add_task(new_task);
        }
        Ok(())
    }

    fn get_task_from_editor(&self, task: Option<Task>) -> io::Result<Option<Task>> {
        let mut file = NamedTempFile::new()?;
        let task = task.unwrap_or_else(|| {
            let last_id = self.task_manager.tasks.last().map(|t| t.id).unwrap_or(0);
            Task {
                id: last_id + 1,
                title: "New Task".to_string(),
                description: "Task description".to_string(),
                done: false,
            }
        });
        let toml = toml::to_string_pretty(&task).map_err(io::Error::other)?;
        file.write_all(toml.as_bytes())?;

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
        Command::new(editor).arg(file.path()).status()?;

        let content = fs::read_to_string(file.path())?;
        let task: Task =
            toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(Some(task))
    }

    /// Pushes tasks to the remote repository.
    pub fn git_push(&self) -> io::Result<()> {
        git::init()?;
        if !git::has_remote()? {
            let url = self.get_remote_url_from_user()?;
            git::add_remote(&url)?;
            self.save_remote_url(&url)?;
        }
        git::push()
    }

    /// Pulls tasks from the remote repository.
    pub fn git_pull(&mut self) -> io::Result<()> {
        git::init()?;
        if !git::has_remote()? {
            let url = self.get_remote_url_from_user()?;
            git::add_remote(&url)?;
            self.save_remote_url(&url)?;
        }
        git::pull()?;
        self.reload_tasks()
    }

    fn get_remote_url_from_user(&self) -> io::Result<String> {
        let mut url = String::new();
        println!("Please enter the remote repository URL:");
        io::stdin().read_line(&mut url)?;
        Ok(url.trim().to_string())
    }

    fn save_remote_url(&self, url: &str) -> io::Result<()> {
        let path = dirs::home_dir()
            .expect("Could not find home directory")
            .join(".tasklings_config");
        fs::write(path, url)
    }
}
