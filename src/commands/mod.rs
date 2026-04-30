pub mod admin;
pub mod status;
pub mod welcome;
pub mod audio;
pub mod embed_animated;
pub mod shared;

use serenity::all::{CommandInteraction, Context, CreateCommand};

pub fn get_all_commands() -> Vec<CreateCommand> {
    vec![
        audio::register_join(),
        audio::register_stop(),
        audio::register_leave(),
        status::register_status(),
        embed_animated::register_animated_embed(),
    ]
}

pub async fn dispatch_interaction(ctx: &Context, command: &CommandInteraction) {
    match command.data.name.as_str() {
        "join" => audio::run_join_and_play(ctx, command).await,
        "stop" => audio::run_stop(ctx, command).await,
        "leave" => audio::run_leave(ctx, command).await,
        "status" => status::run_slash_status(ctx, command).await,
        "animated_embed" => embed_animated::run_slash_animated_embed(ctx, command).await,
        _ => {}
    }
}