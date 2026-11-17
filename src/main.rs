slint::include_modules!();

use cool_system_monitor::sys_output::print_phys_cpu;
use cool_system_monitor::sys_output::print_cpu_usage;

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new().unwrap();

    //println!("CPU Usage: {:.1}%", print_cpu_usage());
    print_cpu_usage();
    println!("Physical cores:  {}", print_phys_cpu());

    //main_window.set_cpu_usage(print_cpu_usage());

    let print_phys_cpu_u16: u16 = print_phys_cpu() as u16;
    main_window.set_cpu_cores(print_phys_cpu_u16.into());

    main_window.run()
}