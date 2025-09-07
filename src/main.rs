use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Clone)]
struct TaskManager {
    tasks: Vec<Task>,
    current_index: usize,
    undone_indexes: Vec<usize>,
    undone_pos: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: i32,
    title: String,
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    tasks: Vec<Task>,
    #[serde(default)]
    current_index: usize,
}

#[derive(Serialize)]
struct TaskListRef<'a> {
    tasks: &'a [Task],
    current_index: usize,
}

impl TaskManager {
    fn new(tasks: Vec<Task>) -> Self {
        let undone_indexes: Vec<usize> = load_undone_indexes(&tasks);
        let undone_pos = 0;
        let current_index = 0;
        Self {
            tasks,
            current_index,
            undone_indexes,
            undone_pos,
        }
    }

    fn current_task(&self) -> &Task {
        &self.tasks[self.current_index]
    }

    fn current_task_mut(&mut self) -> &mut Task {
        &mut self.tasks[self.current_index]
    }

    fn next_undone(&mut self) {
        if !self.is_done() && self.undone_pos + 1 < self.undone_indexes.len() {
            self.undone_pos += 1;
            self.current_index = self.undone_indexes[self.undone_pos];
        }
    }

    fn next(&mut self) {
        if self.current_index + 1 < self.tasks.len() {
            self.current_index += 1;
        }
    }

    fn previous(&mut self) {
        if self.current_index > 0 {
            self.current_index = self.current_index.saturating_sub(1);
        }
    }

    fn mark_done(&mut self) {
        self.current_task_mut().done = true;
        self.undone_indexes.remove(self.undone_pos);

        // adjust undone_pos if we removed the last element
        if self.undone_pos >= self.undone_indexes.len() && !self.undone_indexes.is_empty() {
            self.undone_pos = self.undone_indexes.len() - 1;
        }
        self.persist();
        print!("✅ Marked done");
    }

    fn mark_undone(&mut self) {
        let index = self.current_index;
        self.tasks[index].done = false;

        // insert into undone_indexes at correct sorted position
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
        println!("↩️  Marked undone");
    }

    fn is_done(&self) -> bool {
        self.undone_indexes.is_empty()
    }

    /// save the current tasks and current undone position to disk
    fn persist(&self) {
        let tasklist = TaskListRef {
            tasks: &self.tasks,
            current_index: self
                .undone_indexes
                .get(self.undone_pos)
                .copied()
                .unwrap_or(0),
        };

        let path = get_tasks_file();
        let toml = toml::to_string_pretty(&tasklist).expect("Failed to serialize tasks");
        fs::write(path, toml).expect("Failed to write tasks file");

        persist_undone_indexes(&self.undone_indexes);
    }
}

fn get_tasks_file() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".tasks")
}

/// Keep a cache of indexes of undone tasks to prevent recalculating each time
fn get_undone_file() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".task_undone")
}

fn load_undone_indexes(tasks: &[Task]) -> Vec<usize> {
    let path = get_undone_file();
    if path.exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        let mut indexes: Vec<usize> = content
            .lines()
            .filter_map(|line| line.parse::<usize>().ok())
            .collect();

        // validate indexes against current tasks
        indexes.retain(|&i| i < tasks.len() && !tasks[i].done);
        if indexes.is_empty() {
            indexes = rebuild_undone(tasks);
        }
        indexes
    } else {
        rebuild_undone(tasks)
    }
}

fn rebuild_undone(tasks: &[Task]) -> Vec<usize> {
    tasks
        .iter()
        .enumerate()
        .filter_map(|(i, t)| if !t.done { Some(i) } else { None })
        .collect()
}

fn persist_undone_indexes(indexes: &[usize]) {
    let content = indexes
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(get_undone_file(), content).expect("Failed to write undone indexes")
}

fn first_undone(tasks: &[Task]) -> usize {
    tasks.iter().position(|t| !t.done).unwrap_or(0)
}

fn load_tasks() -> TaskList {
    let path = get_tasks_file();
    if !path.exists() {
        // Initialize empty list if no file
        return TaskList {
            tasks: vec![],
            current_index: 0,
        };
    }
    let content = fs::read_to_string(path).expect("Failed to read tasks file");
    let mut tasks_list: TaskList =
        toml::from_str(&content).expect("Invalid TOML format in tasks file");
    // make sure index is valid
    if tasks_list.current_index >= tasks_list.tasks.len() {
        tasks_list.current_index = first_undone(&tasks_list.tasks);
    }
    tasks_list
}

fn print_progress(manager: &TaskManager) {
    let total = manager.tasks.len();
    let done_count = manager.tasks.iter().filter(|t| t.done).count();
    let undone_count = total - done_count;
    let bar_width = 70;
    let filled = bar_width * done_count / total;
    let empty = bar_width - filled;
    let percent_done = (done_count * 100) / total;

    let task_status: String = if manager.current_task().done {
        "✅".into()
    } else {
        "❌".into()
    };

    println!("\n[");
    println!(
        "{} Task {} of {}:",
        task_status,
        manager.current_task().id,
        total
    );

    println!("\n==============================");
    println!(
        "Progress: {}/{} done | {} undone | {}%",
        done_count, total, undone_count, percent_done
    );
    println!(
        "[{}{}] {}%",
        "#".repeat(filled),
        "-".repeat(empty),
        percent_done
    );
    println!("==============================\n");

    println!("{}\n", manager.current_task().title);
    println!("{}\n", manager.current_task().description);
    println!("\nd:mark done / u:mark undone / p:previous task / n:next task / q:quit ?\n");
    println!("]");
}

fn main() {
    let tasks = load_tasks();
    if tasks.tasks.is_empty() {
        println!("No tasks defined in ~/.tasks");
        return;
    }

    let mut manager = TaskManager::new(tasks.tasks);

    loop {
        print_progress(&manager);

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim();

        match cmd {
            "d" => {
                manager.mark_done();
            }
            "u" => {
                manager.mark_undone();
            }
            "p" => {
                manager.previous();
            }
            "n" => {
                manager.next();
            }
            "N" => {
                manager.next_undone();
            }
            "q" => {
                println!("Bye 👋");
                break;
            }
            _ => {
                println!("Unknown command");
            }
        }
    }
}

