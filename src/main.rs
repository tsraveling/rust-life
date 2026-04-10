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
    style::{Color, Style},
    widgets::Paragraph,
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
        // Add some startup noise
        let mut g = Grid::new(w, h);
        g.add_noise();
        g.add_noise();
        Self {
            counter: 0,
            current: g,
            next: Grid::new(w, h),
        }
    }
}

// SECTION: Update

enum Msg {
    Tick,
    Quit,
    Noise,
}

/// Returns `false` when the app should exit.
fn update(model: &mut LifeViewModel, msg: Msg) -> bool {
    match msg {
        Msg::Tick => {
            // Logic
            for x in 0..model.current.width {
                for y in 0..model.current.height {
                    let was_alive = model.current.get(x, y);
                    let n = model.current.neighbor_count(x, y);

                    // This line here is the entirety of Conway's Game of Life!
                    let alive = (was_alive && n == 2) || n == 3;

                    model.next.set(x, y, alive)
                }
            }

            // Finally, swap the buffers
            std::mem::swap(&mut model.current, &mut model.next);

            // Meta
            model.counter += 1;
            true
        }
        Msg::Quit => false,
        Msg::Noise => {
            model.current.add_noise();
            true
        }
    }
}

// SECTION: View

fn view(f: &mut ratatui::Frame, model: &LifeViewModel) {
    let lines: Vec<ratatui::text::Line> = (0..model.current.height)
        .map(|y| {
            let content: String = (0..model.current.width)
                .map(|x| if model.current.get(x, y) { 'X' } else { ' ' })
                .collect();
            ratatui::text::Line::from(content)
        })
        .collect();

    let para = Paragraph::new(lines).style(Style::default().fg(Color::Yellow));
    f.render_widget(para, f.area());
}

// SECTION: Terminal setup/teardown

fn setup_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

/// Ditch cleanup errors, just restore the terminal so we can print out the relevant stuff
fn restore_terminal() {
    let _ = disable_raw_mode();
    let _ = execute!(std::io::stdout(), LeaveAlternateScreen);
}

// SECTION: Main loop

fn run(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let (w, h) = crossterm::terminal::size()?;
    let mut model = LifeViewModel::new(w.into(), h.into());
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| view(f, &model))?;

        let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
        {
            let msg = match key.code {
                KeyCode::Char('q') => Msg::Quit,
                KeyCode::Char('a') => Msg::Noise,
                _ => continue,
            };
            if !update(&mut model, msg) {
                break;
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            if !update(&mut model, Msg::Tick) {
                break;
            }
            last_tick = Instant::now();
        }
    }

    Ok(())
}

// SECTION: Main

fn main() -> Result<()> {
    // Install panic hook so terminal restores even on panic
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_terminal();
        default_hook(info);
    }));

    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal);
    restore_terminal();
    result
}
