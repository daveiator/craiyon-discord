mod commands;
mod database;
mod craiyon;
mod image_formatter;

#[macro_use] extern crate log;
use simplelog as slog;

use std::collections::HashSet;
use std::fs;
use std::env;
use std::sync::Arc;


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
#[commands(prefix, ai)]
struct General;

#[tokio::main]
async fn main() {
    // Setup logger
    if let Err(why) = slog::TermLogger::init(
        slog::LevelFilter::Info,
        slog::Config::default(),
        slog::TerminalMode::Mixed,
        slog::ColorChoice::Auto) 
    {
        panic!("Failed to initialize logger: {}", why);
    }
    info!("Logger stated!");

    //Setup directories
    info!("Setting up directories...");
    fs::create_dir_all("./data").unwrap();
    fs::create_dir_all("./temp").unwrap();


    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    info!("Loading environment variables...");
    if let Err(why) = dotenv::dotenv() {
        error!("Error loading .env file: {}", why);
        warn!("Using default environment variables...");
    }

    // Getting the token
    info!("Loading Discord token...");
    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(why) => {
            error!("Error loading DISCORD_TOKEN: {}", why);
            panic!("No token found! Can't continue!");
        }
    };

    // Create http context
    info!("Creating http context...");
    let http = Http::new(&token);

    // We will fetch your bot's owners and id
    info!("Fetching application info...");
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => {
            error!("Error loading application info: {:?}", why);
            panic!("Could not access application info: {:?}", why);
        },
    };

    // Create the framework
    info!("Creating Discord framework...");
    let framework =
        StandardFramework::new().configure(|c|  {
            c.owners(owners);
            c.dynamic_prefix(|_, msg| {
                Box::pin(
                    async move { Some(
                        if let Ok(prefix) = database::get_prefix(match msg.guild_id {
                            Some(id) => {
                                *id.as_u64()
                            },
                            None => {
                                error!("Couldn't get prefix: No guild ID: {:?}", msg.guild_id);
                                0
                            },
                        })
                        {
                            info!("Got prefix: {}", prefix);
                            prefix
                        } else {
                            let default_prefix: String = match env::var("DEFAULT_PREFIX") {
                                Ok(prefix) => prefix,
                                Err(why) => {
                                    warn!("Error loading DEFAULT_PREFIX: {}", why);
                                    warn!("Using default prefix: {}", "crai>");
                                    "crai>".to_string()
                                },
                            };
                            info!("No custom prefix not found for guild. Defaulting to {}", default_prefix);
                            default_prefix
                        }
                        
                    )},
                )
            })
        }).group(&GENERAL_GROUP);

    // Create the client.
    info!("Creating Discord client...");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = match Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await {
            Ok(client) => client,
            Err(why) => {
                error!("Error creating client: {:?}", why);
                panic!("Could not create client: {:?}", why);
            }
        };
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        if let Err(why) = tokio::signal::ctrl_c().await {
            error!("Couldn't register CTRL-C handler: {:?}", why);
            panic!("Couldn't register CTRL-C handler: {:?}", why);
        }
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
        panic!("Client error: {:?}", why);
    }
}
