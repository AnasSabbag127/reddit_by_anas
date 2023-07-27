use uuid::Uuid;
use actix_web::{web,HttpResponse,Responder,get,post,delete};
use crate::{model::comment::Comments, AppState};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct CommentInputData{
    user_id: Uuid,
    post_id: Uuid,
    comment: String
}



#[post("/post_comment")]
async fn post_comment(
    body:web::Json<CommentInputData>,
    data:web::Data<AppState>
)->impl Responder{

    let query_result = sqlx::query_as!(
        Comments,
        "INSERT INTO comments(user_id,post_id,comment) VALUES($1,$2,$3) returning *",
        body.user_id,
        body.post_id,
        body.comment,    
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(_comment) => return HttpResponse::Ok(),
        Err(_) => return HttpResponse::InternalServerError(),
    }

}


#[get("/get_comments/{id}")]
async fn get_comments(
    path: web::Path<Uuid>,
    data: web::Data<AppState>
) -> impl Responder{

    let comment_id = path.into_inner();
    let query_result = sqlx::query_as!(
        Comments,
        "SELECT * FROM comments where id = $1",
        comment_id
    )
    .fetch_one(&data.db)
    .await;
    match query_result{
        Ok(comment)=>{
            HttpResponse::Ok().json(comment)
        },
        Err(_err)=>{
            HttpResponse::NotFound().json("comment does not exists with that id : ")
        }
    }

} 
#[delete("/delete_comment/{id}")]
pub async fn delete_comment(
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) ->impl Responder{

    let comment_id = path.into_inner();
    let query_result = sqlx::query!(
        "DELETE FROM comments WHERE id = $1 returning *",
        comment_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(_query) => return  HttpResponse::Ok(),
        Err(_) =>  {return HttpResponse::InternalServerError()},// here i can't the id not found error ...?
    }
  
}

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/comments")
        .service(post_comment)
        .service(get_comments)
        .service(delete_comment);
    conf.service(scope);

}