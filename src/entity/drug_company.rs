use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DrugCompany {
    id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    company_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drug_id: Option<i32>
}