use std::{collections::HashMap, sync::OnceLock, time::Duration};

use serenity::{
    all::{ChannelId, GuildId, UserId},
    builder::EditInteractionResponse,
    prelude::Context,
};
use tokio::{sync::Mutex, task::JoinHandle, time::interval};

use crate::{status::metrics::collect_status, ui::embeds};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusScope {
    Guild(GuildId),
    Channel(ChannelId),
}

impl StatusScope {
    pub fn from_command(command: &serenity::all::CommandInteraction) -> Self {
        match command.guild_id {
            Some(guild_id) => StatusScope::Guild(guild_id),
            None => StatusScope::Channel(command.channel_id),
        }
    }
}

struct ActiveStatus {
    owner: UserId,
    handle: JoinHandle<()>,
}

static ACTIVE_BY_SCOPE: OnceLock<Mutex<HashMap<StatusScope, ActiveStatus>>> = OnceLock::new();

fn active_map() -> &'static Mutex<HashMap<StatusScope, ActiveStatus>> {
    ACTIVE_BY_SCOPE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub async fn stop_status(scope: StatusScope) -> bool {
    let mut map = active_map().lock().await;

    if let Some(active) = map.remove(&scope) {
        active.handle.abort();
        true
    } else {
        false
    }
}

pub async fn start_interaction_updater(
    ctx: Context,
    scope: StatusScope,
    owner: UserId,
    token: String,
    every: Duration,
) -> Result<(), &'static str> {
    let mut map = active_map().lock().await;

    if let Some(active) = map.get(&scope) {
        if active.owner == owner {
            return Err("ya tienes una actualización activa aquí");
        }
    }

    if let Some(active) = map.remove(&scope) {
        active.handle.abort();
    }

    let handle = tokio::spawn(async move {
        let frames = [
            "[■□□□]",
            "[■■□□]",
            "[■■■□]",
            "[■■■■]",
            "[□■■■]",
            "[□□■■]",
            "[□□□■]",
        ];
        let mut i = 0usize;
        let mut tick = interval(every);

        loop {
            tick.tick().await;

            let frame = frames[i % frames.len()];
            i = i.wrapping_add(1);

            let status = collect_status();
            let embed = embeds::info_embed(frame, &status);
            let edit = EditInteractionResponse::new().embed(embed);

            if let Err(err) = ctx.http.edit_original_interaction_response(&token, &edit, vec![]).await{
                eprintln!("Error actualizando respuesta de interacción: {err:?}");
                break;
            }
        }
    });

    map.insert(scope, ActiveStatus { owner, handle });
    Ok(())
}
