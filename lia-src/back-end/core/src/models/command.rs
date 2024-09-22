use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub command_text: String,
    pub tags: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCommand {
    pub name: String,
    pub description: Option<String>,
    pub command_text: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCommand {
    pub name: String,
    pub new_tags: Option<Vec<String>>,
    pub new_description: Option<String>,
    pub new_command_text: Option<String>,
}
