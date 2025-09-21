//! User input handling.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io;

/// Input events.
#[derive(Debug, PartialEq, Eq)]
pub enum InputEvent {
    Quit,
    MarkDone,
    MarkUndone,
    NextTask,
    PreviousTask,
    NextUndoneTask,
    FirstUndone,
    LastTask,
    NewTask,
    EditTask,
    GitPush,
    GitPull,
    Noop,
}

/// Handles user input.
pub fn handle_input() -> io::Result<InputEvent> {
    if event::poll(std::time::Duration::from_millis(250))? {
        let event = event::read()?;
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(InputEvent::Quit),
                    KeyCode::Char('d') => return Ok(InputEvent::MarkDone),
                    KeyCode::Char('u') => return Ok(InputEvent::MarkUndone),
                    KeyCode::Char('j') | KeyCode::Right => return Ok(InputEvent::NextTask),
                    KeyCode::Char('k') | KeyCode::Left => return Ok(InputEvent::PreviousTask),
                    KeyCode::Char('N') => return Ok(InputEvent::NextUndoneTask),
                    KeyCode::Char('f') => return Ok(InputEvent::FirstUndone),
                    KeyCode::Char('l') => return Ok(InputEvent::LastTask),
                    KeyCode::Char('n') => return Ok(InputEvent::NewTask),
                    KeyCode::Char('e') => return Ok(InputEvent::EditTask),
                    KeyCode::Char('P') => return Ok(InputEvent::GitPush),
                    KeyCode::Char('p') => return Ok(InputEvent::GitPull),
                    _ => {}
                }
            }
        }
    }
    Ok(InputEvent::Noop)
}
