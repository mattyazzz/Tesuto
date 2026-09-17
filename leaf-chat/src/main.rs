slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    println!("log 1: arrancó");

    let app = AppWindow::new()?;
    println!("log 2: AppWindow creado");

    app.show()?;
    println!("log 3: AppWindow mostrado");

    slint::run_event_loop()?;

    println!("log 4: event loop terminado");

    Ok(())
}