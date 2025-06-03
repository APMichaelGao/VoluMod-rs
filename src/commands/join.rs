use serenity::{
    all::{CommandInteraction, Context},
    builder::{
        CreateCommand, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
};

use crate::{
    audio,
    commands::command::{Command, CommandFuture},
};

pub struct Join;
pub const JOIN: Join = Join;

impl Command for Join {
    fn name(&self) -> &'static str {
        "join"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("join a voice channel")
    }

    fn run<'a>(&'a self, ctx: &'a Context, itx: &'a CommandInteraction) -> CommandFuture<'a> {
        Box::pin(async move {
            let guild_id = itx.guild_id.ok_or(serenity::Error::Other("no guild"))?;
            let user_id = itx.user.id;

            let channel_id = ctx
                .cache
                .guild(guild_id)
                .and_then(|g| g.voice_states.get(&user_id).and_then(|vs| vs.channel_id));

            if let Some(channel) = channel_id {
                audio::join(ctx, guild_id, channel)
                    .await
                    .map_err(|_| serenity::Error::Other("voice join failed"))?;

                itx.create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content("joined"),
                    ),
                )
                .await
            } else {
                itx.create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content("Please join a voice channel first."),
                    ),
                )
                .await
            }
        })
    }
}
