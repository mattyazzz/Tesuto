use reqwest::Client;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
pub struct Mensaje {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub content: Option<String>,
    pub created_at: String,
}

pub async fn obtener_mensajes() -> Result<Vec<Mensaje>, Box<dyn std::error::Error>> {
    let url = std::env::var("SUPABASE_URL")
        .expect("Falta SUPABASE_URL");

    let key = std::env::var("SUPABASE_KEY")
        .expect("Falta SUPABASE_KEY");

    let client = Client::new();

    let respuesta = client
        .get(format!("{}/rest/v1/test?select=*", url))
        .header("apikey", key)
        .send()
        .await?;

    let text = respuesta.text().await?;

    let mensajes: Vec<Mensaje> = serde_json::from_str(&text)?;

    Ok(mensajes)
}
pub async fn enviar_mensaje(
    contenido: String,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}