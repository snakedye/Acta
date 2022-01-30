use super::*;

use serenity::model::channel::{
    GuildChannel,
    ChannelCategory,
    ChannelType,
};

#[command]
#[only_in(guilds)]
#[usage("record [voice-channel]")]
pub async fn record(ctx: &client::Context, msg: &Message, mut args: Args) -> CommandResult {
    if let Some(guild) = msg.guild(ctx).await {
        let voice_channel_id = if let Ok(channel_name) = args.single_quoted::<String>() {
            if let Some(channel_id) = guild.channel_id_from_name(ctx, channel_name).await {
                if let Some(channel) = guild.channels.get(&channel_id) {
                    match channel.kind {
                        ChannelType::Voice => {
                            // J'imagine deplacer le bot dans le channel et par la suite demarrer l'enregistrement
                            msg.reply_mention(
                                ctx,
                                "L'enregistrement a demarre!"
                            );
                        }
                        _ => {}
                    }
                }
            }
        };
    }

    Ok(())
}
