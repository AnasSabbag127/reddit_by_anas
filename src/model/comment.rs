use uuid::Uuid;

pub struct Comments{

   pub comment_id: Uuid,
   pub user_id: Uuid,
   pub post_id:Uuid,
   pub comments: String
}


