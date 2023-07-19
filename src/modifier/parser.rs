use super::data;

pub struct Config {
    pub git_dir: Option<String>,
}

static DEFAULT_GIT_DIR: &str = ".gal";

pub fn init(config: Config, make_dir: fn(String) -> bool) {
    let dir = match config.git_dir {
        Some(dir) => dir,
        None => DEFAULT_GIT_DIR.to_string(),
    };
    data::init(dir, make_dir);
}
