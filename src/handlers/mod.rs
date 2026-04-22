pub mod welcome;

use crate::commands;
use serenity::all::{Command, Ready};
use serenity::async_trait;
use serenity::model::application::Interaction;
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::prelude::*;
use tokio::task;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("✨ {} conectado. Registrando comandos...", ready.user.name);
        let commands = commands::get_all_commands();
        if let Err(why) = Command::set_global_commands(&ctx.http, commands).await {
            println!("❌ Error al registrar comandos: {:?}", why);
        } else {
            println!("✅ Comandos sincronizados con la barra de Discord.");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            task::spawn(async move {
                commands::dispatch_interaction(&ctx, &command).await;
            });
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase() == "!ping" {
            task::spawn(async move {
                commands::utils::run_manual_ping(&ctx, &msg).await;
            });
        }
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        task::spawn(async move {
            welcome::handle_welcome(ctx, new_member).await;
        });
    }
}