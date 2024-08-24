use minigrep::Config;
use std::{env::args, process};

#[derive(Debug)]
struct Config {
    text: String,
    filepath: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Require More Parameters");
        }
        let (text, filepath) = (args[1].clone(), args[2].clone());
        Ok(Config { text, filepath })
    }
}

fn main() {
    println!("Enter in the Format of -- [text] [filepath]");
    let vars: Vec<String> = args().collect();
    let config = Config::build(&vars).unwrap_or_else(|err| {
        println!("GOt an Err of {err}");
        process::exit(999);
    });
    if let Err(err) = run(config) {
        println!("Error Had Occured due to {}", err);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for the {} in {}", config.text, config.filepath);
    let content = read_to_string(config.filepath)?;
    println!("{content}");
    Ok(())
}
