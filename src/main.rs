mod models;

use models::PackageLock;
use serde_json::{Error};
use std::{env, fs, path::Path};

fn read_package_lock_json(json_path: &str) -> Result<PackageLock, Error> {
    let json_file_path = Path::new(json_path).join("package-lock.json");
    let json_contents = fs::read_to_string(json_file_path).expect("reading package-lock.json");

    serde_json::from_str(&json_contents)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_path = match args.len() {
        2 => &args[1],
        _ => ".",
    };

    match read_package_lock_json(json_path) {
        Ok(package_lock) => println!("{:#?}", package_lock),
        Err(err) => panic!("invalid json: {}", err.to_string()),
    };
}
