// cargo run -- -f minx.cfg

mod config;
use self::config::*;
mod services;
use self::services::*;



use std::{thread, time};



fn help () {
    println!("Usage: minx [-f json_file]");
    println!("Example:");
    println!("    minx -f config.cfg          # load config.cfg and running");
    println!("");
}

//#[async_std::main]
fn main() {
    let _args: Vec<String> = std::env::args ().collect ();
    //println!("{:?}", args);
    g_config = get_config (&_args);
    match g_config {
        Some (_cfg) => {
            let _services = ServiceManager::new (&_cfg.modules);
        },
        None => help (),
    }
    loop {
        let _ten_s = time::Duration::from_secs(10);
        thread::sleep(_ten_s);
    }
}
