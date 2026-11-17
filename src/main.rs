slint::include_modules!();

use cool_system_monitor::sys_output::print_cpu_usage;
use cool_system_monitor::sys_output::print_phys_cpu;

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new().unwrap();

    println!("CPU Usage: {}%", print_cpu_usage());
    println!("Physical cores:  {}", print_phys_cpu());

    main_window.set_cpu_usage(print_cpu_usage().into());
    main_window.set_cpu_cores(print_phys_cpu().into());

    main_window.run()
}
