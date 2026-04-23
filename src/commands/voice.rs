use serenity::all::{CommandInteraction, Context, CreateCommand};
use songbird::input::File;

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
    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        let mut handler = handler_lock.lock().await;


        let source = File::new("sonido.wav");
        handler.play_input(source.into());
        let _ = command.create_response(&ctx.http,
            serenity::all::CreateInteractionResponse::Message(
                serenity::all::CreateInteractionResponseMessage::new().content("🔊 ¡Reproduciendo audio!")
            )
        ).await;
    }
}

