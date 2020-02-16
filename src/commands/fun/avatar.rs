use crate::utils::parse::parse_user;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[min_args(0)]
#[max_args(1)]
fn avatar(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id;
    if args.len() == 1 {
        user_id = match parse_user(
            &args.quoted().current().unwrap().to_string(),
            msg.guild_id.as_ref(),
            Some(&ctx),
        ) {
            Some(i) => i,
            None => msg.author.id,
        };
    } else {
        user_id = msg.author.id;
    }

    let user = user_id.to_user(&ctx)?;
    let avatar = user.face();

    let _ = msg.channel_id.send_message(&ctx.http, |message| {
        message.content(format!("{} avatar", user.tag()));
        message.add_file(&avatar[..]);
        message
    });

    Ok(())
}
