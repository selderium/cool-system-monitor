slint::include_modules!();

use cool_system_monitor::sys_output::{ print_cpu_usage, print_phys_cpu, print_total_ram };

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new().unwrap();

    println!("CPU Usage: {}%", print_cpu_usage());
    println!("Physical cores:  {}", print_phys_cpu());

    main_window.set_cpu_usage(print_cpu_usage().into());
    main_window.set_cpu_cores(print_phys_cpu().into());

    main_window.on_print_cpu_usage(move || {
        print_cpu_usage()
    });

    main_window.set_ram_total((print_total_ram() as i32).into());

    main_window.run()
}