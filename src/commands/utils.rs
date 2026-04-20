use serenity::all::{CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage, Message};
use serenity::builder::CreateMessage;
use serenity::prelude::*;
use crate::ui::embeds;

pub async fn run_slash_ping(ctx: Context, command: CommandInteraction) {
    let embed = embeds::info_embed();
    let data = CreateInteractionResponseMessage::new().add_embed(embed);
    let builder = CreateInteractionResponse::Message(data);

    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Error en slash ping: {:?}", why);
    }
}

pub async fn run_manual_ping(ctx: Context, msg: Message) {
    let embed = embeds::info_embed();
    let builder = CreateMessage::new().add_embed(embed);

    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
        println!("Error en manual ping: {:?}", why);
    }
}