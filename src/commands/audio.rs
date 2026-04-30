use serenity::all::{
    CommandInteraction, Context, CreateCommand,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};
use songbird::input::File;

use crate::workspace::paths::WORKSPACE;

pub fn register_join(name: &str) -> CreateCommand {
    CreateCommand::new(name)
        .description("El bot se une a tu canal de voz y reproduce un audio")
}

pub fn register_stop(name: &str) -> CreateCommand {
    CreateCommand::new(name)
        .description("Detiene la reproducción y limpia la cola")
}

pub fn register_leave(name: &str) -> CreateCommand {
    CreateCommand::new(name)
        .description("Saca al bot del canal de voz")
}

async fn reply(ctx: &Context, command: &CommandInteraction, content: impl Into<String>) {
    let _ = command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(content.into()),
            ),
        )
        .await;
}

fn user_voice_channel_id(ctx: &Context, command: &CommandInteraction) -> Option<serenity::all::ChannelId> {
    let guild_id = command.guild_id?;
    let guild = ctx.cache.guild(guild_id)?;
    guild
        .voice_states
        .get(&command.user.id)
        .and_then(|vs| vs.channel_id)
}

pub async fn run_join_and_play(ctx: &Context, command: &CommandInteraction) {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return,
    };

    let connect_to = match user_voice_channel_id(ctx, command) {
        Some(id) => id,
        None => {
            reply(ctx, command, "¡Debes estar en un canal de voz!").await;
            return;
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird no inicializado")
        .clone();

    let handler_lock = match manager.join(guild_id, connect_to).await {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Falló join: {:?}", e);
            reply(ctx, command, "No pude unirme al canal de voz.").await;
            return;
        }
    };

    let ws = WORKSPACE.get().unwrap();
    let audio_path = match ws.get_audio_file() {
        Some(p) => p,
        None => {
            reply(ctx, command, "No se encontró el archivo de audio.").await;
            return;
        }
    };

    let input = File::new(audio_path).into();

    let mut handler = handler_lock.lock().await;
    let _track = handler.enqueue_input(input).await;

    reply(ctx, command, "Audio encolado.").await;
}

pub async fn run_stop(ctx: &Context, command: &CommandInteraction) {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return,
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird no inicializado")
        .clone();

    match manager.get(guild_id) {
        Some(call_lock) => {
            let call = call_lock.lock().await;
            call.queue().stop();
            reply(ctx, command, "Reproducción detenida y cola limpiada.").await;
        }
        None => {
            reply(ctx, command, "No estoy en un canal de voz.").await;
        }
    }
}

pub async fn run_leave(ctx: &Context, command: &CommandInteraction) {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return,
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird no inicializado")
        .clone();

    match manager.remove(guild_id).await {
        Ok(_) => reply(ctx, command, "Salí del canal de voz.").await,
        Err(e) => {
            eprintln!("Error al salir del canal: {:?}", e);
            reply(ctx, command, "No pude salir del canal de voz.").await;
        }
    }
}