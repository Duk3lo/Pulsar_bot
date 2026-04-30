use std::time::Duration;

use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::builder::CreateEmbed;

use crate::commands::shared::opt_bool;
use crate::{
    commands::{shared::register_live_command},
    paths::{load_frames_from_file, Workspace, WORKSPACE},
    task::embed_updater::{self as live_status, LiveEmbedRenderer},
};

pub struct AnimatedRenderer;

impl LiveEmbedRenderer for AnimatedRenderer {
    fn render(&self, frame: &str) -> CreateEmbed {
        CreateEmbed::new()
            .title("Embed animado")
            .description(format!("```\n{frame}\n```"))
    }
}

fn load_animation_frames() -> Vec<String> {
    let ws = WORKSPACE.get_or_init(|| {
        Workspace::load_workspace().expect("No se pudo cargar el workspace")
    });

    let anim_path = ws.get_animation_file("animated_embed.txt");

    load_frames_from_file(&anim_path).unwrap_or_else(|err| {
        eprintln!("No se pudo leer la animación: {err:?}");
        vec!["[■□□□]".to_string()]
    })
}

pub fn register_animated_embed() -> CreateCommand {
    register_live_command("animated_embed", "Embed animado")
}

pub async fn run_slash_animated_embed(ctx: &Context, command: &CommandInteraction) {
    let scope = live_status::LiveScope::from_command(command);
    let stop = opt_bool(command, "stop");
    let update = opt_bool(command, "update");

    if stop {
        let stopped = live_status::stop(scope).await;

        let content = if stopped {
            "Actualización detenida."
        } else {
            "No había una actualización activa."
        };

        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);

        if let Err(err) = command.create_response(&ctx.http, builder).await {
            eprintln!("Error respondiendo stop: {err:?}");
        }
        return;
    }

    let frames = load_animation_frames();
    let renderer = AnimatedRenderer;
    let first_frame = frames.first().map(|s| s.as_str()).unwrap_or("[■□□□]");

    let data = CreateInteractionResponseMessage::new().add_embed(renderer.render(first_frame));
    let builder = CreateInteractionResponse::Message(data);

    if let Err(err) = command.create_response(&ctx.http, builder).await {
        eprintln!("Error respondiendo animated_embed: {err:?}");
        return;
    }

    if update {
        if let Err(err) = live_status::start(
            ctx.clone(),
            scope,
            command.user.id,
            command.token.clone(),
            Duration::from_secs(2),
            frames,
            AnimatedRenderer,
        )
        .await
        {
            eprintln!("{err}");
        }
    }
}