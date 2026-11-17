use sysinfo::{ System, MINIMUM_CPU_UPDATE_INTERVAL };
use std::thread;

pub fn print_cpu_usage() {
    let mut sys = System::new_all().unwrap();

    sys.refresh_cpu_usage();
    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();

    println!("{:.1}%", sys.global_cpu_usage());
}

pub fn print_phys_cpu() -> usize {
    System::physical_core_count().unwrap_or(0)
}