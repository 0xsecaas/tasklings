//! User input handling.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io;

/// Input events.
pub enum InputEvent {
    Quit,
    MarkDone,
    MarkUndone,
    NextTask,
    PreviousTask,
    NextUndoneTask,
    Noop,
}

/// Handles user input.
pub fn handle_input() -> io::Result<InputEvent> {
    if event::poll(std::time::Duration::from_millis(250))?
        && let Event::Key(key) = event::read()?
        && key.kind == KeyEventKind::Press
    {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(InputEvent::Quit),
            KeyCode::Char('d') => return Ok(InputEvent::MarkDone),
            KeyCode::Char('u') => return Ok(InputEvent::MarkUndone),
            KeyCode::Char('n') | KeyCode::Right => return Ok(InputEvent::NextTask),
            KeyCode::Char('p') | KeyCode::Left => return Ok(InputEvent::PreviousTask),
            KeyCode::Char('N') => return Ok(InputEvent::NextUndoneTask),
            _ => {}
        }
    }
    Ok(InputEvent::Noop)
}
