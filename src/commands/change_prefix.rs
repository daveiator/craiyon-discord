use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::CommandError;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::database::*;

#[command]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    info!("Changing prefix!");
    match msg.content.split_whitespace().nth(1) {
        Some(prefix) => {
            match set_prefix(*msg.guild_id.unwrap().as_u64(), prefix.to_string()) {
                Ok(_) => {
                    info!("Prefix changed to {}", prefix);
                    msg.channel_id.say(&ctx.http, "Prefix set!").await?;
                    Ok(())
                },
                Err(e) => {
                    error!("Couldn't set prefix: {}", e);
                    msg.channel_id.say(&ctx.http, format!("Couldn't set prefix due to error: {}", &e)).await?;
                    Err(CommandError::from(e))
                }
            }
        },
        None => {
            msg.channel_id.say(&ctx.http, "Please specify a prefix!").await?;
            warn!("No prefix specified!");
            Err(CommandError::from("Not enough arguments"))
        }
    }
    
}
