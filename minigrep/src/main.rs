use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing the arguments: {err}");
        process::exit(1);
    });

    println!("Serching for {}", config.q);
    println!("In file {}", config.fp);

    if let Err(e) = Config::run(config){
        println!("Application error: {e}");
        process::exit(1);
    }
}

