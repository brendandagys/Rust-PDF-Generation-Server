use serde::Deserialize;

#[derive(Deserialize)]
pub struct Comment {
    pub note: String,
    pub created_user_name: Option<String>,
    pub created_timestamp: String,
}
