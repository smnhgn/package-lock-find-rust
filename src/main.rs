use serde::{Deserialize, Serialize};
use serde_json::{Error};
use std::{env, fs, path::Path, collections::HashMap};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Package {
    version: Option<String>,
    integrity: Option<String>,
    resolved: Option<String>,
    dev: Option<bool>,
    optional: Option<bool>,
    dev_optional: Option<bool>,
    name: Option<String>,
    in_bundle: Option<bool>,
    has_install_script: Option<bool>,
    has_shrink_wrap: Option<bool>,
    license: Option<String>,
    bin: Option<HashMap<String, String>>,
    engines: Option<HashMap<String, String>>,
    dependencies: Option<HashMap<String, String>>,
    dev_dependencies: Option<HashMap<String, String>>,
    optional_dependencies: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PackageLock {
    name: String,
    version: Option<String>,
    lockfile_version: i8,
    packages: HashMap<String, Package>,
    // dependencies: Record<string, Dependency>;
}

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

    if let Ok(package_lock) = read_package_lock_json(json_path) {
        println!("{:?}", package_lock);
    };
}
