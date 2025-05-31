//  Slash command basic structure

use std::{future::Future, pin::Pin};

use serenity::{
    all::{CommandInteraction, Context},
    builder::CreateCommand,
};

// Boxed async result 
pub type CommandFuture<'a> =
    Pin<Box<dyn Future<Output = serenity::Result<()>> + Send + 'a>>;

// Slash command trait    
pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;

    fn register(&self) -> CreateCommand;

    fn run<'a>(&'a self, ctx: &'a Context, itx: &'a CommandInteraction) -> CommandFuture<'a>;
}
