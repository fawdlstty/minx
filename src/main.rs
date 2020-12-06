// cargo run -- -f minx.cfg

mod config;
//use async_std::task::{sleep, spawn};

use self::config::*;
mod services;
use self::services::*;



fn help () {
    println!("Usage: minx [-f json_file]");
    println!("Example:");
    println!("    minx -f config.cfg          # load config.cfg and running");
    println!("");
}

#[async_std::main]
async fn main () {
    let _args: Vec<String> = std::env::args ().collect ();
    //println!("{:?}", args);
    let _cfg = get_config (&_args).await;
    match _cfg {
        Some (_cfg) => {
            let mut _services = ServiceManager::new (_cfg.modules);
            //_services.async_logger_critical ("main", "Program Start.").await;
        },
        None => help (),
    }
}
