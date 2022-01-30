extern crate serenity;

mod api;
mod commands;

use std::env;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{
    channel::Message,
    gateway::Ready
};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(record)]
struct AudioRecorder;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Check if the bot is ready to go
    async fn ready(&self, _: Context, ready: Ready){
        println!("{} is connected ", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
                                      .configure(|c| c.prefix("!"))
                                      .group(&AUDIORECORDER_GROUP);
    // Go to .cargo/config.toml and set your discord token value for DISCORD_TOKEN
    let token = env!("DISCORD_TOKEN");
    let mut client = Client::builder(&token)
                            .event_handler(Handler)
                            .framework(framework)
                            .await
                            .expect("Error creating client");
    
    if let Err(error) = client.start().await {
        println!("Error while running the client: {:?} ", error);
    }
}

#[command]
async fn record(ctx: &Context, msg: &Message) -> CommandResult {
    api::AudioRecorder::record(ctx, msg).await
}
