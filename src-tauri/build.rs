use std::fs;
use serde_json;

fn main() {
  //get version from tauri configstargazer_tauri-plugin-shell = "2.0.0-rc"

  let config_str = fs::read_to_string("tauri.conf.json")
        .expect("Failed to read tauri.conf.json");
  let config: serde_json::Value = serde_json::from_str(&config_str)
        .expect("Failed to parse tauri.conf.json");

  let version_str = config["version"].as_str()
        .expect("Failed to get version from tauri.conf.json");

  let version: String = version_str.chars().filter(|c| c.is_digit(10)).collect();

  //ensure version is parsable as a number
  let _version = version.parse::<u128>()
        .expect(&format!{"Version {} is not a valid number", config_str});

  //pass version as an environment variable
  println!("cargo:rustc-env=VERSION={}", version);

  //build
  tauri_build::build()
}