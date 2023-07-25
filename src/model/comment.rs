use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Comments{

   pub comment_id: Uuid,
   pub user_id: Uuid,
   pub post_id:Uuid,
   pub comments: String
}


