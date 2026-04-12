use std::path::PathBuf;

const APP_ID: Option<&str> = option_env!("APP_ID");

pub fn app_id() -> &'static str {
    APP_ID.expect("APP_ID env var not set")
}

pub fn history_file() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(app_id());
    std::fs::create_dir_all(&path).expect("Could not create directory");
    path.push("history.json");
    path
}
