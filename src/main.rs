mod models;

use models::{Dependency, PackageLock};
use serde_json::Error;
use std::{collections::HashMap, convert::identity, env, fs, path::Path};

fn read_package_lock(json_path: &str) -> Result<PackageLock, Error> {
    let json_file_path = Path::new(json_path).join("package-lock.json");
    let json_contents = fs::read_to_string(json_file_path).expect("reading package-lock.json");

    serde_json::from_str(&json_contents)
}

fn find_dependency_by_name(
    dep_map: &HashMap<String, Dependency>,
    dep_name: &String,
    dep_parents: &mut Vec<String>,
) -> Vec<Option<(Vec<String>, Dependency)>> {
    let dependency_list = Vec::from_iter(dep_map.into_iter());
    let dependency_result = dependency_list
        .into_iter()
        .flat_map(|(name, dep)| {
            let mut new_dep_parents = Vec::from(dep_parents.clone());

            new_dep_parents.push(name.to_owned());

            return match name == dep_name {
                true => vec![Some((new_dep_parents, dep.to_owned()))],
                false => {
                    return match dep.dependencies.to_owned() {
                        Some(dep_map) => {
                            find_dependency_by_name(&dep_map, &dep_name, &mut new_dep_parents)
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
    let mut dep_parents: Vec<String> = Vec::new();
    let dep_result: Vec<(Vec<String>, Dependency)> =
        find_dependency_by_name(&dep_map, &dep_name, &mut dep_parents)
            .into_iter()
            .filter_map(identity)
            .collect();

    println!("{:#?}", dep_result);
}
