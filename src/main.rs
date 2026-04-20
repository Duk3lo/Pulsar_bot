use std::env;
use dotenvy::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase() == "!ping" {
            let _ = msg.channel_id.say(&ctx.http, "Pong!").await;
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let token = env::var("DISCORD_TOKEN").expect("Token no encontrado en el entorno");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creando el cliente");

    if let Err(why) = client.start().await {
        println!("Error: {why:?}");
    }
}