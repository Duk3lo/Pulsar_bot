pub mod ui;
pub mod commands;
pub mod handlers;
pub mod workspace;

use std::env;
use dotenvy::dotenv;
use serenity::prelude::*;
use songbird::SerenityInit;
use crate::handlers::handler;

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
        .event_handler(handler::Handler)
        .register_songbird()
        .await
        .expect("Error creando el cliente");

    println!("🚀 Bot iniciado. Separando tareas por hilos...");

    if let Err(why) = client.start().await {
        println!("Error: {why:?}");
    }
}