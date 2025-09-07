//! Handles persistence of application data.

use crate::tasks::{Task, TaskList};
use std::fs;
use std::io;
use std::path::PathBuf;

/// Returns the path to the tasks file.
fn get_tasks_file() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".tasks")
}

/// Returns the path to the undone indexes file.
fn get_undone_file() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".tasks_undone")
}

/// Loads tasks from the tasks file.
pub fn load_tasks() -> io::Result<TaskList> {
    let path = get_tasks_file();
    if !path.exists() {
        return Ok(TaskList {
            tasks: vec![],
            current_index: 0,
        });
    }
    let content = fs::read_to_string(path)?;
    let mut tasks_list: TaskList =
        toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    if tasks_list.current_index >= tasks_list.tasks.len() {
        tasks_list.current_index = tasks_list.tasks.iter().position(|t| !t.done).unwrap_or(0);
    }
    Ok(tasks_list)
}

/// Persists tasks to the tasks file.
pub fn persist_tasks(tasks: &[Task], current_index: usize) -> io::Result<()> {
    let tasklist = TaskList {
        tasks: tasks.to_vec(),
        current_index,
    };
    let toml = toml::to_string_pretty(&tasklist).map_err(io::Error::other)?;
    fs::write(get_tasks_file(), toml)
}

/// Loads undone indexes from the undone indexes file.
pub fn load_undone_indexes(tasks: &[Task]) -> io::Result<Vec<usize>> {
    let path = get_undone_file();
    if path.exists() {
        let content = fs::read_to_string(path)?;
        let mut indexes: Vec<usize> = content
            .lines()
            .filter_map(|line| line.parse::<usize>().ok())
            .collect();

        indexes.retain(|&i| i < tasks.len() && !tasks[i].done);
        if indexes.is_empty() {
            indexes = tasks
                .iter()
                .enumerate()
                .filter_map(|(i, t)| if !t.done { Some(i) } else { None })
                .collect();
        }
        Ok(indexes)
    } else {
        Ok(tasks
            .iter()
            .enumerate()
            .filter_map(|(i, t)| if !t.done { Some(i) } else { None })
            .collect())
    }
}

/// Persists undone indexes to the undone indexes file.
pub fn persist_undone_indexes(indexes: &[usize]) -> io::Result<()> {
    let content = indexes
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(
            "
",
        );
    fs::write(get_undone_file(), content)
}
