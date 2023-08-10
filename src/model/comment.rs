use sqlx::FromRow;
use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Comments{

   pub id: Uuid,
   pub user_id: Option<Uuid>,
   pub post_id: Option<Uuid>,
   pub comment: String,
   pub reply_on_comment: Option<Uuid>
   // pub reply_on_comment: Option<Vec<Comments>>
}

// pub struct ReplyComment{
//    pub id:Uuid,
//    // pub user_id: Option<Uuid>,
//    // pub post_id: Option<Uuid>,
//    pub comment: String,
//    pub reply_comment: Option<Uuid>
// }

