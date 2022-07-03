// https://docs.npmjs.com/cli/v8/configuring-npm/package-lock-json#file-format

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Package {
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
    // engines: Option<HashMap<String, String>>,
    dependencies: Option<HashMap<String, String>>,
    dev_dependencies: Option<HashMap<String, String>>,
    optional_dependencies: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    version: Option<String>,
    integrity: Option<String>,
    resolved: Option<String>,
    dev: Option<bool>,
    optional: Option<bool>,
    dev_optional: Option<bool>,
    bundled: Option<bool>,
    requires: Option<HashMap<String, String>>,
    pub dependencies: Option<HashMap<String, Dependency>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageLock {
    name: String,
    version: Option<String>,
    lockfile_version: i8,
    packages: HashMap<String, Package>,
    pub dependencies: Option<HashMap<String, Dependency>>,
}