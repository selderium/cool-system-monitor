use std::cell::RefCell;
use sysinfo::System;

thread_local! {
    static SYSTEM: RefCell<System> = RefCell::new(System::new_all().unwrap());
}

pub fn refresh_cpu_usage() {
    SYSTEM.with_borrow_mut(|sys| sys.refresh_cpu_usage());
}

pub fn print_phys_cpu() -> i32 {
    System::physical_core_count().unwrap_or(0) as i32
}

pub fn print_cpu_usage() -> i32 {
    SYSTEM.with_borrow(|sys| sys.global_cpu_usage() as i32)
}

pub fn print_cores_usage() -> Vec<i32> {
    SYSTEM.with_borrow(|sys| sys.cpus().iter().map(|cpu| cpu.usage() as i32).collect())
}

pub fn print_total_ram() -> u64 {
    let sys = System::new_all().unwrap();
    sys.total_memory() / 1024 / 1024
}
