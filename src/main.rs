mod models;

use models::{Dependency, PackageLock};
use serde_json::Error;
use std::{collections::HashMap, convert::identity, env, fs, path::Path};

#[derive(Debug, Clone)]
struct DependencyRecord {
    name: String,
    dependency: Dependency,
}

fn read_package_lock(json_path: &str) -> Result<PackageLock, Error> {
    let json_file_path = Path::new(json_path).join("package-lock.json");
    let json_contents = fs::read_to_string(json_file_path).expect("reading package-lock.json");

    serde_json::from_str(&json_contents)
}

fn find_dependency_by_name(
    dep_map: &HashMap<String, Dependency>,
    dep_name: &String,
    dep_history: &mut Vec<DependencyRecord>,
) -> Vec<Option<(Vec<DependencyRecord>, Dependency)>> {
    let dependency_list = Vec::from_iter(dep_map.into_iter());
    let dependency_result = dependency_list
        .into_iter()
        .flat_map(|(name, dependency)| {
            let mut current_dep_history = Vec::from(dep_history.clone());
            let current_dep = DependencyRecord {
                name: name.to_string(),
                dependency: dependency.to_owned(),
            };

            current_dep_history.push(current_dep);

            return match name == dep_name {
                true => vec![Some((current_dep_history, dependency.to_owned()))],
                false => {
                    return match dependency.dependencies.to_owned() {
                        Some(dep_map) => {
                            find_dependency_by_name(&dep_map, &dep_name, &mut current_dep_history)
                        }
                        None => vec![None],
                    };
                }
            };
        })
        .collect();

    dependency_result
}

#[allow(unused_variables)]
fn main() {
    let dep_name = env::args().nth(1).expect("no package given");

    let package_lock_path = match env::args().nth(2) {
        Some(p) => p,
        None => ".".to_string(),
    };

    let package_lock = match read_package_lock(&package_lock_path) {
        Ok(package_lock) => package_lock,
        Err(err) => panic!("invalid json: {}", err.to_string()),
    };

    let dep_map = package_lock.dependencies.expect("read dependencies");
    let mut dep_history: Vec<DependencyRecord> = Vec::new();
    let dep_result: Vec<(Vec<DependencyRecord>, Dependency)> =
        find_dependency_by_name(&dep_map, &dep_name, &mut dep_history)
            .into_iter()
            .filter_map(identity)
            .collect();

    dep_result.into_iter().for_each(|(parents, dependency)| {
        println!(
            "{}",
            parents
                .into_iter()
                .map(|DependencyRecord { name, dependency }| vec![
                    name,
                    dependency.version.unwrap_or("unknown version".to_owned())
                ]
                .join("@"))
                .collect::<Vec<String>>()
                .join(" => ")
        )
    })
}
