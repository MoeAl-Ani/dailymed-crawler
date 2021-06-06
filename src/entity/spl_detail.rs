use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct SplsData {
    pub title: String,
    pub setid: String
}
#[derive(Deserialize, Serialize, Debug)]
pub struct MetaData {
    pub next_page_url: Option<String>,
    pub previous_page_url: Option<String>
}
impl MetaData {
    pub fn new() -> Self {
        MetaData {
            next_page_url: None,
            previous_page_url: None
        }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SplRootObject {
    pub data: Vec<SplsData>,
    pub metadata: MetaData
}

impl SplRootObject {
    pub fn new() -> Self {
        SplRootObject {
            data: vec![],
            metadata: MetaData::new()
        }
    }
}