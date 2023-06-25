use std::env;          // enables reading cmd line arguments
use std::process;      // process::exit()
use minigrep::Config;

fn main() {
    // materialize the iterator to the cmd line arguments
    // the function panics on invalid Unicode
    let args: Vec<String>  = env::args().collect();
    let config             = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
    // print all command line arguments
    //dbg!(args);
}