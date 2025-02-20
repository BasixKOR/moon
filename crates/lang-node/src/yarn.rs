use crate::YARN;
use cached::proc_macro::cached;
use moon_error::MoonError;
use moon_lang::{config_cache, LockfileDependencyVersions};
use moon_utils::fs::sync::read_yaml;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

config_cache!(YarnLock, YARN.lock_filename, read_yaml);

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YarnLockDependency {
    pub bin: Option<HashMap<String, String>>,
    pub checksum: Option<String>,
    pub dependencies: Option<HashMap<String, Value>>,
    pub language_name: String,
    pub link_type: String,
    pub peer_dependencies: Option<HashMap<String, Value>>,
    pub peer_dependencies_meta: Option<Value>,
    pub resolution: String,
    pub version: String,

    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YarnLockMetadata {
    pub cache_key: Value,
    pub version: Value,

    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct YarnLock {
    #[serde(rename = "__metadata")]
    pub metadata: YarnLockMetadata,

    #[serde(flatten)]
    pub dependencies: HashMap<String, YarnLockDependency>,

    #[serde(skip)]
    pub path: PathBuf,
}

#[cached(result)]
pub fn load_lockfile_dependencies(path: PathBuf) -> Result<LockfileDependencyVersions, MoonError> {
    let mut deps: LockfileDependencyVersions = HashMap::new();

    if let Some(lockfile) = YarnLock::read(path)? {
        for dep in lockfile.dependencies.values() {
            let name = if let Some(at_index) = dep.resolution.rfind('@') {
                &dep.resolution[0..at_index]
            } else {
                &dep.resolution
            };

            if let Some(versions) = deps.get_mut(name) {
                versions.push(dep.version.to_owned());
            } else {
                deps.insert(name.to_owned(), vec![dep.version.to_owned()]);
            }
        }
    }

    Ok(deps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use moon_utils::string_vec;
    use pretty_assertions::assert_eq;
    use serde_yaml::Number;

    #[test]
    fn parses_lockfile() {
        let temp = assert_fs::TempDir::new().unwrap();

        temp.child("yarn.lock").write_str(r#"
# This file is generated by running "yarn install" inside your project.
# Manual changes might be lost - proceed with caution!

__metadata:
  version: 6
  cacheKey: 8

"@algolia/autocomplete-core@npm:1.5.2":
  version: 1.5.2
  resolution: "@algolia/autocomplete-core@npm:1.5.2"
  dependencies:
    "@algolia/autocomplete-shared": 1.5.2
  checksum: a8ab49c689c7fe7782980af167dfd9bea0feb9fe9809d003da509096550852f48abff27b59e0bc9909a455fb998ff4b8a7ce45b7dd42cef42f675c81340a47e9
  languageName: node
  linkType: hard

"@babel/plugin-syntax-logical-assignment-operators@npm:^7.10.4, @babel/plugin-syntax-logical-assignment-operators@npm:^7.8.3":
  version: 7.10.4
  resolution: "@babel/plugin-syntax-logical-assignment-operators@npm:7.10.4"
  dependencies:
    "@babel/helper-plugin-utils": ^7.10.4
  peerDependencies:
    "@babel/core": ^7.0.0-0
  checksum: aff33577037e34e515911255cdbb1fd39efee33658aa00b8a5fd3a4b903585112d037cce1cc9e4632f0487dc554486106b79ccd5ea63a2e00df4363f6d4ff886
  languageName: node
  linkType: hard

"eslint-scope@npm:5.1.1, eslint-scope@npm:^5.1.1":
  version: 5.1.1
  resolution: "eslint-scope@npm:5.1.1"
  dependencies:
    esrecurse: ^4.3.0
    estraverse: 4
  checksum: 47e4b6a3f0cc29c7feedee6c67b225a2da7e155802c6ea13bbef4ac6b9e10c66cd2dcb987867ef176292bf4e64eccc680a49e35e9e9c669f4a02bac17e86abdb
  languageName: node
  linkType: hard
"#).unwrap();

        let lockfile: YarnLock = read_yaml(temp.path().join("yarn.lock")).unwrap();

        assert_eq!(
            lockfile,
            YarnLock {
                metadata: YarnLockMetadata {
                    cache_key: Value::Number(Number::from(8)),
                    version: Value::Number(Number::from(6)),
                    ..YarnLockMetadata::default()
                },
                dependencies: HashMap::from([
                    ("@algolia/autocomplete-core@npm:1.5.2".to_owned(), YarnLockDependency {
                        checksum: Some("a8ab49c689c7fe7782980af167dfd9bea0feb9fe9809d003da509096550852f48abff27b59e0bc9909a455fb998ff4b8a7ce45b7dd42cef42f675c81340a47e9".into()),
                        dependencies: Some(HashMap::from([
                            ("@algolia/autocomplete-shared".to_owned(), Value::String("1.5.2".to_owned()))
                        ])),
                        language_name: "node".into(),
                        link_type: "hard".into(),
                        resolution: "@algolia/autocomplete-core@npm:1.5.2".into(),
                        version: "1.5.2".into(),
                        ..YarnLockDependency::default()
                    }),
                    ("@babel/plugin-syntax-logical-assignment-operators@npm:^7.10.4, @babel/plugin-syntax-logical-assignment-operators@npm:^7.8.3".to_owned(), YarnLockDependency {
                        checksum: Some("aff33577037e34e515911255cdbb1fd39efee33658aa00b8a5fd3a4b903585112d037cce1cc9e4632f0487dc554486106b79ccd5ea63a2e00df4363f6d4ff886".into()),
                        dependencies: Some(HashMap::from([
                            ("@babel/helper-plugin-utils".to_owned(), Value::String("^7.10.4".to_owned()))
                        ])),
                        language_name: "node".into(),
                        link_type: "hard".into(),
                        peer_dependencies: Some(HashMap::from([
                            ("@babel/core".to_owned(), Value::String("^7.0.0-0".to_owned()))
                        ])),
                        resolution: "@babel/plugin-syntax-logical-assignment-operators@npm:7.10.4".into(),
                        version: "7.10.4".into(),
                        ..YarnLockDependency::default()
                    }),
                    ("eslint-scope@npm:5.1.1, eslint-scope@npm:^5.1.1".to_owned(), YarnLockDependency {
                        checksum: Some("47e4b6a3f0cc29c7feedee6c67b225a2da7e155802c6ea13bbef4ac6b9e10c66cd2dcb987867ef176292bf4e64eccc680a49e35e9e9c669f4a02bac17e86abdb".into()),
                        dependencies: Some(HashMap::from([
                            ("estraverse".to_owned(), Value::Number(Number::from(4))),
                            ("esrecurse".to_owned(), Value::String("^4.3.0".to_owned()))
                        ])),
                        language_name: "node".into(),
                        link_type: "hard".into(),
                        resolution: "eslint-scope@npm:5.1.1".into(),
                        version: "5.1.1".into(),
                        ..YarnLockDependency::default()
                    })
                ]),
                ..YarnLock::default()
            }
        );

        assert_eq!(
            load_lockfile_dependencies(temp.path().join("yarn.lock")).unwrap(),
            HashMap::from([
                (
                    "@algolia/autocomplete-core".to_owned(),
                    string_vec!["1.5.2"]
                ),
                (
                    "@babel/plugin-syntax-logical-assignment-operators".to_owned(),
                    string_vec!["7.10.4"]
                ),
                ("eslint-scope".to_owned(), string_vec!["5.1.1"])
            ])
        );

        temp.close().unwrap();
    }

    #[test]
    fn parses_complex_lockfile() {
        let content = reqwest::blocking::get(
            "https://raw.githubusercontent.com/moonrepo/moon/master/yarn.lock",
        )
        .unwrap()
        .text()
        .unwrap();

        let _: YarnLock = serde_yaml::from_str(&content).unwrap();
    }
}
