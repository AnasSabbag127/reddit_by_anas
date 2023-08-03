use sqlx::FromRow;
use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,FromRow)]
pub struct Post{
    pub id: Uuid,
    pub user_id:Option<Uuid>,
    pub post_title: Option<String>,
    pub post_text: Option<String>,
}