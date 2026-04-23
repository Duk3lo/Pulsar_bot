use serenity::all::{CommandInteraction, Context, CreateCommand};
//use songbird::input::File;

use crate::workspace::paths::Workspace;

pub fn register_join() -> CreateCommand {
    CreateCommand::new("join").description("El bot se une a tu canal de voz")
}

pub async fn run_join_and_play(ctx: &Context, command: &CommandInteraction) {
    let guild_id = command.guild_id.expect("Solo se puede usar en servidores");
    let guild = ctx.cache.guild(guild_id).unwrap().clone();
    let channel_id = guild.voice_states.get(&command.user.id)
        .and_then(|vs| vs.channel_id);
    let connect_to = match channel_id {
        Some(id) => id,
        None => {
            let _ = command.create_response(&ctx.http,
                serenity::all::CreateInteractionResponse::Message(
                    serenity::all::CreateInteractionResponseMessage::new().content("¡Debes estar en un canal de voz!")
                )
            ).await;
            return;
        }
    };
    let manager = songbird::get(ctx).await.expect("Songbird no inicializado").clone();
    //if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {//let mut handler = handler_lock.lock().await; }

    let ws = Workspace::global();
        let audio_file = ws.get_audio_file();
        if let Some(audio_path) = audio_file {
            print!("Reproduciendo audio desde: {:?}", audio_path);
        }else {
            print!("Archivo de audio no encontrado en: {:?}", ws.folder_audio);
        }
}

