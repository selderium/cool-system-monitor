slint::include_modules!();

use cool_system_monitor::sys_output::{
    print_cores_usage, print_cpu_usage, print_phys_cpu, print_total_ram, refresh_cpu_usage,
};

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new().unwrap();

    // Static

    main_window.set_cpu_cores(print_phys_cpu().into());
    main_window.set_ram_total((print_total_ram() as i32).into());

    // Dynamic

    let window = main_window.as_weak();
    main_window.on_update_cpu_usage(move || {
        refresh_cpu_usage();
        let window = window.unwrap();
        window.set_cpu_usage(print_cpu_usage());
        window.set_cores_usage(print_cores_usage().as_slice().into());
    });

    main_window.run()
}
