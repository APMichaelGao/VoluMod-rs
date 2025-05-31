use serenity::{
    all::{CommandInteraction, Context},
    builder::{
        CreateCommand,
        CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
};

use crate::commands::command::{Command, CommandFuture};

pub struct Ping;

pub const PING: Ping = Ping;

impl Command for Ping {
    fn name(&self) -> &'static str { "ping" }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("Replies with Pong!")
    }

    fn run<'a>(
        &'a self,
        ctx: &'a Context,
        itx: &'a CommandInteraction,
    ) -> CommandFuture<'a> {
        Box::pin(async move {
            itx.create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("🏓 Pong!"),
                ),
            )
            .await
        })
    }
}
