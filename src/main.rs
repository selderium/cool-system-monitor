slint::include_modules!();

use cool_system_monitor::sys_output::logs;
use cool_system_monitor::sys_output::print_cpu;

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new().unwrap();

    logs();

    let print_cpu_u16: u16 = print_cpu() as u16;
    main_window.set_cores(print_cpu_u16.into());

    main_window.run()
}