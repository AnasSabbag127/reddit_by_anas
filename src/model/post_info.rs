use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Post{
    pub id: Uuid,
    pub user_id:Uuid,
    pub post_title: Option<String>,
    pub post_text: Option<String>
}