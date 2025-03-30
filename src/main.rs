mod gaal_core;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use gaal_core::{
    core::{GaalCore, GaalCoreTrait},
    provider::{
        directory::{GaalCoreDirectory, GaalCoreDirectoryActions, GaalCoreDirectoryActionsType},
        directory_config::{
            GaalDirectoryConfig, GaalDirectoryConfigSection, GaalDirectoryConfigSectionItem,
        },
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
    type ConfigSection = GaalDirectoryConfigSection<GaalDirectoryConfigSectionItem<String>>;
    type Config = GaalDirectoryConfig<Self::ConfigSection>;

    fn make_path(path: Vec<Self::PathItem>) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(Path::new(&path.join("/")))
    }
    fn is_path(path: Vec<Self::PathItem>) -> bool {
        Path::new(&path.join("/")).is_dir()
    }
    fn get_path() -> Vec<Self::PathItem> {
        todo!("Implement get_path")
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
    fn is_config(path: Vec<Self::PathItem>) -> bool {
        let path = path.join("/") + ".json";
        let config_path = Path::new(&path);
        Path::new(&config_path).is_file()
    }
    fn get_config(path: Vec<Self::PathItem>) -> Result<Self::Config, std::io::Error> {
        let path = path.join("/") + ".json";
        let config_path = Path::new(&path);
        let mut file = File::open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents)?)
    }
    fn save_config(path: Vec<Self::PathItem>, config: Self::Config) -> Result<(), std::io::Error> {
        let path = path.join("/") + ".json";
        let config_path = Path::new(&path);
        let mut file = File::create(config_path)?;
        file.write_all(serde_json::to_string(&config)?.as_bytes())
    }
}

impl GaalCoreDirectoryActions for GaalCoreDirectoryInit {}

pub type GaalCoreDirectoryBuild = GaalCoreDirectory<GaalCoreDirectoryInit>;

fn main() {
    /*
    Using main for tests at the moment.
    */
    let gal_core = GaalCore::new(GaalCoreDirectoryBuild::default());
    let repository =
        gal_core.derive_from_path(["", "home", "aliyss"].map(|x| x.to_string()).to_vec());
    match repository {
        Ok(_) => println!("Repository exists"),
        Err(e) => {
            println!("{:?}", e.to_string())
        }
    }
    // let repository = gal_core.init(vec!["/home/aliyss".to_string()]);
    // match repository {
    //     Ok(_) => println!("Repository created"),
    //     Err(e) => {
    //         println!("{:?}", e.to_string())
    //     }
    // }
}
