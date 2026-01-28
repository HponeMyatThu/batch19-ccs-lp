use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MasterContent {
    pub id: Option<i32>,
    pub page_name: String,
    pub section_name: String,
    pub lang: String,
    pub content_type: String,
    pub content: String,
    pub visible: Option<i32>,
    pub display_order: Option<i32>,
}
