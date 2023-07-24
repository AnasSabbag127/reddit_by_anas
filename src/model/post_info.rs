use uuid::Uuid;

pub struct Post{
    pub post_id: Uuid,
    pub user_id:Uuid,
    pub post_title: String,
    pub post_text: String
}