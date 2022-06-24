use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::CommandError;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::custom;

#[command]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    match custom::set_prefix(*msg.guild_id.unwrap().as_u64(), msg.content.split_whitespace().nth(1).unwrap().to_string()) {
        Ok(_) => {
            msg.channel_id.say(&ctx.http, "Prefix set!").await?;
            Ok(())
        },
        Err(e) => {
            msg.channel_id.say(&ctx.http, e).await?;
            Err(CommandError::from(e))
        }
    }
}
