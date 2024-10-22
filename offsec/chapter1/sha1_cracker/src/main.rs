use hex;
use sha1::Digest;
use std::env;
use std::error::Error;
use std::fs;
fn main() -> Result<(), Box<dyn Error>> {
    const SHA1_HEX_STRING_LENGTH: usize = 40;
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage:");
        println!("sha_cracker: <wordlist.txt> <sha_hash>");
        return Ok(());
    }
    let hashee = args[2].trim();
    if hashee.len() != SHA1_HEX_STRING_LENGTH {
        return Err("SHA1 HASH IS NOT VALID".into());
    }
    let words = fs::read_to_string(args[1].trim()).expect("Expected a  file");

    for i in words.lines() {
        let common_password = i.trim().to_string();
        if hashee == hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!(
                "Hooray! Password is matched with the following : {}",
                hashee
            );
            return Ok(());
        }
    }
    return Err("Hash not found in the file".into());
}
