use async_std::fs::File;
use async_std::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;



#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleItem {
    pub m_type: String,
    pub m_name: String,
    pub m_param: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server_port: i32,
    pub log_console: bool,
    pub modules: Vec<ModuleItem>,
}

pub async fn get_config (_args: &Vec<String>) -> Option<Config> {
    return match _args.get (1) {
        Some (a) => match a.as_str () {
            "-f" => match _args.get (2) {
                Some (b) => match File::open (b).await {
                    Ok (mut _f) => {
                        let mut _buf = String::new ();
                        match _f.read_to_string (&mut _buf).await {
                            Ok (_) => match serde_json::from_str (&_buf.as_str ()) {
                                Ok (c) => Some (c),
                                Err (_) => None,
                            },
                            Err (_) => None,
                        }
                    },
                    Err (_) => None,
                },
                None => None,
            },
            _ => None,
        },
        None => None,
    };
}
