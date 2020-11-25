use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;



// struct EntryItem {
// 	module: &str,
// 	data: &str,
// }

// struct Config {
// 	log_path: &str,
// 	entrys: vec<EntryItem>,
// }

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

pub fn get_config (_args: &Vec<String>) -> Option<Config> {
    return match _args.get (1) {
        Some (a) => match a.as_str () {
            "-f" => match _args.get (2) {
                Some (b) => match File::open (b) {
                    Ok (mut _f) => {
                        let mut _buf = String::new ();
                        match _f.read_to_string (&mut _buf) {
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
