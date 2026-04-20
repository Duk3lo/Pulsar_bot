use serenity::all::User;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};

pub fn welcome_embed(user: &User) -> CreateEmbed {
    CreateEmbed::default()
        .title("¡Bienvenido al sistema!")
        .description(format!("Hola {}, la tripulación te estaba esperando. 🚀", user.name))
        .color(0x00FF00)
        .thumbnail(user.face())
        .footer(CreateEmbedFooter::new("Protocolo de bienvenida v1.0"))
}

pub fn info_embed() -> CreateEmbed {
    CreateEmbed::default()
        .title("🛰️ Estado de la Estación")
        .description("Sistemas operativos al 100%. No se detectan anomalías.")
        .color(0x00FFFF)
        .field("Motor", "Rust 🦀", true)
        .field("Latencia", "Nominal", true)
        .footer(CreateEmbedFooter::new("Core-Health Monitoring"))
}