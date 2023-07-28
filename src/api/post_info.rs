use actix_web::{web,HttpResponse,Responder,get,post,delete};
use serde::{Serialize,Deserialize};
use uuid::Uuid;
use crate::AppState;
use crate::model::post_info::Post;

#[derive(Serialize,Deserialize)]
pub struct PostInputData{
    post_title:String,
    post_text:String,
}

#[post("/post_text/{user_id}")]
pub async fn post_text(
    body:web::Json<PostInputData>,
    path:web::Path<Uuid>,
    data:web::Data<AppState>    
) -> impl Responder{

    let by_user_id = path.into_inner();
    log::info!("adding a new post in database by user id : {} ",by_user_id);

    let query_result = sqlx::query_as!(
        Post,
        "INSERT INTO posts(post_title,post_text,user_id) VALUES($1,$2,$3) returning *",
        body.post_title,
        body.post_text,
        by_user_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(_res) => { 
            log::info!("New post details have been saved");
            return HttpResponse::Ok();
        },
        Err(err) => {  
            log::info!("Error : {}",err);
            return HttpResponse::InternalServerError();
        }
    }

   
}

#[get("/get_post/{post_id}")]
pub async fn get_post(
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> impl Responder{

    let post_id = path.into_inner();
    log::info!(" getting the post information with post_id: {} ",post_id);
    let query_result = sqlx::query_as!(
        Post,
        "select * from posts where id = $1",
        post_id)
        .fetch_one(&data.db)
        .await;

    match query_result{
        Ok(query_ok) => {
            log::info!("post is showing..");
            HttpResponse::Ok().json(query_ok)
        },
        Err(err) => {
            log::info!("Error: {} and INVALID post ID: {} ",err,post_id);
            HttpResponse::NotFound().json("no post found")
        },
    }
}

#[delete("/delete_post/{post_id}")]
pub async fn delete_post(
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) ->impl Responder{

    let post_id = path.into_inner();
    log::info!("deleting post of post_id : {}",post_id);
    let query_result = sqlx::query!(
        "DELETE FROM posts WHERE id = $1 RETURNING *",
        post_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(_query) => {
            log::info!("POST DELETED...");
            return  HttpResponse::Ok()
        },
        Err(err) =>  {
            log::info!("Error : {} ",err);
            return HttpResponse::InternalServerError()
        },// here i can't return id not found error ...?
    }
  
}

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/user_post")
        .service(post_text)
        .service(get_post)
        .service(delete_post);
    conf.service(scope);
} 