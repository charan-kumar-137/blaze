use crate::{cpu::CPUInfo, disk::DiskInfo, memory::MemoryInfo};

pub struct App {
    pub cpu: CPUInfo,
    pub memory: MemoryInfo,
    pub disk: DiskInfo,
}

impl App {
    pub fn new() -> Self {
        Self {
            cpu: CPUInfo::new(),
            memory: MemoryInfo::new(),
            disk: DiskInfo::new(),
        }
    }

    pub fn on_tick(&mut self) {
        self.cpu.refresh();
        self.memory.refresh();
        self.disk.refresh();
    }
}
