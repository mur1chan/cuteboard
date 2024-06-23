use std::collections::HashMap;
use std::io::Read;
use std::fs::File;
use ron::from_str;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub headlines: String,
    pub lists: String,
    pub citations: String,
    pub codes: String,
    pub highlighting: String,
    pub links: String,
    pub tables: String,
    pub inline_codes: String,
}

impl Config {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("headlines".to_string(), self.headlines.clone());
        map.insert("lists".to_string(), self.lists.clone());
        map.insert("citations".to_string(), self.citations.clone());
        map.insert("codes".to_string(), self.codes.clone());
        map.insert("highlighting".to_string(), self.highlighting.clone());
        map.insert("links".to_string(), self.links.clone());
        map.insert("tables".to_string(), self.tables.clone());
        map.insert("inline_codes".to_string(), self.inline_codes.clone());
        map
    }
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open("config.ron")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = from_str(&contents)?;
    let map = config.to_map();
    println!("{:?}", map.get("lists"));
    Ok(config)
}