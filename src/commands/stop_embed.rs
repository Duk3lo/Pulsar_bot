use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use crate::{
    commands::shared::base_live_command,
    task::embed_updater as live_status,
};

pub fn register_live_stop(name: &'static str) -> CreateCommand {
    base_live_command(name, "Detiene cualquier actualización activa")
}

pub async fn run_live_stop(ctx: &Context, command: &CommandInteraction) {
    let scope = live_status::LiveScope::from_command(command);
    let stopped = live_status::stop(scope).await;

    let content = if stopped {
        "Actualización detenida."
    } else {
        "No había una actualización activa."
    };

    let data = CreateInteractionResponseMessage::new().content(content);
    let builder = CreateInteractionResponse::Message(data);

    if let Err(err) = command.create_response(&ctx.http, builder).await {
        eprintln!("Error respondiendo live_stop: {err:?}");
    }
}