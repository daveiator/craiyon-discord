#[macro_use] extern crate log;

mod commands;
mod custom;
mod craiyon;
mod image_formatter;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use std::fs;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::commands::change_prefix::*;
use crate::commands::ai::*;
use crate::commands::test::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(prefix, ai, amogus)]
struct General;

#[tokio::main]
async fn main() {

    fs::create_dir_all("./data").unwrap();
    fs::create_dir_all("./temp").unwrap();

    //TODO (Someday): Add a logger

    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework =
        StandardFramework::new().configure(|c|  {
            c.owners(owners);
            c.dynamic_prefix(|_, msg| {
                Box::pin(
                    async move { Some(
                        if let Ok(prefix) = custom::get_prefix(match msg.guild_id {
                            Some(id) => {
                                *id.as_u64()
                            },
                            None => {
                                eprintln!("Couldn't get prefix: No guild ID: {:?}", msg.guild_id);
                                0
                            },
                        })
                        {
                            println!("Got prefix: {}", prefix);
                            prefix
                        } else {
                            let default_prefix: String = env::var("DEFAULT_PREFIX").unwrap_or_else(|_| "crai>".to_string());
                            println!("Custom prefix not found. Defaulting to {}", default_prefix);
                            default_prefix
                        }
                        
                    )},
                )
            })
        }).group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
