use serenity::all::{CommandInteraction, Context, CreateCommand};
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};
use songbird::input::File;

use crate::workspace::paths::WORKSPACE;

pub fn register_join() -> CreateCommand {
    CreateCommand::new("join").description("El bot se une a tu canal de voz y reproduce un audio")
}

pub async fn run_enqueue_audio(ctx: &Context, command: &CommandInteraction) {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return,
    };

    let guild = match ctx.cache.guild(guild_id) {
        Some(g) => g.clone(),
        None => return,
    };

    let channel_id = guild
        .voice_states
        .get(&command.user.id)
        .and_then(|vs| vs.channel_id);

    let connect_to = match channel_id {
        Some(id) => id,
        None => {
            let _ = command.create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("¡Debes estar en un canal de voz!"),
                ),
            ).await;
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
            return;
        }
    };

    let ws = WORKSPACE.get().unwrap();
    let audio_path = match ws.get_audio_file() {
        Some(p) => p,
        None => {
            eprintln!("No se encontró el archivo de audio");
            return;
        }
    };

    let input = File::new(audio_path).into();

    let mut handler = handler_lock.lock().await;
    let _track = handler.enqueue_input(input).await;

    let _ = command.create_response(
        &ctx.http,
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("Audio encolado."),
        ),
    ).await;
}

pub async fn run_stop(ctx: &Context, command: &CommandInteraction) {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return,
    };

    let manager = songbird::get(ctx).await.expect("Songbird no inicializado").clone();

    if let Some(call_lock) = manager.get(guild_id) {
        let call = call_lock.lock().await;
        call.queue().stop();
    }
}

pub async fn run_leave(ctx: &Context, command: &CommandInteraction) {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => return,
    };

    let manager = songbird::get(ctx).await.expect("Songbird no inicializado").clone();
    let _ = manager.remove(guild_id).await;
}

pub async fn run_join_and_play(ctx: &Context, command: &CommandInteraction) {
    let guild_id = match command.guild_id {
        Some(id) => id,
        None => {
            eprintln!("No guild_id");
            return;
        }
    };

    let guild = match ctx.cache.guild(guild_id) {
        Some(g) => g.clone(),
        None => {
            eprintln!("Guild no está en cache");
            return;
        }
    };

    let channel_id = guild
        .voice_states
        .get(&command.user.id)
        .and_then(|vs| vs.channel_id);

    let connect_to = match channel_id {
        Some(id) => id,
        None => {
            eprintln!("Usuario no está en voz");
            let _ = command.create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("¡Debes estar en un canal de voz!"),
                ),
            ).await;
            return;
        }
    };

    println!("Intentando unir a: {:?}", connect_to);

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird no inicializado")
        .clone();

    match manager.join(guild_id, connect_to).await {
        Ok(handler_lock) => {
            println!("Se unió al canal");

            let ws = WORKSPACE.get().unwrap();
            if let Some(audio_path) = ws.get_audio_file() {
                let input = File::new(audio_path).into();
                let mut handler = handler_lock.lock().await;
                handler.play_input(input);
                println!("Audio enviado al canal");
            } else {
                eprintln!("No se encontró el archivo de audio");
            }
        }
        Err(e) => {
            eprintln!("Falló join: {:?}", e);
        }
    }
}