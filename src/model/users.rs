use uuid::Uuid;
use sqlx::{self,FromRow};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,FromRow)]
pub struct User{
    pub user_id: Uuid,
    pub user_name: Option<String>,
    pub user_email_id: Option<String>,
}