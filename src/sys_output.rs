use sysinfo::System;

pub fn print_cpu() -> usize {
    System::physical_core_count().unwrap_or(0)
}

pub fn logs() {
    println!("{}", print_cpu());
}