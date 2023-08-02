use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Comments{

   pub id: Uuid,
   pub user_id: Option<Uuid>,
   pub post_id: Option<Uuid>,
   pub comment: String
}


