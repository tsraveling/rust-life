mod grid;

use std::time::{Duration, Instant};

use crate::grid::Grid;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

const TICK_RATE: Duration = Duration::from_millis(500);

// SECTION: Model

struct LifeViewModel {
    counter: u64,
    current: Grid,
    next: Grid,
}

impl LifeViewModel {
    fn new(w: usize, h: usize) -> Self {
        Self {
            counter: 0,
            current: Grid::new(w, h),
            next: Grid::new(w, h),
        }
    }
}

// SECTION: Update

enum Msg {
    Tick,
    Quit,
}

/// Returns `false` when the app should exit.
fn update(model: &mut LifeViewModel, msg: Msg) -> bool {
    match msg {
        Msg::Tick => {
            // Logic
            // STUB: Just flip the grid for now
            for x in 0..model.current.width {
                for y in 0..model.current.height {
                    model.next.set(x, y, !model.current.get(x, y))
                }
            }

            // Finally, swap the buffers
            std::mem::swap(&mut model.current, &mut model.next);

            // Meta
            model.counter += 1;
            true
        }
        Msg::Quit => false,
    }
}

// SECTION: View

fn view(f: &mut ratatui::Frame, model: &LifeViewModel) {
    let mut lines = String::new();
    for y in 0..model.current.height {
        for x in 0..model.current.width {
            let val = model.current.get(x, y);
            lines.push(if val { 'X' } else { ' ' });
        }
        lines.push('\n');
    }
    let para = Paragraph::new(lines).style(Style::default().fg(Color::Yellow));

    // This is a ratatui method that actually renders the frame to the buffer.
    f.render_widget(para, f.area());
}

// SECTION: Main

fn main() -> Result<()> {
    // Setup terminal with raw mode: gives us more precise control over inputs etc.
    // Also disables ctrl+c so you would have to support that manually.
    enable_raw_mode()?;

    // Enters TUI "alternate screen" aka full screen mode.
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // These connect us to the actual terminal so we can do stuff there
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Builds our TEA model
    let (w, h) = crossterm::terminal::size()?;
    let mut model = LifeViewModel::new(w.into(), h.into());
    let mut last_tick = Instant::now();

    loop {
        // Draw:
        // This syntax basically passes a closure that operates on frame f, in this
        // case our view method above. The question mark basically is Rust's way of
        // saying "if this fails, throw the error to Result above (if err { return err})"
        terminal.draw(|f| view(f, &model))?;

        // Poll for input, sleeping at most until the next tick
        let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
        {
            // This bit translates key presses into explicit messages;
            // in future I might just pass em directly.
            let msg = match key.code {
                KeyCode::Char('q') => Msg::Quit,
                _ => continue,
            };
            if !update(&mut model, msg) {
                break;
            }
        }

        // Fire a tick when the interval elapses
        if last_tick.elapsed() >= TICK_RATE {
            if !update(&mut model, Msg::Tick) {
                break;
            }
            last_tick = Instant::now();
        }
    }

    // Relinquish total control over inputs
    disable_raw_mode()?;

    // Exit alt mode
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    // Return control to term
    terminal.show_cursor()?;

    // It all worked, boss!
    // () is Rust "unit type", it's basically an empty thing, "nothing to add here."
    Ok(())
}
