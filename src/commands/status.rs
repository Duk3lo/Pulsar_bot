use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage, Message,
};
use serenity::builder::{CreateEmbed, CreateMessage};

use crate::commands::shared::opt_u64;
use crate::{
    commands::shared::{opt_bool, register_status_command},
    status::metrics::collect_status,
    task::embed_updater::{self as live_status, LiveEmbedRenderer},
    ui::embeds,
};

pub struct StatusRenderer;

impl LiveEmbedRenderer for StatusRenderer {
    fn render(&self, frame: &str) -> CreateEmbed {
        let status = collect_status();
        embeds::info_embed(frame, &status)
    }
}

static STATUS_FRAMES: [&str; 4] = ["|", "/", "-", "\\"];
static STATUS_FRAMES_CACHE: OnceLock<Arc<[String]>> = OnceLock::new();

fn spinner_frames_cached() -> Arc<[String]> {
    STATUS_FRAMES_CACHE
        .get_or_init(|| {
            STATUS_FRAMES
                .iter()
                .map(|s| (*s).to_string())
                .collect::<Vec<_>>()
                .into()
        })
        .clone()
}

pub fn register_status(name: &'static str) -> CreateCommand {
    register_status_command(name)
}

pub async fn run_slash_status(ctx: &Context, command: &CommandInteraction) {
    let scope = live_status::LiveScope::from_command(command);
    let update = opt_bool(command, "update");
    let duration_secs = opt_u64(command, "duration").unwrap_or(1);

    let renderer = StatusRenderer;
    let frames = spinner_frames_cached();
    let first_frame = frames.first().map(|s| s.as_str()).unwrap_or("|");

    let data = CreateInteractionResponseMessage::new()
        .add_embed(renderer.render(first_frame));
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
            Duration::from_secs(duration_secs),
            frames,
            1,
            true,
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