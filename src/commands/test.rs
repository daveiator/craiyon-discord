use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;

use serenity::model::prelude::*;
use serenity::prelude::*;

use serenity::model::channel::ReactionType;




#[command]
async fn amogus(ctx: &Context, msg: &Message) -> CommandResult {
    msg.react(&ctx.http, ReactionType::try_from("<:amogus:834731006761369640>")?).await?;
    
    Ok(())
}
