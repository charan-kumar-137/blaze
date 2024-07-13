use std::{
    error::Error,
    fs::File,
    io::{Read, Seek},
};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Padding},
    Frame,
};

#[derive(Debug, Default)]
pub struct CPUInfo {
    stat_file: Option<File>,
    pub cores: Vec<CPUCore>,
}

#[derive(Debug)]
pub struct CPUCore {
    pub id: String,
    idle_time: usize,
    total_time: usize,
    pub utilization: f64,
}

impl CPUInfo {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let mut stat_file = File::open("/proc/stat")?;
        let mut buffer = String::new();
        stat_file.read_to_string(&mut buffer)?;

        let mut cores: Vec<CPUCore> = Vec::new();

        let mut lines = buffer.lines();
        lines.next();

        for line in lines {
            if line.starts_with("cpu") {
                let fields: Vec<&str> = line.split_whitespace().collect();

                let name: String = fields[0].parse()?;

                let user: usize = fields[1].parse()?;
                let nice: usize = fields[2].parse()?;
                let system: usize = fields[3].parse()?;
                let idle: usize = fields[4].parse()?;
                let iowait: usize = fields[5].parse()?;
                let irq: usize = fields[6].parse()?;
                let softirq: usize = fields[7].parse()?;
                let steal: usize = fields[8].parse()?;
                let guest: usize = fields[9].parse()?;
                let guest_nice: usize = fields[10].parse()?;

                let idle_time = idle + iowait;
                let systemd_all_time = system + irq + softirq;
                let virt_all_time = guest + guest_nice;
                let total_time = user + nice + systemd_all_time + idle_time + steal + virt_all_time;

                let core = CPUCore {
                    id: name,
                    idle_time,
                    total_time,
                    utilization: 0.0,
                };

                cores.push(core);
            }
        }

        Ok(Self {
            stat_file: Some(stat_file),
            cores,
        })
    }

    pub fn new() -> Self {
        match CPUInfo::init() {
            Ok(cpu) => cpu,
            Err(e) => {
                eprintln!("{}", e);
                CPUInfo::default()
            }
        }
    }

    pub fn refresh_utilization(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(fd) = &mut self.stat_file {
            fd.seek(std::io::SeekFrom::Start(0))?;

            let mut buffer = String::new();
            fd.read_to_string(&mut buffer)?;

            let mut lines = buffer.lines();
            lines.next();

            for line in lines {
                if line.starts_with("cpu") {
                    let fields: Vec<&str> = line.split_whitespace().collect();

                    let name: String = fields[0].parse()?;

                    let user: usize = fields[1].parse()?;
                    let nice: usize = fields[2].parse()?;
                    let system: usize = fields[3].parse()?;
                    let idle: usize = fields[4].parse()?;
                    let iowait: usize = fields[5].parse()?;
                    let irq: usize = fields[6].parse()?;
                    let softirq: usize = fields[7].parse()?;
                    let steal: usize = fields[8].parse()?;
                    let guest: usize = fields[9].parse()?;
                    let guest_nice: usize = fields[10].parse()?;

                    let idle_time = idle + iowait;
                    let systemd_all_time = system + irq + softirq;
                    let virt_all_time = guest + guest_nice;
                    let total_time =
                        user + nice + systemd_all_time + idle_time + steal + virt_all_time;

                    if let Some(core) = &mut self.cores.iter_mut().find(|core| core.id == name) {
                        core.utilization = {
                            let total_diff = (total_time - core.total_time) as f64;
                            let idle_diff = (idle_time - core.idle_time) as f64;

                            100.0 * (total_diff - idle_diff) / total_diff
                        };

                        core.total_time = total_time;
                        core.idle_time = idle_time;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn refresh(&mut self) {
        if let Err(e) = self.refresh_utilization() {
            eprintln!("{}", e);
        }
    }

    pub fn render(&self, frame: &mut Frame, block: Rect) {
        let container = Block::default()
            .borders(Borders::ALL)
            .title("CPU")
            .padding(Padding::horizontal(1))
            .title_style(Style::new().bold());

        let inside_container = container.inner(block);

        let (left_block, right_block) = {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(inside_container);

            (chunks[0], chunks[1])
        };

        let (left_cpu, right_cpu) = self.cores.split_at(self.cores.len() / 2);

        let left_cpu_barchat = BarChart::default()
            .block(Block::default())
            .bar_width(1)
            .group_gap(0)
            .direction(Direction::Horizontal)
            .data(
                BarGroup::default().bars(
                    &left_cpu
                        .iter()
                        .map(|core| {
                            Bar::default()
                                .label(Line::styled(&core.id, Style::default().bold()))
                                .value(core.utilization.round() as u64)
                                .text_value(format!("{:.1}% ", core.utilization))
                        })
                        .collect::<Vec<Bar>>(),
                ),
            )
            .max(100);

        let right_cpu_barchat = BarChart::default()
            .block(Block::default())
            .bar_width(1)
            .group_gap(0)
            .direction(Direction::Horizontal)
            .data(
                BarGroup::default().bars(
                    &right_cpu
                        .iter()
                        .map(|core| {
                            Bar::default()
                                .label(Line::styled(&core.id, Style::default().bold()))
                                .value(core.utilization.round() as u64)
                                .text_value(format!("{:.1}% ", core.utilization))
                        })
                        .collect::<Vec<Bar>>(),
                ),
            )
            .max(100);

        frame.render_widget(container, block);
        frame.render_widget(right_cpu_barchat, right_block);
        frame.render_widget(left_cpu_barchat, left_block);
    }
}
