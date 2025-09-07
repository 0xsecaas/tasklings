use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
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
    current_index: usize, // optional, defaults to 0 if missing
}

fn get_tasks_file() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".tasks")
}

fn load_tasks() -> TaskList {
    let path = get_tasks_file();
    if !path.exists() {
        // Initialize empty list if no file
        let empty = TaskList {
            tasks: vec![],
            current_index: 0,
        };
        save_tasks(&empty);
        return empty;
    }
    let content = fs::read_to_string(path).expect("Failed to read tasks file");
    let mut tasks_list: TaskList =
        toml::from_str(&content).expect("Invalid TOML format in tasks file");
    // make sure index is valid
    if tasks_list.current_index >= tasks_list.tasks.len() {
        tasks_list.current_index = tasks_list.tasks.iter().position(|t| !t.done).unwrap_or(0);
    }
    tasks_list
}

fn save_tasks(tasklist: &TaskList) {
    let path = get_tasks_file();
    let toml = toml::to_string_pretty(tasklist).expect("Failed to serialize tasks");
    fs::write(path, toml).expect("Failed to write tasks file");
}

fn print_progress(tasks: &TaskList) {
    let total = tasks.tasks.len();
    let done_count = tasks.tasks.iter().filter(|t| t.done).count();

    let bar_width = 70;
    let filled = bar_width * done_count / total;
    let empty = bar_width - filled;

    println!("\n[");
    println!("Task {} of {}:", tasks.current_index + 1, total);
    println!("{}\n", tasks.tasks[tasks.current_index].title);
    println!("{}\n", tasks.tasks[tasks.current_index].description);
    println!(
        "Progress: [{}{}] {}/{}",
        "#".repeat(filled),
        "-".repeat(empty),
        done_count,
        total
    );

    println!("\nd:mark done / u:mark undone / p:previous task / n:next task / q:quit ?\n");
    println!("]");
}

fn main() {
    let mut tasks = load_tasks();
    if tasks.tasks.is_empty() {
        println!("No tasks defined in ~/.tasks");
        return;
    }

    loop {
        print_progress(&tasks);

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim();

        match cmd {
            "d" => {
                tasks.tasks[tasks.current_index].done = true;

                // Move to the next undone task automatically
                if let Some(next) = tasks.tasks.iter().position(|t| !t.done) {
                    tasks.current_index = next;
                }

                save_tasks(&tasks);
                print!("✅ Marked done");
            }
            "u" => {
                tasks.tasks[tasks.current_index].done = false;
                save_tasks(&tasks);
                println!("↩️  Marked undone");
            }
            "p" => {
                tasks.current_index = tasks.current_index.saturating_sub(1);
            }
            "n" => {
                if tasks.current_index + 1 < tasks.tasks.len() {
                    tasks.current_index += 1;
                    save_tasks(&tasks);
                }
            }
            "q" => {
                println!("Bye 👋");
            }
            _ => {
                println!("Unknown command");
            }
        }
    }
}
