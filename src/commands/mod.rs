pub mod admin;
pub mod utils;
pub mod voice;
pub mod welcome;
pub mod audio;


use serenity::all::{CommandInteraction, Context, CreateCommand};

pub fn get_all_commands() -> Vec<CreateCommand> {
    vec![
        audio::register_join(),
        audio::register_stop(),
        audio::register_leave(),
    ]
}

pub async fn dispatch_interaction(ctx: &Context, command: &CommandInteraction) {
    match command.data.name.as_str() {
        "join" => audio::run_join_and_play(ctx, command).await,
        "stop" => audio::run_stop(ctx, command).await,
        "leave" => audio::run_leave(ctx, command).await,
        _ => {}
    }
}