use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, CreateCommand,
    CreateCommandOption, InstallationContext, InteractionContext,
};

pub const CMD_JOIN: &str = "join";
pub const CMD_STOP: &str = "stop";
pub const CMD_LEAVE: &str = "leave";
pub const CMD_STATUS: &str = "status";
pub const CMD_ANIMATED: &str = "animated_embed";
pub const CMD_LIVE_STOP: &str = "stop_embed";
pub const CMD_CLEAR_BOT: &str = "clear_bot";
pub const CMD_WELCOME: &str = "welcome";

pub fn base_live_command(name: &'static str, description: &'static str) -> CreateCommand {
    CreateCommand::new(name)
        .description(description)
        .integration_types(vec![
            InstallationContext::Guild,
            InstallationContext::User,
        ])
        .contexts(vec![
            InteractionContext::Guild,
            InteractionContext::BotDm,
            InteractionContext::PrivateChannel,
        ])
}

pub fn register_loop_animation_command(name: &'static str, description: &'static str) -> CreateCommand {
    base_live_command(name, description)
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "loop",
                "Repite la animación desde el primer frame",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "duration",
                "Segundos entre cada frame (por defecto: 1)",
            )
            .required(false)
            .min_int_value(1),
        )
}

pub fn register_status_command(name: &'static str) -> CreateCommand {
    base_live_command(name, "Muestra información sobre el bot")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "update",
                "Inicia la actualización automática del embed",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "duration",
                "Segundos entre cada actualización (por defecto: 1)",
            )
            .required(false)
            .min_int_value(1),
        )
}

pub fn register_live_stop_command() -> CreateCommand {
    base_live_command("live_stop", "Detiene cualquier actualización activa")
}

pub fn opt_bool(command: &CommandInteraction, name: &str) -> bool {
    command
        .data
        .options
        .iter()
        .any(|opt| opt.name == name && matches!(opt.value, CommandDataOptionValue::Boolean(true)))
}

pub fn opt_u64(command: &CommandInteraction, name: &str) -> Option<u64> {
    command.data.options.iter().find(|opt| opt.name == name).and_then(|opt| {
        if let CommandDataOptionValue::Integer(val) = opt.value {
            Some(val as u64)
        } else {
            None
        }
    })
}