use actix_web::{web,HttpResponse,Responder,get,post};
use serde::{Serialize,Deserialize};
use uuid::Uuid;
use crate::AppState;
use crate::model::post_info::Post;

#[derive(Serialize,Deserialize)]
pub struct PostInputData{
    post_title:String,
    post_text:String,
}

#[post("/post_text")]
pub async fn post_text(
    body:web::Json<PostInputData>,
    data:web::Data<AppState>    
) -> impl Responder{

    let query_result = sqlx::query_as!(
        Post,
        "INSERT INTO posts(post_title,post_text) VALUES($1,$2)",
        body.post_title,
        body.post_text
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(_res) => { return HttpResponse::Ok();},
        Err(_) => {  return HttpResponse::InternalServerError();}
    }

   
}

#[get("/get_post/{post_id}")]
pub async fn get_post(
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> impl Responder{

    let post_id = path.into_inner();
    let query_result = sqlx::query_as!(
        Post,
        "select * from posts where id = $1",
        post_id)
        .fetch_one(&data.db)
        .await;

    match query_result{
        Ok(query_ok) => HttpResponse::Ok().json(query_ok),
        Err(_) => HttpResponse::NotFound().json("no post found"),
    }
}

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/user_post")
        .service(post_text)
        .service(get_post);
    conf.service(scope);
} 