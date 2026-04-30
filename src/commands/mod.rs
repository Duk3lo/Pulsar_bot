pub mod admin;
pub mod status;
pub mod welcome;
pub mod audio;
pub mod animated_embed;
pub mod shared;
pub mod stop_embed;

use serenity::all::{CommandInteraction, Context, CreateCommand};
use crate::commands::shared::*;

pub fn get_all_commands() -> Vec<CreateCommand> {
    vec![
        audio::register_join(CMD_JOIN),
        audio::register_stop(CMD_STOP),
        audio::register_leave(CMD_LEAVE),
        status::register_status(CMD_STATUS),
        animated_embed::register_animated_embed(CMD_ANIMATED),
        stop_embed::register_live_stop(CMD_LIVE_STOP),
        admin::register_clear_bot(CMD_CLEAR_BOT),
        welcome::register_welcome(CMD_WELCOME),
    ]
}

pub async fn dispatch_interaction(ctx: &Context, command: &CommandInteraction) {
    let cmd_name = command.data.name.as_str();

    match cmd_name {
        CMD_JOIN => audio::run_join_and_play(ctx, command).await,
        CMD_STOP => audio::run_stop(ctx, command).await,
        CMD_LEAVE => audio::run_leave(ctx, command).await,
        CMD_STATUS => status::run_slash_status(ctx, command).await,
        CMD_ANIMATED => animated_embed::run_slash_animated_embed(ctx, command).await,
        CMD_LIVE_STOP => stop_embed::run_live_stop(ctx, command).await,
        CMD_CLEAR_BOT => admin::run_clear_bot(ctx, command).await,
        CMD_WELCOME => welcome::run(ctx, command).await,
        _ => eprintln!("Comando no reconocido: {}", cmd_name),
    }
}