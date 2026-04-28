pub mod admin;
pub mod status;
pub mod welcome;
pub mod audio;
pub mod message;

use serenity::all::{CommandInteraction, Context, CreateCommand};

pub fn get_all_commands() -> Vec<CreateCommand> {
    vec![
        audio::register_join(),
        audio::register_stop(),
        audio::register_leave(),
        status::register_status(),
        message::register_message(),
    ]
}

pub async fn dispatch_interaction(ctx: &Context, command: &CommandInteraction) {
    match command.data.name.as_str() {
        "join" => audio::run_join_and_play(ctx, command).await,
        "stop" => audio::run_stop(ctx, command).await,
        "leave" => audio::run_leave(ctx, command).await,
        "status" => status::run_slash_status(ctx, command).await,
        //"message" => message::run_message(ctx, command).await,
        _ => {}
    }
}