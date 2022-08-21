use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args)?;

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");

        process::exit(1);
    }
    Ok(())
}
