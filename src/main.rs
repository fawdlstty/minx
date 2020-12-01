// cargo run -- -f minx.cfg
use std::{thread, time};

mod config;
use self::config::*;
mod services;
use self::services::*;



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
    let _cfg = get_config (&_args);
    match _cfg {
        Some (_cfg) => {
            let mut _services = ServiceManager::new (&_cfg.modules);
            _services.send ("logger", "hello");
        },
        None => help (),
    }
    loop {
        let _ten_s = time::Duration::from_secs(10);
        thread::sleep(_ten_s);
    }
}
