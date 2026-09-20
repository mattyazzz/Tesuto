mod servers;

use slint::{ModelRc, VecModel};
slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new()?;

    let test = runtime.block_on(
        servers::obtener_mensajes()
    )?;

    println!("Mensajes recibidos:");
    println!("{:?}", test);
    for mensaje in &test {
        println!("Contenido: {:?}", mensaje.content);
    }
    for user in &test {
        println!("id: {:?}", user.sender_id);
    }
    let mensajes: Vec<slint::SharedString> = test
        .into_iter()
        .filter_map(|mensaje| mensaje.content)
        .map(slint::SharedString::from)
        .collect();
    let app = AppWindow::new()?;
    let modelo = ModelRc::new(VecModel::from(mensajes));
    app.set_mensajes(modelo);
    app.run()?;

    Ok(())
}