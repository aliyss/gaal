use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GaalDirectoryConfigSectionItem<X> {
    value: X,
}

pub trait GaalDirectoryConfigSectionItemActions {
    type Value: Into<String> + From<String> + Clone;
    fn new(value: &Self::Value) -> Self;
    fn get_value(&self) -> &Self::Value;
    fn set_value(&mut self, value: &Self::Value);
}

impl<X> GaalDirectoryConfigSectionItemActions for GaalDirectoryConfigSectionItem<X>
where
    X: Into<String> + From<String> + Clone,
{
    type Value = X;
    fn new(value: &Self::Value) -> GaalDirectoryConfigSectionItem<Self::Value> {
        Self {
            value: value.clone(),
        }
    }

    fn get_value(&self) -> &Self::Value {
        &self.value
    }

    fn set_value(&mut self, value: &Self::Value) {
        self.value = value.clone();
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GaalDirectoryConfigSection<GCDSCI>
where
    GCDSCI: GaalDirectoryConfigSectionItemActions,
{
    items: HashMap<String, GCDSCI>,
    sections: HashMap<String, Self>,
}

pub trait GaalDirectoryConfigSectionActions {
    type GCDSCI: GaalDirectoryConfigSectionItemActions;
    fn new() -> Self;
    fn get_section(&self, path: &[&str]) -> Option<&Self>;
    fn get_section_mut(&mut self, path: &[&str]) -> Option<&mut Self>;
    fn insert_section(&mut self, path: &[&str], section: Self, force: bool);
    fn delete_section(&mut self, path: &[&str]);
    fn get_item(&self, path: &[&str]) -> Option<&Self::GCDSCI>;
    fn delete_item(&mut self, path: &[&str]);
    fn insert_item(&mut self, path: &[&str], item: Self::GCDSCI, force: bool);
}

impl<GCDCSI> GaalDirectoryConfigSectionActions for GaalDirectoryConfigSection<GCDCSI>
where
    GCDCSI: GaalDirectoryConfigSectionItemActions + Clone,
{
    type GCDSCI = GCDCSI;
    fn new() -> Self {
        Self {
            items: HashMap::new(),
            sections: HashMap::new(),
        }
    }

    fn get_section(&self, path: &[&str]) -> Option<&Self> {
        let mut current_sections = &self.sections;
        let mut current_section = Option::None;
        for section_name in path {
            if section_name == &path[path.len() - 1] {
                current_section = current_sections.get(*section_name);
                break;
            }
            match current_sections.get(*section_name) {
                Some(section) => current_sections = &section.sections,
                None => {
                    return None;
                }
            }
        }
        current_section
    }

    fn get_section_mut(&mut self, path: &[&str]) -> Option<&mut Self> {
        let mut current_sections = &mut self.sections;
        let mut current_section = Option::None;
        for section_name in path {
            if section_name == &path[path.len() - 1] {
                current_section = current_sections.get_mut(*section_name);
                break;
            }
            match current_sections.get_mut(*section_name) {
                Some(section) => {
                    current_sections = &mut section.sections;
                }
                None => {
                    return None;
                }
            }
        }
        current_section
    }

    fn insert_section(&mut self, path: &[&str], section: Self, force: bool) {
        if path.is_empty() {
            eprintln!("Path cannot be empty!");
        } else if path.len() == 1 {
            self.sections.insert(path[0].to_string(), section);
        } else {
            let current_section = self.get_section_mut(&path[0..path.len() - 1]);
            if let Some(current_section) = current_section {
                current_section.insert_section(path, section, force);
            } else if force {
                let mut current_section = Self::new();
                current_section.insert_section(&path[1..path.len()], section, force);
                self.insert_section(&[path[0]], current_section, force);
            }
        }
    }

    fn delete_section(&mut self, path: &[&str]) {
        let mut current_sections = &mut self.sections;
        for section_name in path {
            if section_name == &path[path.len() - 1] {
                current_sections.remove(*section_name);
                break;
            }
            match current_sections.get_mut(*section_name) {
                Some(section) => {
                    current_sections = &mut section.sections;
                }
                None => {
                    return;
                }
            }
        }
    }

    fn get_item(&self, path: &[&str]) -> Option<&Self::GCDSCI> {
        let section_path = &path[0..path.len() - 1];
        let item_name = path[path.len() - 1];
        if section_path.is_empty() {
            return self.items.get(item_name);
        }
        let section = self.get_section(section_path)?;
        section.items.get(item_name)
    }

    fn delete_item(&mut self, path: &[&str]) {
        let section_path = &path[0..path.len() - 1];
        let item_name = path[path.len() - 1];
        if section_path.is_empty() {
            self.items.remove(item_name);
            return;
        }
        let section = self.get_section_mut(section_path);
        if let Some(section) = section {
            section.items.remove(item_name);
        }
    }

    fn insert_item(&mut self, path: &[&str], item: Self::GCDSCI, force: bool) {
        let section_path = &path[0..path.len() - 1];
        let item_name = path[path.len() - 1];
        if section_path.is_empty() {
            self.items.insert(item_name.to_string(), item);
            return;
        }
        let section = self.get_section_mut(section_path);
        if let Some(section) = section {
            section.items.insert(item_name.to_string(), item);
        } else if force {
            let mut section = Self::new();
            section.insert_item(&[item_name], item, force);
            self.insert_section(&path[0..path.len() - 1], section, force);
        }
    }
}

impl<GCDSI: GaalDirectoryConfigSectionItemActions + Clone> Default
    for GaalDirectoryConfigSection<GCDSI>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test_section {
    use super::*;

    #[test]
    fn test_get_section() {
        let mut section =
            GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        let new_section =
            GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        section.insert_section(&["section1"], new_section.clone(), false);
        let section = section.get_section(&["section1"]);
        assert_eq!(section, Some(new_section).as_ref());
    }

    #[test]
    fn test_get_multipath_section() {
        let mut section =
            GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        let new_section =
            GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        section.insert_section(
            &["section1", "section2", "section3"],
            new_section.clone(),
            true,
        );
        let retrieved_section = section.get_section(&["section1", "section2", "section3"]);
        assert_eq!(retrieved_section, Some(new_section).as_ref());
    }

    #[test]
    fn test_insert_section() {
        let mut section =
            GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        let new_section =
            GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        section.insert_section(&["section1"], new_section.clone(), false);
        let section = section.get_section(&["section1"]);
        assert_eq!(section, Some(new_section).as_ref());
    }

    #[test]
    fn test_delete_section() {
        let mut section =
            GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        let new_section =
            GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        section.insert_section(&["section1"], new_section.clone(), false);
        section.delete_section(&["section1"]);
        let section = section.get_section(&["section1"]);
        assert_eq!(section, None);
    }

    #[test]
    fn test_get_item() {
        let mut section = GaalDirectoryConfigSection::new();
        let item = GaalDirectoryConfigSectionItem {
            value: "value".to_string(),
        };
        let item_name = "item1";
        section.insert_item(&[item_name], item.clone(), false);
        let new_item = section.get_item(&[item_name]);
        assert_eq!(new_item, Some(item).as_ref());
    }

    #[test]
    fn test_get_multipath_item() {
        let mut section = GaalDirectoryConfigSection::new();
        let item = GaalDirectoryConfigSectionItem {
            value: "value".to_string(),
        };
        let item_name = "item1";
        section.insert_item(&["section1", "section2", item_name], item.clone(), true);
        let new_item = section.get_item(&["section1", "section2", item_name]);
        assert_eq!(new_item, Some(item).as_ref());
    }

    #[test]
    fn test_delete_item() {
        let mut section = GaalDirectoryConfigSection::new();
        let item = GaalDirectoryConfigSectionItem {
            value: "value".to_string(),
        };
        let item_name = "item1";
        section.insert_item(&[item_name], item.clone(), false);
        section.delete_item(&[item_name]);
        let item = section.get_item(&[item_name]);
        assert_eq!(item, None);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GaalDirectoryConfig<GCDCS>
where
    GCDCS: GaalDirectoryConfigSectionActions,
{
    id: String,
    sections: HashMap<String, GCDCS>,
}

pub trait GaalDirectoryConfigActions<GCDCS>
where
    GCDCS: GaalDirectoryConfigSectionActions,
{
    fn new(id: &str) -> Self;
    fn get_section(&self, path: &[&str]) -> Option<&GCDCS>;
    fn get_section_mut(&mut self, path: &[&str]) -> Option<&mut GCDCS>;
    fn delete_section(&mut self, path: &[&str]);
    fn insert_section(&mut self, path: &[&str], section: GCDCS, force: bool);
    fn get_item(&self, path: &[&str]) -> Option<&GCDCS::GCDSCI>;
    fn delete_item(&mut self, path: &[&str]);
    fn insert_item(&mut self, path: &[&str], item: GCDCS::GCDSCI, force: bool);
}

impl<GCDCS> GaalDirectoryConfigActions<GCDCS> for GaalDirectoryConfig<GCDCS>
where
    GCDCS: GaalDirectoryConfigSectionActions,
{
    fn new(id: &str) -> GaalDirectoryConfig<GCDCS> {
        GaalDirectoryConfig {
            id: id.to_string(),
            sections: HashMap::new(),
        }
    }

    fn get_section(&self, path: &[&str]) -> Option<&GCDCS> {
        if path.is_empty() {
            return None;
        }
        if path.len() == 1 {
            return self.sections.get(path[0]);
        }
        self.sections.get(path[0])?.get_section(&path[1..])
    }

    fn get_section_mut(&mut self, path: &[&str]) -> Option<&mut GCDCS> {
        if path.is_empty() {
            return None;
        }
        if path.len() == 1 {
            return self.sections.get_mut(path[0]);
        }
        self.sections.get_mut(path[0])?.get_section_mut(&path[1..])
    }

    fn delete_section(&mut self, path: &[&str]) {
        if path.is_empty() {
            return;
        }
        if path.len() == 1 {
            self.sections.remove(path[0]);
        } else if let Some(section) = self.sections.get_mut(path[0]) {
            section.delete_section(&path[1..]);
        }
    }

    fn insert_section(&mut self, path: &[&str], section: GCDCS, force: bool) {
        if path.is_empty() {
            eprintln!("Path cannot be empty!");
        } else if path.len() == 1 {
            self.sections.insert(path[0].to_string(), section);
        } else {
            let current_section = self.get_section_mut(&path[0..path.len() - 1]);
            if let Some(current_section) = current_section {
                current_section.insert_section(path, section, force);
            } else if force {
                let mut current_section = GCDCS::new();
                current_section.insert_section(&path[1..path.len()], section, force);
                self.insert_section(&[path[0]], current_section, force);
            }
        }
    }

    fn get_item(&self, path: &[&str]) -> Option<&GCDCS::GCDSCI> {
        if path.len() <= 1 {
            panic!("Path must have at least 2 elements");
        }
        self.sections.get(path[0])?.get_item(&path[1..])
    }

    fn delete_item(&mut self, path: &[&str]) {
        if path.is_empty() {
            return;
        }
        if path.len() == 1 {
            if let Some(section) = self.sections.get_mut(path[0]) {
                section.delete_item(&[path[0]])
            }
        } else if let Some(section) = self.sections.get_mut(path[0]) {
            section.delete_item(&path[1..]);
        }
    }

    fn insert_item(&mut self, path: &[&str], item: GCDCS::GCDSCI, force: bool) {
        if path.len() <= 1 {
            panic!("Path must have at least 2 elements");
        } else {
            let current_section = self.get_section_mut(&path[0..path.len() - 1]);
            if let Some(current_section) = current_section {
                current_section.insert_item(&[path[path.len() - 1]], item, force);
            } else if force {
                let mut current_section = GCDCS::new();
                current_section.insert_item(&path[1..path.len()], item, force);
                self.insert_section(&[path[0]], current_section, force);
            }
        }
    }
}

impl<GCDCS: GaalDirectoryConfigSectionActions> Default for GaalDirectoryConfig<GCDCS> {
    fn default() -> Self {
        let mut config = Self::new("gal_config");
        let repository_format_version = GCDCS::GCDSCI::new(&"0".to_string().into());
        config.insert_item(
            &["core", "repositoryformatversion"],
            repository_format_version,
            true,
        );
        let file_mode = GCDCS::GCDSCI::new(&"false".to_string().into());
        config.insert_item(&["core", "filemode"], file_mode, true);
        let bare = GCDCS::GCDSCI::new(&"false".to_string().into());
        config.insert_item(&["core", "bare"], bare, true);
        config
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;
    #[test]
    fn test_get_section() {
        let mut config = GaalDirectoryConfig::new("config1");
        let section = GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        config.insert_section(&["config_section1"], section.clone(), false);
        let retrieved_section = config.get_section(&["config_section1"]);
        assert_eq!(retrieved_section, Some(section).as_ref());
    }

    #[test]
    fn test_get_multipath_section() {
        let mut config = GaalDirectoryConfig::new("config1");
        let section = GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        config.insert_section(
            &["config_section1", "section2", "section3", "section4"],
            section.clone(),
            true,
        );
        let retrieved_section =
            config.get_section(&["config_section1", "section2", "section3", "section4"]);
        assert_eq!(retrieved_section, Some(section).as_ref());
    }

    #[test]
    fn test_insert_section() {
        let mut config = GaalDirectoryConfig::new("config1");
        let section = GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        config.insert_section(&["config_section1"], section.clone(), false);
        let retrieved_section = config.get_section(&["config_section1"]);
        assert_eq!(retrieved_section, Some(section).as_ref());
    }

    #[test]
    fn test_delete_section() {
        let mut config = GaalDirectoryConfig::new("config1");
        let section = GaalDirectoryConfigSection::<GaalDirectoryConfigSectionItem<String>>::new();
        config.insert_section(&["config_section1"], section.clone(), false);
        config.delete_section(&["config_section1"]);
        let section = config.get_section(&["config_section1"]);
        assert_eq!(section, None);
    }

    #[test]
    fn test_get_item() {
        let mut config = GaalDirectoryConfig::<
            GaalDirectoryConfigSection<GaalDirectoryConfigSectionItem<String>>,
        >::new("config1");
        let item = GaalDirectoryConfigSectionItem {
            value: "value".to_string(),
        };
        let item_name = "item1";
        config.insert_item(&["config_section1", item_name], item.clone(), true);
        let new_item = config.get_item(&["config_section1", item_name]);
        assert_eq!(new_item, Some(item).as_ref());
    }

    #[test]
    fn test_get_multipath_item() {
        let mut config = GaalDirectoryConfig::<
            GaalDirectoryConfigSection<GaalDirectoryConfigSectionItem<String>>,
        >::new("config1");
        let item = GaalDirectoryConfigSectionItem {
            value: "value".to_string(),
        };
        let item_name = "item1";
        config.insert_item(
            &[
                "config_section1",
                "section2",
                "section3",
                "section4",
                item_name,
            ],
            item.clone(),
            true,
        );
        let new_item = config.get_item(&[
            "config_section1",
            "section2",
            "section3",
            "section4",
            item_name,
        ]);
        assert_eq!(new_item, Some(item).as_ref());
    }

    #[test]
    fn test_delete_item() {
        let mut config = GaalDirectoryConfig::<
            GaalDirectoryConfigSection<GaalDirectoryConfigSectionItem<String>>,
        >::new("config1");
        let item = GaalDirectoryConfigSectionItem {
            value: "value".to_string(),
        };
        let item_name = "item1";
        config.insert_item(&["config_section1", item_name], item.clone(), true);
        config.delete_item(&["config_section1", item_name]);
        let item = config.get_item(&["config_section1", item_name]);
        assert_eq!(item, None);
    }

    #[test]
    fn test_insert_item() {
        let mut config = GaalDirectoryConfig::<
            GaalDirectoryConfigSection<GaalDirectoryConfigSectionItem<String>>,
        >::new("config1");
        let item = GaalDirectoryConfigSectionItem {
            value: "value".to_string(),
        };
        let item_name = "item1";
        config.insert_item(&["config_section1", item_name], item.clone(), true);
        let new_item = config.get_item(&["config_section1", item_name]);
        assert_eq!(new_item, Some(item).as_ref());
    }
}
