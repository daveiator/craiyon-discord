use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{Error, ErrorKind};

const DATA_FILE_PATH: &str = "./data/data.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Guild {
    id: u64,
    prefix: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    guilds: Vec<Guild>,
}
impl Config {
    pub fn new() -> Config {
        Config {
            guilds: Vec::new(),
        }
    }
    pub fn from(guilds: Vec<Guild>) -> Config {
        Config {
            guilds,
        }
    } 
}

pub fn get_prefix(guild_id: u64) -> Result<String, Error> {
    println!("Getting prefix for guild: {}", guild_id);
    let config: Config = serde_json::from_str::<Config>(&read_datafile()?)?;
    let guild: Option<&Guild> = config.guilds.iter().find(|g| g.id == guild_id);
    match guild {
        Some(g) => Ok(g.prefix.to_string()),
        None => Err(Error::new(ErrorKind::InvalidData, format!("No prefix found for guild {}", guild_id))),
    }
}

pub fn set_prefix(guild_id: u64, prefix: String) -> Result<(), Error> {
    let config = match serde_json::from_str::<Config>(&read_datafile()?) {
        Ok(c) => c,
        Err(e) => {
            println!("Couldn't read datafile: {}", e);
            println!("Creating new data-file");
            Config::new()
        },
    };
    println!("Config: {:?}", config);
    let mut guilds = config.guilds;
    let guild: Option<&mut Guild> = guilds.iter_mut().find(|g| g.id == guild_id);
    match guild {
        Some(g) => {
            g.prefix = prefix;
        }
        None => {
            // Add the guild to the config
            guilds.push(Guild {
                id: guild_id,
                prefix,
            });
        }
    }
    let mut file = fs::File::create(DATA_FILE_PATH)?;
    serde_json::to_writer_pretty(&mut file, &Config::from(guilds))?;
    Ok(())
}
fn read_datafile() -> Result<String, std::io::Error> {
    match fs::read_to_string(DATA_FILE_PATH) {
        Ok(string) => Ok(string),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match fs::File::create(DATA_FILE_PATH) {
                Ok(_) => Ok("".to_string()),
                Err(error_2) => Err(error_2),
            },
            _ => Err(error),
        },
    }
}