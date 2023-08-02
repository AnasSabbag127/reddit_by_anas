use actix_web::{web,HttpResponse,Responder,get,post,patch,delete};
use serde::{Serialize,Deserialize};
use uuid::Uuid;
use crate::AppState;
use crate::model::post_info::Post;
use serde_json;
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
        Ok(post) =>{ 
            let query_response = serde_json::json!({
                "status":"success",
                "data":serde_json::json!({"post":post})
            });        
            return HttpResponse::Ok().json(query_response);
        },
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "failed",
                "message":format!("{:?}",err)
            }));
        }
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
        Ok(post) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "status":"success",
                "data":serde_json::json!({"post":post})
            }));
        },
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "failed",
                "message":format!("{:?}",err)
            }));
        },
    }
}

#[delete("/delete_post/{id}")]
pub async fn delete_post(
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) ->impl Responder{

    let post_id = path.into_inner();
    let query_result = sqlx::query!(
        "DELETE FROM posts WHERE id = $1 RETURNING *",
        post_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(_post) => {
            return  HttpResponse::Ok().json(serde_json::json!({
                "status":"success",
                "message":"post deleted.."
            }));
        },
        Err(err) =>  {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status":"failed",
                "message":format!("{:?}",err)
            }));
        },
    }
  
}

#[patch("/update_post/{id}")]
async fn update_post(
    body:web::Json<PostInputData>,
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> impl Responder{

    let post_id = path.into_inner();
    let query_result = sqlx::query_as!(
    Post,
    "update posts set post_title = $1,post_text = $2 where id = $3 returning *",
    body.post_title,
    body.post_text,
    post_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(post) =>{
            let query_response = serde_json::json!({
                "status":"success",
                "data":serde_json::json!({"post":post})
            });
            return HttpResponse::Ok().json(query_response);
        },
        Err(err) =>{
            return HttpResponse::InternalServerError().json(
                serde_json::json!({"status":"failed","message":format!("{:?}",err)})
            );
        }
    }

}


pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/user_post")
        .service(post_text)
        .service(get_post)
        .service(update_post)
        .service(delete_post);
    conf.service(scope);
} 