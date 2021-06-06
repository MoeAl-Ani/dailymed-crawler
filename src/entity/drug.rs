use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Drug {
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dosage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl Drug {
    pub fn new(id: Option<i32>,
               set_id: Option<String>,
               code: Option<String>,
               name: Option<String>,
               form: Option<String>,
               dosage: Option<String>,
               route: Option<String>,
               created_at: Option<DateTime<Utc>>) -> Self {
        Drug {
            id,
            set_id,
            code,
            name,
            form,
            dosage,
            route,
            created_at
        }
    }
    pub fn id(&self) -> i32 {
        self.id.as_ref().unwrap().clone()
    }
}