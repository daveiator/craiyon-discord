use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::CommandError;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::custom;

#[command]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    match msg.content.split_whitespace().nth(1) {
        Some(prefix) => {
            match custom::set_prefix(*msg.guild_id.unwrap().as_u64(), prefix.to_string()) {
                Ok(_) => {
                    msg.channel_id.say(&ctx.http, "Prefix set!").await?;
                    Ok(())
                },
                Err(e) => {
                    msg.channel_id.say(&ctx.http, format!("Couldn't set prefix due to error: {}", &e)).await?;
                    eprintln!("Couldn't set prefix: {}", e);
                    Err(CommandError::from(e))
                }
            }
        },
        None => {
            msg.channel_id.say(&ctx.http, "Please specify a prefix!").await?;
            eprintln!("Couldn't set prefix: Not enough arguments");
            Err(CommandError::from("Not enough arguments"))
        }
    }
    
}
