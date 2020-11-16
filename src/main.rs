// cargo run -- -f minx.cfg

mod config;
use self::config::*;



fn help () {
    println!("Usage: minx [-f json_file]");
    println!("Example:");
    println!("    minx -f config.cfg          # load config.cfg and running");
    println!("");
}

fn main() {
    let _args: Vec<String> = std::env::args ().collect ();
    //println!("{:?}", args);
    match get_config (&_args) {
        Some (_cfg) => {
            println! ("log_path: {}", _cfg.log_path);
        },
        None => help (),
    }
}
