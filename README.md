# Tasklings 🐣
Track your long-term goals one step at a time.  
Break big dreams into small tasks, stay focused, and keep moving forward.

➡️ Simple, minimal, and progress-driven ⬅️  


![tasklings onboarding image](./res/3.png)

## Installation & Usage

To install `tasklings`, ensure you have Rust and Cargo installed, then run:

```bash
cargo install tasklings
```

After installation, you can run the tool from your terminal:

```bash
tasklings
```

## Features

- **Create, Edit, and Manage Tasks**: Add new tasks or edit existing ones directly from the application using your default command-line editor.
- **GitHub Sync**: Keep your tasks synced across multiple machines by pushing and pulling from a GitHub repository. The git repository is now located in `~/.tasks/tasks` and uses the `main` branch.
- **Vim-like Keybindings**: Navigate your tasks with `j` and `k`.
- **Minimalist UI**: Stay focused on your tasks with a clean, distraction-free interface.

## Keybindings

- `d`: Mark task as done
- `u`: Mark task as undone
- `j` / `→`: Next task
- `k` / `←`: Previous task
- `N`: Next undone task
- `f`: First undone task
- `l`: Last task
- `n`: New task
- `e`: Edit task
- `p`: Pull from remote repository
- `P`: Push to remote repository
- `q` / `Esc`: Quit

## Why Tasklings?

It answers one daily question:  
> "What should I do today to make 1% progress?"


Your job: follow and execute.  


![tasklings onboarding image 3](./res/2.png)

--- 

Inspired by [Rustlings](https://github.com/rust-lang/rustlings).