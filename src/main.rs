#![windows_subsystem = "windows"]

slint::include_modules!();

use cool_system_monitor::sys_output::{
    print_available_ram, 
    print_cores_usage, 
    print_cpu_usage, 
    print_free_ram, 
    print_phys_cpu,
    print_total_ram, 
    print_used_ram, 
    refresh_cpu_usage, 
    refresh_ram_usage,
};

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new().unwrap();

    // Open dialog window
    main_window.on_open_about_dialog(move || {
        let about_dialog = AboutDialog::new().unwrap();
        about_dialog.show().unwrap();
    });

    // Static stats

    main_window.set_cpu_cores(print_phys_cpu().into());
    main_window.set_ram_total((print_total_ram() as i32).into());

    // Dynamic stats

    let window = main_window.as_weak();
    main_window.on_update_stats(move || {
        refresh_cpu_usage();
        refresh_ram_usage();
        let window = window.unwrap();
        window.set_cpu_usage(print_cpu_usage());
        window.set_cores_usage(print_cores_usage().as_slice().into());
        window.set_ram_available(print_available_ram() as i32);
        window.set_ram_free(print_free_ram() as i32);
        window.set_ram_used(print_used_ram() as i32);
    });

    main_window.run()
}
