use serenity::all::User;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};

use crate::status::metrics::{StatusSnapshot, format_bytes};



pub fn welcome_embed(user: &User) -> CreateEmbed {
    CreateEmbed::default()
        .title("¡Bienvenido al sistema!")
        .description(format!("Hola {}, la tripulación te estaba esperando. 🚀", user.name))
        .color(0x00FF00)
        .thumbnail(user.face())
        .footer(CreateEmbedFooter::new("Protocolo de bienvenida v1.0"))
}

pub fn info_embed(frame: &str, status: &StatusSnapshot) -> CreateEmbed {
    CreateEmbed::default()
        .title("🛰️ Estado de la Estación")
        .description(format!("Sistemas operativos al 100% {}", frame))
        .color(0x00FFFF)
        .field("CPU", format!("{:.1}%", status.cpu), true)
        .field(
            "RAM del sistema",
            format!("{} / {}", format_bytes(status.used_ram), format_bytes(status.total_ram)),
            true,
        )
        .field("RAM del bot", format_bytes(status.bot_ram), true)
        .field("Motor", "Rust 🦀", true)
        .field("Latencia", "Nominal", true)
        .footer(CreateEmbedFooter::new("Core-Health Monitoring"))
}