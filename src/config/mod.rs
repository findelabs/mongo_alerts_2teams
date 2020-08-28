use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use http::request::Parts;

use crate::ConfigHash;

pub fn parse(file: &str) -> Result<ConfigHash,serde_yaml::Error> {
    let mut file = File::open(file).expect("Unable to open config");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read config");

    let deck: HashMap<String,String> = serde_yaml::from_str(&contents)?;

    Ok(Arc::new(Mutex::new(deck)))
}

fn params(req: &Parts) -> Option<HashMap<String,String>> {
    let params: HashMap<String, String> = req.uri
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
            })
        .unwrap_or_else(HashMap::new);
    Some(params)
}

pub fn channel(req: &Parts) -> Option<String> {
    let params = params(&req).unwrap_or_else(HashMap::new);
    match params.get("channel") {
        Some(channel) => Some(channel.to_string()),
        None => None
    }
}

pub fn match_channel(req: &Parts, config: ConfigHash) -> Option<String> {
    match channel(&req) {
        Some(channel) => {
            let config = config.lock().expect("Unable to unlock config HashMap");
            match config.get(&channel) {
                Some(url) => Some(url.to_string()),
                None => {
                    log::error!("Channel not found: {}", &channel);
                    None
                }
            }
        },
        None => {
            log::error!("Missing channel parameter for post to {}", &req.uri);
            None
        }
    }
}
