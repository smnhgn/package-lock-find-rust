mod models;

use models::{Dependency, PackageLock};
use serde_json::Error;
use std::{collections::HashMap, env, fs, path::Path};

fn read_package_lock(json_path: &str) -> Result<PackageLock, Error> {
    let json_file_path = Path::new(json_path).join("package-lock.json");
    let json_contents = fs::read_to_string(json_file_path).expect("reading package-lock.json");

    serde_json::from_str(&json_contents)
}

fn find_dependency_by_name(
    dependencies: &HashMap<String, Dependency>,
    dependency_name: &str,
) -> Option<Dependency> {
    let empty_hash_map: HashMap<String, Dependency> = HashMap::new();
    let nested_dependencies_list: Vec<&HashMap<String, Dependency>> = dependencies
        .values()
        .map(|dependency| dependency.dependencies.as_ref().unwrap_or(&empty_hash_map))
        .collect();

    let dependency = match dependencies.get(dependency_name) {
        Some(dependency) => Some(dependency).cloned(),
        None => nested_dependencies_list
            .into_iter()
            .find_map(|dependencies| find_dependency_by_name(&dependencies, dependency_name)),
    };

    dependency
}

#[allow(unused_variables)]
fn main() {
    let dependency_name = env::args().nth(1).expect("no package given");

    let package_lock_path = match env::args().nth(2) {
        Some(p) => p,
        None => ".".to_string(),
    };

    let package_lock = match read_package_lock(&package_lock_path) {
        Ok(package_lock) => package_lock,
        Err(err) => panic!("invalid json: {}", err.to_string()),
    };

    let dependency = match package_lock.dependencies {
        Some(dependencies) => find_dependency_by_name(&dependencies, &dependency_name),
        None => panic!("couldn't find dependency: {}", dependency_name),
    };

    println!("{:#?}", dependency)
}
