use serenity::all::{
    CommandInteraction, Context, CreateCommand, EditInteractionResponse,
    GetMessages, MessageId,
};
use crate::commands::shared::base_live_command;

pub fn register_clear_bot(name: &'static str) -> CreateCommand {
    base_live_command(name, "Elimina los mensajes recientes del bot en este canal")
}

pub async fn run_clear_bot(ctx: &Context, command: &CommandInteraction) {
    let _ = command.defer(&ctx.http).await;

    let channel_id = command.channel_id;
    let bot_id = ctx.cache.current_user().id;
    let messages = match channel_id.messages(&ctx.http, GetMessages::new().limit(100)).await {
        Ok(msgs) => msgs,
        Err(err) => {
            eprintln!("Error obteniendo mensajes: {err:?}");
            let _ = command.edit_response(&ctx.http, 
                EditInteractionResponse::new().content("❌ No tengo permiso para leer mensajes en este canal.")
            ).await;
            return;
        }
    };

    let mut bot_message_ids: Vec<MessageId> = messages
        .iter()
        .filter(|m| m.author.id == bot_id)
        .map(|m| m.id)
        .collect();

    if bot_message_ids.is_empty() {
        let _ = command.edit_response(&ctx.http, 
            EditInteractionResponse::new().content("No encontré mensajes míos para borrar.")
        ).await;
        return;
    }
    
    let is_guild = command.guild_id.is_some();
    let mut deleted_count = 0;

    if is_guild {
        let count = bot_message_ids.len();
        if let Err(err) = channel_id.delete_messages(&ctx.http, bot_message_ids).await {
            eprintln!("Error en Bulk Delete: {err:?}");
            let _ = command.edit_response(&ctx.http, 
                EditInteractionResponse::new().content("Hubo un error borrando (quizás son mensajes de más de 14 días).")
            ).await;
            return;
        }
        deleted_count = count;
    } else {
        bot_message_ids.truncate(10);
        for msg_id in &bot_message_ids {
            if channel_id.delete_message(&ctx.http, msg_id).await.is_ok() {
                deleted_count += 1;
            } else {
                break;
            }
        }
    }
    let response_text = if deleted_count > 0 {
        format!("✅ Se eliminaron {} mensajes del bot.", deleted_count)
    } else {
        "No se pudo eliminar ningún mensaje.".to_string()
    };

    let _ = command.edit_response(&ctx.http, 
        EditInteractionResponse::new().content(response_text)
    ).await;
}