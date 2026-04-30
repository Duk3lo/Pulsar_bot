use std::{
    collections::HashMap,
    sync::OnceLock,
    time::Duration,
};

use serenity::{
    all::{ChannelId, CommandInteraction, GuildId, UserId},
    builder::{CreateEmbed, EditInteractionResponse},
    prelude::Context,
};
use tokio::{sync::Mutex, task::JoinHandle, time::interval};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiveScope {
    Guild(GuildId),
    Channel(ChannelId),
}

impl LiveScope {
    pub fn from_command(command: &CommandInteraction) -> Self {
        match command.guild_id {
            Some(guild_id) => LiveScope::Guild(guild_id),
            None => LiveScope::Channel(command.channel_id),
        }
    }
}

pub trait LiveEmbedRenderer: Send + Sync + 'static {
    fn render(&self, frame: &str) -> CreateEmbed;
}

struct ActiveLive {
    owner: UserId,
    handle: JoinHandle<()>,
}

static ACTIVE_BY_SCOPE: OnceLock<Mutex<HashMap<LiveScope, ActiveLive>>> = OnceLock::new();

fn active_map() -> &'static Mutex<HashMap<LiveScope, ActiveLive>> {
    ACTIVE_BY_SCOPE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub async fn stop(scope: LiveScope) -> bool {
    let mut map = active_map().lock().await;

    if let Some(active) = map.remove(&scope) {
        active.handle.abort();
        true
    } else {
        false
    }
}

pub async fn start<R>(
    ctx: Context,
    scope: LiveScope,
    owner: UserId,
    token: String,
    every: Duration,
    frames: Vec<String>,
    renderer: R,
) -> Result<(), &'static str>
where
    R: LiveEmbedRenderer,
{
    if frames.is_empty() {
        return Err("la animación no tiene frames");
    }

    let mut map = active_map().lock().await;

    if let Some(active) = map.get(&scope) {
        if active.owner == owner {
            return Err("ya tienes una actualización activa aquí");
        }
    }

    if let Some(active) = map.remove(&scope) {
        active.handle.abort();
    }

    let frame_count = frames.len();

    let handle = tokio::spawn(async move {
        let mut i = 0usize;
        let mut tick = interval(every);

        loop {
            tick.tick().await;

            let frame = &frames[i % frame_count];
            i = i.wrapping_add(1);

            let embed = renderer.render(frame);
            let edit = EditInteractionResponse::new().embed(embed);

            if let Err(err) = ctx
                .http
                .edit_original_interaction_response(&token, &edit, vec![])
                .await
            {
                eprintln!("Error actualizando respuesta de interacción: {err:?}");
                break;
            }
        }
    });

    map.insert(scope, ActiveLive { owner, handle });
    Ok(())
}