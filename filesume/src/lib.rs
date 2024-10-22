#![crate_name = "filesume"]
use core::str;
use std::error::Error;
use std::fs::read_to_string;
pub struct Config {
    text: String,
    filepath: String,
    case: bool,
}
///
/// This section provides an overview of `my_function`.
///
/// ## Parameters
///
/// - `param1`: Description of the first parameter.
/// - `param2`: Description of the second parameter.
///
/// ## Example
///
/// Here's how you might use `my_function`:
///
/// ```rust
/// ```
///Hello Namaste Tata GOod Bye
/// # Code Snippet
/// ```
/// ````
/// Says "Hello, [Config](Error)" to the `Person` it is called on.

/// /! This function return String
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Require More Parameters");
        }
        let case;
        if args.len() == 4 {
            case = true;
        } else {
            case = false;
        }
        let (text, filepath) = (args[1].clone(), args[2].clone());
        Ok(Config {
            text,
            filepath,
            case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for the {} in {}", config.text, config.filepath);
    let content = read_to_string(config.filepath)?;
    if config.case {
        let res = casesearch(&config.text, &content);
        println!("{res:?}");
    } else {
        let res = search(&config.text, &content);
        println!("{res:?}");
    }
    Ok(())
}

pub fn search<'a>(text: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(text) {
            results.push(line);
        }
    }
    results
}

pub fn casesearch<'a>(text: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let text = text.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&text) {
            results.push(line);
        }
    }
    results
}
