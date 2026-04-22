mod ui;
mod commands;
mod handlers;

use std::env;
use dotenvy::dotenv;
use serenity::prelude::*;
use songbird::SerenityInit;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Token no encontrado");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token, intents)
        .event_handler(handlers::Handler)
        .register_songbird()
        .await
        .expect("Error creando el cliente");

    println!("🚀 Bot iniciado. Separando tareas por hilos...");

    if let Err(why) = client.start().await {
        println!("Error: {why:?}");
    }
}