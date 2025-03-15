mod gaal_core;

use std::fs::File;
use std::io::prelude::*;
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
    type Data = String;
    type Config = String;

    fn make_path(path: Vec<Self::PathItem>) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(Path::new(&path.join("/")))
    }
    fn is_path(path: Vec<Self::PathItem>) -> bool {
        Path::new(&path.join("/")).is_dir()
    }
    fn get_path() -> Vec<Self::PathItem> {
        ".gal".split("/").map(|x| x.to_string()).collect()
    }
    fn get_data(path: Vec<Self::PathItem>) -> Result<Self::Data, std::io::Error> {
        let mut file = File::open(Path::new(&path.join("/")))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
    fn save_data(path: Vec<Self::PathItem>, data: Self::Data) -> Result<(), std::io::Error> {
        let mut file = File::create(Path::new(&path.join("/")))?;
        file.write_all(data.as_bytes())
    }
    fn get_config(path: Vec<Self::PathItem>) -> Result<Self::Config, std::io::Error> {
        let mut file = File::open(Path::new(&path.join("/")))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
    fn save_config(path: Vec<Self::PathItem>, config: Self::Config) -> Result<(), std::io::Error> {
        let mut file = File::create(Path::new(&path.join("/")))?;
        file.write_all(config.as_bytes())
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
