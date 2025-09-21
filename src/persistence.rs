//! Handles persistence of application data.

use crate::tasks::{Task, TaskList};
use std::fs;
use std::io;
use std::path::PathBuf;

/// Returns the path to the tasks directory.
fn get_tasks_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".tasks")
        .join("tasks")
}

/// Returns the path to the tasks file.
fn get_tasks_file() -> PathBuf {
    get_tasks_dir().join("tasks.toml")
}

/// Returns the path to the undone indexes file.
fn get_undone_file() -> PathBuf {
    get_tasks_dir().join("tasks_undone.toml")
}

/// Loads tasks from the tasks file.
pub fn load_tasks() -> io::Result<TaskList> {
    let dir = get_tasks_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    let path = get_tasks_file();
    if !path.exists() {
        return create_sample_tasks_file();
    }
    let content = fs::read_to_string(path)?;
    let mut tasks_list: TaskList =
        toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    if tasks_list.current_index >= tasks_list.tasks.len() {
        tasks_list.current_index = tasks_list.tasks.iter().position(|t| !t.done).unwrap_or(0);
    }
    Ok(tasks_list)
}

/// Creates a sample tasks file.
fn create_sample_tasks_file() -> io::Result<TaskList> {
    let sample_tasks = TaskList {
        tasks: vec![
            Task {
                id: 1,
                title: "Make this task Done!".to_string(),
                description: "- Press [d] to make this task Done".to_string(),
                done: false,
            },
            Task {
                id: 2,
                title: "Add your desired tasks".to_string(),
                description:
                    "Open the $HOME/.tasks/tasks file and add as many sequential tasks you want."
                        .to_string(),
                done: false,
            },
            Task {
                id: 3,
                title: "Follow your dream!".to_string(),
                description: "Don't think what I have to do today! just open Taskling and follow your plan.\n\nSee your progress visually."
                    .to_string(),
                done: false,
            },
        ],
        current_index: 0,
        the_goal: "1 Step at a time!".to_string(),
    };
    let toml = toml::to_string_pretty(&sample_tasks).map_err(io::Error::other)?;
    fs::write(get_tasks_file(), toml)?;
    Ok(sample_tasks)
}

/// Persists tasks to the tasks file.
pub fn persist_tasks(task_list: &TaskList) -> io::Result<()> {
    let toml = toml::to_string_pretty(task_list).map_err(io::Error::other)?;
    fs::write(get_tasks_file(), toml)
}

/// Loads undone indexes from the undone indexes file.
pub fn load_undone_indexes(tasks: &[Task]) -> io::Result<Vec<usize>> {
    let path = get_undone_file();
    if !path.exists() {
        fs::write(&path, "")?;
    }
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
