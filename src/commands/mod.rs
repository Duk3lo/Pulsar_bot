pub mod admin;
pub mod utils;
pub mod voice;
pub mod welcome;

use serenity::all::{CommandInteraction, Context, CreateCommand};

pub fn get_all_commands() -> Vec<CreateCommand> {
    vec![
        utils::register_ping(),
        voice::register_join(),
        welcome::register(),
    ]
}
pub async fn dispatch_interaction(ctx: &Context, command: &CommandInteraction) {
    match command.data.name.as_str() {
        "ping" => utils::run_slash_ping(&ctx, &command).await,
        "join" => voice::run_join_and_play(&ctx, &command).await,
        "welcome" => welcome::run(&ctx, &command).await,
        _ => println!("Comando no reconocido: {}", &command.data.name),
    }
}