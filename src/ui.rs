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

use crate::{app::App, cpu::CPUInfo, disk::DiskInfo, memory::MemoryInfo};

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f))?;

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

fn ui(frame: &mut Frame) {
    let area = frame.size();

    let vertical = Layout::vertical([Constraint::Percentage(40), Constraint::Percentage(60)]);
    let horizontal = Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]);
    let [chart1, bottom] = vertical.areas(area);
    let [line_chart, scatter] = horizontal.areas(bottom);

    let cpu = CPUInfo::new();
    cpu.render(frame, chart1);
    let memory = MemoryInfo::new();
    memory.render(frame, line_chart);
    let disk = DiskInfo::new();
    disk.render(frame, scatter);
}
