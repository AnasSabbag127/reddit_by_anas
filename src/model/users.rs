use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct User{
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email_id: String,
}