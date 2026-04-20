use serenity::model::guild::Member;
use serenity::prelude::*;
use crate::ui::embeds;
use serenity::builder::CreateMessage;
use serenity::model::id::ChannelId;

pub async fn handle_welcome(ctx: Context, new_member: Member) {
    let channel_id = ChannelId::new(1495418149213048905);
    let embed = embeds::welcome_embed(&new_member.user);
    let builder = CreateMessage::default().add_embed(embed);
    if let Err(why) = channel_id.send_message(&ctx.http, builder).await {
        println!("Error en bienvenida: {:?}", why);
    }
}