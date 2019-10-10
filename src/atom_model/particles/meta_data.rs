use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
pub struct MetaDataKey {
    pub string: String,
}

impl MetaDataKey {
    pub fn encrypted() -> MetaDataKey {
        MetaDataKey{string: "encrypted".to_string() }
    }
}

pub struct MetaData {
    pub map: HashMap<MetaDataKey, String>,
}

impl MetaData {
    pub fn empty() -> MetaData {
        return MetaData{map: HashMap::new()}
    }
}

impl Default for MetaData {
    fn default() -> Self { MetaData::empty() }
}