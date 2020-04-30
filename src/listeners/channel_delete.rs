use crate::data::db::LogType;
use crate::db::log::{get_log_channel, get_log_type};
use log::error;
use serenity::{model::prelude::*, prelude::*};

pub async fn channel_delete(ctx: Context, channel: &GuildChannel) {
    let guildid = channel.guild_id;

    let log_channel = match get_log_channel(&ctx, guildid).await {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    let log_type = match get_log_type(&ctx, guildid).await {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    if log_type & LogType::ChannelDeleted as i64 != LogType::ChannelDeleted as i64 {
        return;
    }

    if let Err(e) = log_channel
        .say(&ctx.http, format!("Channel deleted: {}", channel.name))
        .await
    {
        error!("{:?}", e);
    }
}
