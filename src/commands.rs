//! Command registration and dispatch

pub mod command;
// commands
pub mod ping;

use command::Command;
use serenity::{builder::Builder, http::Http, model::id::GuildId};
use tracing::error;


static COMMANDS: &[&dyn Command] = &[
    &ping::PING,
];

pub async fn register_global_commands(http: &Http) -> serenity::Result<()> {
    for cmd in COMMANDS {
        cmd.register().execute(http, (None, None)).await?;
    }
    Ok(())
}

pub async fn register_guild_commands(
    http: &Http,
    guild: GuildId,
) -> serenity::Result<()> {
    for cmd in COMMANDS {
        cmd.register().execute(http, (Some(guild), None)).await?;
    }
    Ok(())
}

pub async fn dispatch(ctx: &serenity::all::Context, itx: &serenity::all::CommandInteraction) {
    if let Some(cmd) = COMMANDS
        .iter()
        .find(|c| c.name() == itx.data.name.as_str())
    {
        if let Err(e) = cmd.run(ctx, itx).await {
            error!(?e, name = %cmd.name(), "slash‑command failed");
        }
    }
}
