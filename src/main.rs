pub mod commands;
pub mod handlers;
pub mod ui;
pub mod workspace;

use crate::{handlers::handler, workspace::paths::{self, WORKSPACE}};
use dotenvy::dotenv;
use serenity::prelude::*;
use songbird::SerenityInit;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Token no encontrado");
    let workspace = match paths::Workspace::load_workspace() {
        Ok(ws) => ws,
        Err(e) => {
            println!("Error cargando workspace: {:?}", e);
            return;
        }
    };

    WORKSPACE.get_or_init(|| workspace);

    println!("Directorio de audio verificado en: {:?}", paths::Workspace::global().folder_audio);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES;

    let client_result = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .register_songbird()
        .await;

    match client_result {
        Ok(mut client) => {
            println!("Bot configurado. Intentando conectar con Discord...");
            if let Err(why) = client.start().await {
                eprintln!("Error fatal durante la ejecución del bot: {:?}", why);
            }
        }
        Err(why) => {
            eprintln!("Error crítico al configurar el cliente: {:?}", why);
        }
    }
}
