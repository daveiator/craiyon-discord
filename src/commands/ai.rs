use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::CommandError;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::custom;

#[command]
async fn ai(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}
