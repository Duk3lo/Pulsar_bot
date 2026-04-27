use std::sync::{Mutex, OnceLock};
use sysinfo::{System, get_current_pid, ProcessRefreshKind};

pub struct StatusSnapshot {
    pub cpu: f32,
    pub total_ram: u64,
    pub used_ram: u64,
    pub bot_ram: u64,
}
static SYSTEM: OnceLock<Mutex<System>> = OnceLock::new();

fn get_system() -> &'static Mutex<System> {
    SYSTEM.get_or_init(|| Mutex::new(System::new()))
}

pub fn collect_status() -> StatusSnapshot {
    let mut sys = get_system().lock().unwrap();
    sys.refresh_memory();
    sys.refresh_cpu();

    let pid = get_current_pid().unwrap();

    sys.refresh_process_specifics(pid, ProcessRefreshKind::new().with_memory());

    let cpu = sys.global_cpu_info().cpu_usage();
    let total_ram = sys.total_memory();
    let used_ram = sys.used_memory();
    let bot_ram = sys.process(pid).map(|p| p.memory()).unwrap_or(0);

    StatusSnapshot {
        cpu,
        total_ram,
        used_ram,
        bot_ram,
    }
}

pub fn format_bytes(value: u64) -> String {
    let gb = value as f64 / 1024.0 / 1024.0 / 1024.0;
    if gb >= 1.0 {
        format!("{:.2} GB", gb)
    } else {
        let mb = value as f64 / 1024.0 / 1024.0;
        format!("{:.0} MB", mb)
    }
}