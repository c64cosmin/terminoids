use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let source_path = current_dir.join("asset/logo.obj");
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let destination_dir = PathBuf::from(&target_dir).join(&profile);
    fs::create_dir_all(&destination_dir).unwrap();
    let destination_path = destination_dir.join("logo.obj");
    fs::copy(&source_path, &destination_path).unwrap();
}
