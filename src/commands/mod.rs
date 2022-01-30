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
