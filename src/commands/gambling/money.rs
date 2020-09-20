use crate::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[min_args(0)]
#[max_args(1)]
#[only_in(guilds)]
#[description("Retrieves how much money a user has.")]
#[usage("money <optional: person>")]
#[example("money Elinvynia")]
async fn money(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.ok_or(BotError::NoneError)?;
    let user_id;
    if !args.is_empty() && msg.guild_id.is_some() {
        let name: String = args.single()?;
        let gid = msg.guild_id.ok_or(BotError::NoneError)?;

        match parse_user(&name, Some(&gid), Some(&ctx)).await {
            Some(uid) => user_id = uid,
            None => return Ok(()),
        };
    } else {
        user_id = msg.author.id
    }

    let money = get_user_money(guild_id, user_id).await?;
    let member = guild_id.member(&ctx, user_id).await?;

    let message = format!("**{}** has **{}**", member.display_name(), money);

    msg.channel_id.say(&ctx, message).await?;

    Ok(())
}