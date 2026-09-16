slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    println!("1. arrancó");

    let app = AppWindow::new()?;
    println!("2. AppWindow creado");

    app.show()?;
    println!("3. AppWindow mostrado");

    slint::run_event_loop()?;

    println!("4. salió del event loop");

    Ok(())
}