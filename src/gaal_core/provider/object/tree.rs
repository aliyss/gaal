use crate::gaal_core::provider::directory::GaalCoreDirectoryActions;

use super::{
    default::{GaalObject, GaalObjectAction},
    ObjectError,
};

#[derive(Clone, Debug, Default)]
pub struct TreeLeaf {
    pub mode: String,
    pub path: String,
    pub sha: String,
}

#[derive(Clone, Debug, Default)]
pub struct Tree {
    pub leafs: Vec<TreeLeaf>,
}

fn tree_parse_one(raw: String, start: usize) -> (TreeLeaf, usize) {
    let mut mode = String::new();
    let mut path = String::new();

    let mut i = start;
    while i < raw.len() && raw.as_bytes()[i] != b' ' {
        mode.push(raw.as_bytes()[i] as char);
        i += 1;
    }
    if mode.len() == 5 {
        mode = format!("0{}", mode);
    }
    i += 1; // skip space

    while i < raw.len() && raw.as_bytes()[i] != 0x00 {
        path.push(raw.as_bytes()[i] as char);
        i += 1;
    }
    i += 1; // skip null terminator

    let sha_bytes = &raw.to_string()[i..i + 20];
    if sha_bytes.len() != 20 {
        panic!("Invalid SHA length");
    }
    let raw_sha = sha_bytes
        .as_bytes()
        .iter()
        .fold(0u128, |acc, &b| acc << 8 | b as u128);
    let sha = format!("{:040x}", raw_sha);

    (TreeLeaf { mode, path, sha }, i)
}

fn tree_parse(raw: String) -> Vec<TreeLeaf> {
    let mut leafs = Vec::new();
    let mut i = 0;
    while i < raw.len() {
        let (leaf, next_i) = tree_parse_one(raw.clone(), i);
        leafs.push(leaf);
        i = next_i;
    }
    leafs
}

fn tree_leaf_sort_key(leaf: &TreeLeaf) -> String {
    if leaf.mode.starts_with("10") {
        return leaf.path.clone();
    };
    format!("{}/", leaf.path)
}

fn tree_serialize(tree: &mut Tree) -> Result<String, ObjectError> {
    tree.leafs.sort_by_key(tree_leaf_sort_key);
    let mut ret = String::new();

    for item in &tree.leafs {
        ret.push_str(&item.mode);
        ret.push(' ');

        ret.push_str(&item.path);
        ret.push(0x00 as char);

        let sha_bytes_vec = hex::decode(&item.sha)
            .map_err(|_| ObjectError::InvalidData("Invalid SHA".to_string()))?;
        if sha_bytes_vec.len() != 20 {
            return Err(ObjectError::InvalidData("SHA length is not 20".to_string()));
        }

        let sha_bytes: String = sha_bytes_vec.iter().fold(String::new(), |acc, &b| {
            let mut hex_str = String::new();
            hex_str.push_str(&format!("{:02x}", b));
            acc + &hex_str
        });
        ret.push_str(&sha_bytes);
    }

    Ok(ret)
}

impl From<String> for Tree {
    fn from(data: String) -> Self {
        let mut tree = Tree::default();

        if data.is_empty() {
            return tree;
        }
        let leafs = tree_parse(data);
        tree.leafs = leafs;
        tree
    }
}

impl From<Tree> for String {
    fn from(tree: Tree) -> Self {
        let mut tree = tree.clone();
        let result = tree_serialize(&mut tree);
        match result {
            Ok(data) => data,
            Err(e) => {
                panic!("Failed to serialize tree: {}", e);
            }
        }
    }
}

impl<GCDA> GaalObjectAction<GCDA> for GaalObject<Tree>
where
    GCDA: GaalCoreDirectoryActions,
{
    type Data = Tree;
    type Serialized = String;

    fn new(fmt: String) -> Self {
        Self {
            fmt,
            data: Tree::default(),
        }
    }

    fn fmt(&self) -> Result<Self::Serialized, ObjectError> {
        Ok(self.fmt.clone())
    }

    fn from_serialized(fmt: &str, data: Self::Serialized) -> Self {
        let deserialized =
            <GaalObject<String> as GaalObjectAction<GCDA>>::deserialize_data(data).unwrap();
        Self {
            fmt: fmt.to_string(),
            data: deserialized.into(),
        }
    }

    fn from_data(fmt: &str, data: Self::Data) -> Self {
        Self {
            fmt: fmt.to_string(),
            data,
        }
    }

    fn serialize(&self) -> Result<Self::Serialized, ObjectError> {
        Ok(self.data.clone().into())
    }

    fn serialize_data(data: Self::Data) -> Result<Self::Serialized, ObjectError> {
        Ok(data.into())
    }

    fn deserialize(&self) -> Result<Self::Data, ObjectError> {
        Ok(self.data.clone())
    }

    fn deserialize_data(data: Self::Serialized) -> Result<Self::Data, ObjectError> {
        Ok(data.into())
    }
}

#[cfg(test)]
mod test_tree_from_string {

    #[test]
    fn tree_from_string() {
        let tree = super::Tree {
            leafs: vec![super::TreeLeaf {
                path: "README.md".to_string(),
                mode: "100644".to_string(),
                sha: "1234567890abcdef1234567890abcdef12345678".to_string(),
            }],
        };

        let serialized: String = tree.into();
        let result = super::Tree::from(serialized);

        assert_eq!(result.leafs[0].mode, "100644");
        assert_eq!(result.leafs[0].path, "README.md");
        assert_eq!(
            result.leafs[0].sha,
            "1234567890abcdef1234567890abcdef12345678"
        );
    }

    #[test]
    fn tree_from_string_empty() {
        let data = "";
        let tree = super::Tree::from(data.to_string());
        assert_eq!(tree.leafs.len(), 0);
    }

    #[test]
    fn tree_into_string() {
        let serialized = "100644 README.md1234567890abcdef1234567890abcdef12345678";
        let tree = super::Tree::from(serialized.to_string());
        let serialized: String = tree.clone().into();
        println!("Serialized: {}", serialized);
        assert_eq!(tree.leafs[0].mode, "100644");
    }
}
