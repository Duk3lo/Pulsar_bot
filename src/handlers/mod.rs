pub mod welcome;

use crate::commands;
use serenity::async_trait;
use serenity::all::{Command, CreateCommand, Ready};
use serenity::model::application::Interaction;
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::prelude::*;
use tokio::task;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("✨ {} conectado.", ready.user.name);

        let commands = vec![
            CreateCommand::new("ping").description("Muestra el estado del sistema"),
            CreateCommand::new("welcome").description("Ejecuta manualmente la bienvenida"),
        ];

        if let Err(why) = Command::set_global_commands(&ctx.http, commands).await {
            println!("❌ Error al registrar comandos: {:?}", why);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "ping" => {
                    task::spawn(async move {
                        commands::utils::run_slash_ping(ctx, command).await;
                    });
                }
                "welcome" => {
                    task::spawn(async move {
                        if let Some(member) = command.member.clone() {
                            let response = serenity::all::CreateInteractionResponse::Message(
                                serenity::all::CreateInteractionResponseMessage::new()
                                    .content("Procesando protocolo de bienvenida... 🚀")
                                    .ephemeral(true),
                            );
                            let _ = command.create_response(&ctx.http, response).await;
                            welcome::handle_welcome(ctx, *member).await;
                        }
                    });
                }
                _ => (),
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase() == "!ping" {
            task::spawn(async move {
                commands::utils::run_manual_ping(ctx, msg).await;
            });
        }
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        task::spawn(async move {
            welcome::handle_welcome(ctx, new_member).await;
        });
    }
}