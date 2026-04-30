use std::time::Duration;

use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage, Message,
};
use serenity::builder::CreateMessage;

use crate::commands::shared::opt_bool;
use crate::{
    commands::shared::register_live_command,
    status::metrics::collect_status,
    task::embed_updater::{self as live_status, LiveEmbedRenderer},
    ui::embeds,
};

pub struct StatusRenderer;

impl LiveEmbedRenderer for StatusRenderer {
    fn render(&self, frame: &str) -> serenity::builder::CreateEmbed {
        let status = collect_status();
        embeds::info_embed(frame, &status)
    }
}

const SPINNER_FRAMES: [&str; 4] = ["/", "-", "\\", "|"];

fn spinner_frames() -> Vec<String> {
    SPINNER_FRAMES.iter().map(|s| s.to_string()).collect()
}

pub fn register_status() -> CreateCommand {
    register_live_command("status", "Muestra información sobre el bot")
}

pub async fn run_slash_status(ctx: &Context, command: &CommandInteraction) {
    let scope = live_status::LiveScope::from_command(command);
    let stop = opt_bool(command, "stop");
    let update = opt_bool(command, "update");

    if stop {
        let stopped = live_status::stop(scope).await;

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

    let renderer = StatusRenderer;
    let embed = renderer.render(SPINNER_FRAMES[0]);
    let data = CreateInteractionResponseMessage::new().add_embed(embed);
    let builder = CreateInteractionResponse::Message(data);

    if let Err(err) = command.create_response(&ctx.http, builder).await {
        eprintln!("Error en slash status: {err:?}");
        return;
    }

    if update {
        if let Err(err) = live_status::start(
            ctx.clone(),
            scope,
            command.user.id,
            command.token.clone(),
            Duration::from_secs(2),
            spinner_frames(),
            StatusRenderer,
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