use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption, InstallationContext, InteractionContext
};

pub fn register_live_command(name: &'static str, description: &'static str) -> CreateCommand {
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
                CommandOptionType::Boolean,
                "stop",
                "Detiene la actualización activa",
            )
            .required(false),
        )
}

pub fn opt_bool(command: &CommandInteraction, name: &str) -> bool {
    command
        .data
        .options
        .iter()
        .any(|opt| opt.name == name && matches!(opt.value, CommandDataOptionValue::Boolean(true)))
}