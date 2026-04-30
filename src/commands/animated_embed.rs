use std::time::Duration;

use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::builder::CreateEmbed;

use crate::commands::shared::opt_u64;
use crate::{
    commands::shared::{opt_bool, register_loop_animation_command},
    task::embed_updater::{self as live_status, LiveEmbedRenderer},
    workspace::paths::{load_frames_from_file, Workspace, WORKSPACE},
};

pub struct AnimatedRenderer;

impl LiveEmbedRenderer for AnimatedRenderer {
    fn render(&self, frame: &str) -> CreateEmbed {
        CreateEmbed::new()
            .title("Uffas")
            .description(format!("```\n{frame}\n```"))
    }
}

fn load_animation_frames() -> Vec<String> {
    let ws = WORKSPACE.get_or_init(|| {
        Workspace::load_workspace().expect("No se pudo cargar el workspace")
    });

    let anim_path = ws.get_default_animation_file();

    load_frames_from_file(&anim_path).unwrap_or_else(|err| {
        eprintln!("No se pudo leer la animación normal: {err:?}");
        vec!["(sin animación)".to_string()]
    })
}

pub fn register_animated_embed(name: &'static str) -> CreateCommand {
    register_loop_animation_command(name, "Animación normal del embed")
}

pub async fn run_slash_animated_embed(ctx: &Context, command: &CommandInteraction) {
    let scope = live_status::LiveScope::from_command(command);
    let repeat = opt_bool(&command, "loop");
    let duration_secs = opt_u64(command, "duration").unwrap_or(1);

    let frames = load_animation_frames();
    let first_frame = frames.first().map(|s| s.as_str()).unwrap_or("(sin animación)");

    

    let data = CreateInteractionResponseMessage::new()
        .add_embed(AnimatedRenderer.render(first_frame));
    let builder = CreateInteractionResponse::Message(data);

    if let Err(err) = command.create_response(&ctx.http, builder).await {
        eprintln!("Error respondiendo animated_embed: {err:?}");
        return;
    }

    if let Err(err) = live_status::start(
        ctx.clone(),
        scope,
        command.user.id,
        command.token.clone(),
        Duration::from_secs(duration_secs),
        frames,
        1,
        repeat,
        AnimatedRenderer,
    )
    .await
    {
        eprintln!("{err}");
    }
}