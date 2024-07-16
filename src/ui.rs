use std::{
    io,
    time::{Duration, Instant},
};

use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    Frame, Terminal,
};

use crate::app::App;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
    let area = frame.size();

    let vertical = Layout::vertical([Constraint::Percentage(40), Constraint::Percentage(60)]);
    let horizontal = Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]);
    let [cpu_area, remaining_area] = vertical.areas(area);
    let [memory_area, disk_area] = horizontal.areas(remaining_area);

    app.cpu.render(frame, cpu_area);
    app.memory.render(frame, memory_area);
    app.disk.render(frame, disk_area);
}
