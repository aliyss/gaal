mod gaal_core;

use std::path::Path;

use gaal_core::{
    core::{GaalCore, GaalCoreTrait},
    provider::directory::{
        GaalCoreDirectory, GaalCoreDirectoryActions, GaalCoreDirectoryActionsType,
    },
};

#[derive(Clone)]
pub struct GaalCoreDirectoryInit;

#[derive(Clone, Default)]
pub struct X {
    pub x: i32,
    pub y: i32,
}

impl X {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl GaalCoreDirectoryActionsType for GaalCoreDirectoryInit {
    type PathItem = String;

    fn make_dir(path: Vec<Self::PathItem>) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(Path::new(&path.join("/")))
    }
    fn is_dir(path: Vec<Self::PathItem>) -> bool {
        Path::new(&path.join("/")).is_dir()
    }
    fn get_dir() -> Vec<Self::PathItem> {
        ".gal".split("/").map(|x| x.to_string()).collect()
    }
}

impl GaalCoreDirectoryActions for GaalCoreDirectoryInit {}

pub type GaalCoreDirectoryBuild = GaalCoreDirectory<GaalCoreDirectoryInit>;

fn main() {
    /*
    Using main for tests at the moment.
    */
    let gal_core = GaalCore::new(GaalCoreDirectoryBuild::default());
    let repository = gal_core.init(vec!["/home/aliyss".to_string()]);
    match repository {
        Ok(_) => println!("Repository created"),
        Err(e) => {
            println!("{:?}", e.to_string())
        }
    }
}
