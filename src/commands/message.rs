use std::{sync::OnceLock, time::Duration};

use serenity::{
    all::{ChannelId, CommandOptionType, CreateCommand, CreateCommandOption, InstallationContext, InteractionContext, MessageId},
    builder::{CreateEmbed, CreateMessage, EditMessage},
    prelude::Context,
};
use tokio::{sync::Mutex, task::JoinHandle, time::interval};

struct LiveMessage {
    channel_id: ChannelId,
    message_id: MessageId,
    handle: JoinHandle<()>,
}

static LIVE: OnceLock<Mutex<Option<LiveMessage>>> = OnceLock::new();

pub fn register_message() -> CreateCommand {
    CreateCommand::new("message")
        .description("Animacion")
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

fn live_slot() -> &'static Mutex<Option<LiveMessage>> {
    LIVE.get_or_init(|| Mutex::new(None))
}

fn build_embed(frame: &str) -> CreateEmbed {

    CreateEmbed::new()
        .title("Status")
        .description(format!("{frame}"))
}

pub async fn stop_live_message() {
    let mut slot = live_slot().lock().await;

    if let Some(active) = slot.take() {
        active.handle.abort();
    }
}

pub async fn start_live_message(
    ctx: Context,
    channel_id: ChannelId,
    every: Duration,
) -> serenity::Result<MessageId> {
    stop_live_message().await;

    let initial = channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new().embed(build_embed("/")),
        )
        .await?;

    let message_id = initial.id;
    let http = ctx.http.clone();

    let handle = tokio::spawn(async move {
        let frames = ["/", "-", "\\", "|"];
        let mut i = 0usize;
        let mut tick = interval(every);

        loop {
            tick.tick().await;

            let frame = frames[i % frames.len()];
            i = i.wrapping_add(1);

            let edit = EditMessage::new().embed(build_embed(frame));

            if let Err(err) = channel_id.edit_message(&http, message_id, edit).await {
                eprintln!("Error editando mensaje: {err:?}");
                break;
            }
        }
    });

    let mut slot = live_slot().lock().await;
    *slot = Some(LiveMessage {
        channel_id,
        message_id,
        handle,
    });

    Ok(message_id)
}