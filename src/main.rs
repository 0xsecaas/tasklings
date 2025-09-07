//! The main entry point for the Tasklings application.

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
    Terminal,
};
use std::{error::Error, io};

mod app;
mod input;
mod persistence;
mod tasks;

use app::App;
use input::InputEvent;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

/// Runs the main application loop.
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        match input::handle_input()? {
            InputEvent::Quit => app.quit(),
            InputEvent::MarkDone => app.mark_done(),
            InputEvent::MarkUndone => app.mark_undone(),
            InputEvent::NextTask => app.next_task(),
            InputEvent::PreviousTask => app.previous_task(),
            InputEvent::NextUndoneTask => app.next_undone_task(),
            InputEvent::FirstUndone => app.first_undone_task(),
            InputEvent::LastTask => app.last_task(),
            InputEvent::Noop => {}
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

/// Renders the UI.
fn ui(f: &mut Frame, app: &App) {
    let size = f.area();
    let current_task = app.task_manager.current_task();
    let total = app.task_manager.tasks.len();
    let done_count = app.task_manager.tasks.iter().filter(|t| t.done).count();
    let undone_count = total - done_count;
    let percent_done = if total > 0 {
        (done_count * 100) / total
    } else {
        0
    };

    let task_status = if current_task.done { "✅" } else { "❌" };

    let header_text = format!("{} Task {} of {}:", task_status, current_task.id, total);

    let progress_text = format!("Progress: {}/{} done | {} undone", done_count, total, undone_count);

    let title_text = current_task.title.to_string();
    let description_text = current_task.description.to_string();

    let footer_text =
        "d:mark done / u:mark undone / p:prev / n:next / N:next undone / f:first undone / l:last / q:quit";

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(size);

    let available_width = (size.width as usize).saturating_sub(10);
    let filled_width = (percent_done * available_width) / 100;
    let empty_width = available_width - filled_width;
    let progress_bar_line = Line::from(format!("[{}{}] {}%",
        "#".repeat(filled_width),
        "-".repeat(empty_width),
        percent_done
    ));

    let main_content = vec![
        Line::from(header_text),
        Line::from(""),
        Line::from("=============================="),
        Line::from(progress_text),
        progress_bar_line,
        Line::from("=============================="),
        Line::from(""),
        Line::from(title_text),
        Line::from(""),
        Line::from(description_text),
    ];

    let main_paragraph = Paragraph::new(main_content)
        .block(Block::default().borders(Borders::ALL).title("Tasklings"));
    f.render_widget(main_paragraph, chunks[0]);

    let footer_paragraph = Paragraph::new(footer_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(footer_paragraph, chunks[1]);
}
