use std::env;
use std::path::PathBuf;

pub fn get_path_to_asset(asset_rel_path: &str) -> PathBuf {
    let mut exe_path = env::current_exe()
        .expect("Failed to find executable path");

    exe_path.pop();
    exe_path.join(asset_rel_path)
}