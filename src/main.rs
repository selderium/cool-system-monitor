use sysinfo::System;

fn main() {
    print_cpu()
}

fn print_cpu() {
    let cores = System::physical_core_count();

    match cores {
        Ok(count) => println!("Physical cores: {}", count),
        Err(err) => println!("Unable to determine physical core count: {}", err),
    }
}
