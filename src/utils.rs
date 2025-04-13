use std::env;
use std::path::PathBuf;

// TODO: should this go to a constants.rs file?
pub const FILE_PATH: &str = ".bashhh/history.sqlite3";

pub fn get_history_path() -> PathBuf {
    let mut history_path = env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    history_path.push(FILE_PATH);
    history_path
}
