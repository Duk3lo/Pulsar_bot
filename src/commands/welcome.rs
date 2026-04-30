use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::guild::Member;
use serenity::model::id::ChannelId;
use crate::ui::embeds;
use serenity::builder::CreateMessage;

pub fn register_welcome(name: &'static str) -> CreateCommand {
    CreateCommand::new(name).description("Ejecuta manualmente la bienvenida para un miembro")
}
pub async fn send_welcome_embed(ctx: &Context, member: &Member) {
    let channel_id = ChannelId::new(1495418149213048905);
    let embed = embeds::welcome_embed(&member.user);
    let builder = CreateMessage::default().add_embed(embed);
    if let Err(why) = channel_id.send_message(&ctx.http, builder).await {
        println!("❌ Error enviando mensaje de bienvenida: {:?}", why);
    }
}

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    if let Some(member) = command.member.clone() {
        let response = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("✅ Protocolo de bienvenida ejecutado.")
                .ephemeral(true),
        );

        if let Err(why) = command.create_response(&ctx.http, response).await {
            println!("Error al responder interacción welcome: {:?}", why);
        }
        send_welcome_embed(&ctx, &member).await;
    }
}