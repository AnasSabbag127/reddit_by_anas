use uuid::Uuid;
use sqlx::{self,FromRow};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,FromRow)]
pub struct User{
    pub id: Uuid,
    pub username: Option<String>,
    pub user_email_id: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize,Deserialize,FromRow)]
pub struct AccountUser{
    pub id :Uuid,
    pub username: Option<String>,
    pub password: Option<String>,
}