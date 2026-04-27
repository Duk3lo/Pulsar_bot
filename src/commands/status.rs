use std::time::Duration;

use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, CreateCommand,
    CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage,
    InstallationContext, InteractionContext, Message,
};
use serenity::builder::CreateMessage;
use serenity::prelude::*;

use crate::status::metrics::collect_status;
use crate::{task::status, ui::embeds};

pub fn register_status() -> CreateCommand {
    CreateCommand::new("status")
        .description("Muestra información sobre el bot")
        .integration_types(vec![
            InstallationContext::Guild,
            InstallationContext::User,
        ])
        .contexts(vec![
            InteractionContext::Guild,
            InteractionContext::BotDm,
            InteractionContext::PrivateChannel,
        ])
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "update",
                "Inicia la actualización automática del embed",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "stop",
                "Detiene la actualización activa",
            )
            .required(false),
        )
}

fn opt_bool(command: &CommandInteraction, name: &str) -> bool {
    command
        .data
        .options
        .iter()
        .any(|opt| opt.name == name && matches!(opt.value, CommandDataOptionValue::Boolean(true)))
}

pub async fn run_slash_status(ctx: &Context, command: &CommandInteraction) {
    let scope = status::StatusScope::from_command(command);
    let stop = opt_bool(command, "stop");
    let update = opt_bool(command, "update");

    if stop {
        let stopped = status::stop_status(scope).await;

        let content = if stopped {
            "Actualización detenida."
        } else {
            "No había una actualización activa."
        };

        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);

        if let Err(err) = command.create_response(&ctx.http, builder).await {
            eprintln!("Error respondiendo stop: {err:?}");
        }
        return;
    }

    let status = collect_status();
    let embed = embeds::info_embed("", &status);
    let data = CreateInteractionResponseMessage::new().add_embed(embed);
    let builder = CreateInteractionResponse::Message(data);

    if let Err(err) = command.create_response(&ctx.http, builder).await {
        eprintln!("Error en slash status: {err:?}");
        return;
    }

    if update {
        if let Err(err) = status::start_interaction_updater(
            ctx.clone(),
            scope,
            command.user.id,
            command.token.clone(),
            Duration::from_secs(2),
        )
        .await
        {
            eprintln!("{err}");
        }
    }
}

pub async fn run_manual_status(ctx: &Context, msg: &Message) {
    let status = collect_status();
    let embed = embeds::info_embed("", &status);
    let builder = CreateMessage::new().add_embed(embed);

    if let Err(err) = msg.channel_id.send_message(&ctx.http, builder).await {
        eprintln!("Error en manual status: {err:?}");
    }
}