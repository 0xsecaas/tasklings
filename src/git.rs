//! Git integration.

use std::io;
use std::process::Command;
use std::path::PathBuf;

/// Returns the path to the tasks directory.
fn get_tasks_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".tasks")
        .join("tasks")
}

/// Initializes a git repository in the tasks directory if it doesn't exist.
pub fn init() -> io::Result<()> {
    let dir = get_tasks_dir();
    let git_dir = dir.join(".git");
    if !git_dir.exists() {
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .arg("init")
            .status()?;
    }
    Ok(())
}

/// Adds a remote to the git repository.
pub fn add_remote(url: &str) -> io::Result<()> {
    let dir = get_tasks_dir();
    Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(url)
        .status()?;
    Ok(())
}

/// Checks if a remote is configured.
pub fn has_remote() -> io::Result<bool> {
    let dir = get_tasks_dir();
    let output = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("remote")
        .output()?;
    Ok(!output.stdout.is_empty())
}

/// Pushes changes to the remote repository.
pub fn push() -> io::Result<()> {
    let dir = get_tasks_dir();
    Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("add")
        .arg("tasks.toml")
        .arg("tasks_undone.toml")
        .status()?;
    Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("commit")
        .arg("-m")
        .arg("Update tasks")
        .status()?;
    Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("push")
        .arg("origin")
        .arg("main")
        .status()?;
    Ok(())
}

/// Pulls changes from the remote repository.
pub fn pull() -> io::Result<()> {
    let dir = get_tasks_dir();
    Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("pull")
        .arg("origin")
        .arg("main")
        .status()?;
    Ok(())
}
