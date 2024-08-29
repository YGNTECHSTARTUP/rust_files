use filesume::run;
use filesume::Config;
use helo::add;
use std::{env::args, process};
fn main() {
    let re = add(1, 2);
    dbg!(re);
    println!("Enter in the Format of -- [text] [filepath] [case_senistivity]");
    let vars: Vec<String> = args().collect();
    let config = Config::build(&vars).unwrap_or_else(|err| {
        println!("GOt an Err of {err}");

        println!(
            "Enter in the Format of -- [text] [filepath] [press any keyword for case_senistivity]"
        );
        process::exit(999);
    });
    if let Err(err) = run(config) {
        println!("Error Had Occured due to {}", err);
        process::exit(1);
    }
}
