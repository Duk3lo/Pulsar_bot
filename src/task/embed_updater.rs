use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        OnceLock,
    },
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
    id: u64,
    owner: UserId,
    handle: JoinHandle<()>,
}

static ACTIVE_BY_SCOPE: OnceLock<Mutex<HashMap<LiveScope, ActiveLive>>> = OnceLock::new();
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn active_map() -> &'static Mutex<HashMap<LiveScope, ActiveLive>> {
    ACTIVE_BY_SCOPE.get_or_init(|| Mutex::new(HashMap::new()))
}

async fn cleanup_if_same(scope: LiveScope, id: u64) {
    let mut map = active_map().lock().await;

    if map.get(&scope).map(|active| active.id == id).unwrap_or(false) {
        map.remove(&scope);
    }
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
    frames: std::sync::Arc<[String]>,
    start_from: usize,
    repeat: bool,
    renderer: R,
) -> Result<(), &'static str>
where
    R: LiveEmbedRenderer,
{
    if frames.is_empty() {
        return Err("la animación no tiene frames");
    }

    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let frame_count = frames.len();
    let start_index = start_from % frame_count;

    let mut map = active_map().lock().await;

    if let Some(active) = map.get(&scope) {
        if active.owner == owner {
            return Err("ya tienes una actualización activa aquí");
        }
    }

    if let Some(active) = map.remove(&scope) {
        active.handle.abort();
    }

    let frames_for_task = frames.clone();

    let handle = tokio::spawn(async move {
        let mut index = start_index;
        let mut tick = interval(every);

        loop {
            if !repeat && index >= frame_count {
                break;
            }

            tick.tick().await;

            let frame = &frames_for_task[index % frame_count];
            index = index.wrapping_add(1);

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

        cleanup_if_same(scope, id).await;
    });

    map.insert(scope, ActiveLive { id, owner, handle });
    Ok(())
}