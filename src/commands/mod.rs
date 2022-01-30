pub mod record;

pub use serenity::prelude::EventHandler;

pub use std::sync::{
    Arc, atomic::AtomicBool, atomic::Ordering
};
pub use serenity::{
    client,
    async_trait,
    framework::standard::{
        help_commands,
        macros::{command, group, help},
        Args, CommandGroup, CommandOptions, CommandResult, HelpOptions, CommandError
    },
    model::prelude::*,
};

pub async fn name_to_channel_id(
    ctx: &client::Context,
    guild: &Guild,
    msg: &Message,
    name: &str
) -> Option<UserId> {
    name.parse::<UserId>()
    .ok()
    .filter(|id| id.0 > 10_000_000_000_000_000)
}
