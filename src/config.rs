use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum DeviceType {
    engine,
    climatecontrol,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub r#type: DeviceType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub devices: Vec<Device>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Broker {
    pub url: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub nodes: Vec<Node>,
    pub group: String,
    pub broker: Broker,
}

fn load_yaml<T>(path: &String) -> T
where
    T: serde::de::DeserializeOwned,
{
    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    serde_yaml::from_str(&contents).unwrap()
}

pub fn load_config() -> Config {
    let path = std::env::var("CONFIG_FILE").unwrap_or(String::from("./simulator.yaml"));
    load_yaml(&path)
}
