use serde::{Serialize, Deserialize};
use std::fs::{File};
use std::io::ErrorKind;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Guild {
    pub id: u64,
    pub prefix: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    pub guilds: Vec<Guild>,
}

pub fn get_prefix(guild_id: u64) -> Result<String, String> {
    let config: Config = serde_json::from_reader(read_datafile().unwrap()).unwrap();
    let guild: Option<&Guild> = config.guilds.iter().find(|g| g.id == guild_id);
    match guild {
        Some(g) => Ok(g.prefix.to_string()),
        None => Err(format!("No prefix found for guild {}", guild_id)),
    }
}

pub fn set_prefix(guild_id: u64, prefix: String) -> Result<(), String> {
    let config: Config = serde_json::from_reader(read_datafile().unwrap()).unwrap();
    let mut guilds = config.guilds.clone();
    let guild: Option<&mut Guild> = guilds.iter_mut().find(|g| g.id == guild_id);
    match guild {
        Some(g) => {
            g.prefix = prefix;
            let mut file = File::create("./data/config.json").unwrap();
            serde_json::to_writer_pretty(&mut file, &guilds).unwrap();
            Ok(())

        }
    }
}
fn read_datafile() -> Result<File, String> {
    let path = "data/data.json";
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => Ok(fc),
                Err(e) => Err(format!("Problem creating the file: {:?}", e)),
            },
        },
    }
}