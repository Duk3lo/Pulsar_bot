mod ui;
mod commands;
mod handlers;

use std::env;
use dotenvy::dotenv;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Token no encontrado");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&token, intents)
        .event_handler(handlers::Handler)
        .await
        .expect("Error creando el cliente");

    println!("🚀 Bot iniciado. Separando tareas por hilos...");

    if let Err(why) = client.start().await {
        println!("Error: {why:?}");
    }
}