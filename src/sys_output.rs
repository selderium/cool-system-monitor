use sysinfo::System;

pub fn print_cpu_usage() -> i32 {
    let mut sys = System::new_all().unwrap();
    sys.refresh_cpu_usage();

    sys.global_cpu_usage() as i32
}

pub fn print_phys_cpu() -> i32 {
    System::physical_core_count().unwrap_or(0) as i32
}

pub fn print_total_ram() -> u64 {
    let sys = System::new_all().unwrap();
    sys.total_memory() / 1024 / 1024
}
