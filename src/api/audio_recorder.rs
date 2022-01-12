use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;

pub struct AudioRecorder;

impl AudioRecorder{
    pub async fn record(context: &Context, message: &Message) -> CommandResult{
        message.reply(context, "Recording!").await?;
        Ok(())
    } 
}

