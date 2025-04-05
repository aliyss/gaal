mod gaal_core;

use flate2::Compression;
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use gaal_core::provider::object::kvlm::Kvlm;
use gaal_core::provider::object::tree::Tree;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

use gaal_core::{
    core::{GaalCore, GaalCoreTrait},
    provider::{
        directory::{GaalCoreDirectory, GaalCoreDirectoryActions, GaalCoreDirectoryActionsType},
        directory_config::{
            GaalDirectoryConfig, GaalDirectoryConfigSection, GaalDirectoryConfigSectionItem,
        },
        directory_object::{GaalCoreDirectoryObjectsActions, GaalCoreDirectoryObjectsActionsType},
        object::default::GaalObject,
    },
};

#[derive(Clone, Debug)]
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

    fn make_entry(path: Vec<Self::PathItem>) -> Result<(), std::io::Error> {
        let file_path = path.join("/");
        Self::make_path(path[0..path.len() - 1].to_vec())?;
        let mut file = File::create(Path::new(&file_path))?;
        file.write_all(b"{}")
    }

    fn is_path(path: Vec<Self::PathItem>) -> bool {
        Path::new(&path.join("/")).is_dir()
    }

    fn is_entry(path: Vec<Self::PathItem>) -> bool {
        Path::new(&path.join("/")).is_file()
    }

    fn get_path() -> Vec<Self::PathItem> {
        todo!("Implement get_path")
    }
    fn get_data(path: Vec<Self::PathItem>, uncompress: bool) -> Result<Self::Data, std::io::Error> {
        let mut file = File::open(Path::new(&path.join("/")))?;

        if uncompress {
            let b = BufReader::new(file);
            let mut decoder = ZlibDecoder::new(b);
            let mut contents = String::new();
            decoder.read_to_string(&mut contents)?;
            return Ok(contents);
        }
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }
    fn save_data(
        path: Vec<Self::PathItem>,
        data: Self::Data,
        compressed: bool,
    ) -> Result<(), std::io::Error> {
        let mut file = File::create(Path::new(&path.join("/")))?;
        if compressed {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data.as_bytes())?;
            let compressed_data = encoder.finish()?;
            return file.write_all(&compressed_data);
        }

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
    fn hash_object_to_path(hash: String) -> Vec<Self::PathItem> {
        let dir = hash.chars().take(2).collect::<String>();
        let file = hash.chars().skip(2).collect::<String>();
        let path = vec![dir, file];
        path
    }
}

impl GaalCoreDirectoryActions for GaalCoreDirectoryInit {}

#[derive(Clone)]
pub struct GaalCoreDirectoryObjectInit;

impl GaalCoreDirectoryObjectsActionsType<GaalCoreDirectoryInit> for GaalCoreDirectoryObjectInit {
    type GaalBlob = GaalObject<String>;
    type GaalCommit = GaalObject<Kvlm>;
    type GaalTag = GaalObject<String>;
    type GaalTree = GaalObject<Tree>;
}

impl GaalCoreDirectoryObjectsActions<GaalCoreDirectoryInit> for GaalCoreDirectoryObjectInit {}

pub type GaalCoreDirectoryBuild =
    GaalCoreDirectory<GaalCoreDirectoryInit, GaalCoreDirectoryObjectInit>;

fn main() {
    /*
    Using main for tests at the moment.
    */
    let gal_core = GaalCore::new(GaalCoreDirectoryBuild::default());
    let repository =
        gal_core.derive_from_path(["", "home", "aliyss"].map(|x| x.to_string()).to_vec());
    let usable_repo = match repository {
        Ok(repo) => repo,
        Err(e) => {
            println!("{:?}", e.to_string());
            return;
        }
    };

    let data = "Subject Hello\n\
            From Alice\n\
            To Bob\n\
            MultilineField This is a\n \
             multiline\n \
             message\n\
            \n\
            This is the message \n\
            body";
    let gaal_commit = GaalCoreDirectoryObjectInit::new_object("tree".to_string(), data.to_string());

    let x = usable_repo.object_write(gaal_commit).unwrap();
    let y = usable_repo.object_read(x);
    match y {
        Ok(data) => println!("{:?}", data),
        Err(e) => {
            println!("{:?}", e.to_string());
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
