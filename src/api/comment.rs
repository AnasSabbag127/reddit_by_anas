use uuid::Uuid;
use actix_web::{web,HttpResponse,Responder,get,post,delete,patch};
use crate::{model::comment::Comments, AppState};
use serde::{Serialize,Deserialize};
use serde_json;
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
        Ok(comment) =>{ 
            let query_response = serde_json::json!({
                "status":"success",
                "data":serde_json::json!({"comment":comment})
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
        Ok(comment) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "status":"success",
                "data":serde_json::json!({"comment":comment})
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
        Ok(_comment) => {
            return  HttpResponse::Ok().json(serde_json::json!({
                "status":"success",
                "message":"comment deleted.."
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


#[patch("/update_comment/{id}")]
async fn update_comment(
    body:web::Json<CommentInputData>,
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> impl Responder{

    let comment_id = path.into_inner();
    let query_result = sqlx::query_as!(
    Comments,
    "update comments set comment = $1 where id = $2 and user_id =$3  returning *",
    body.comment,
    comment_id,
    body.user_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(comment) =>{
            let query_response = serde_json::json!({
                "status":"success",
                "data":serde_json::json!({"comment":comment})
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
    let scope = web::scope("/comments")
        .service(post_comment)
        .service(get_comments)
        .service(update_comment)
        .service(delete_comment);
    conf.service(scope);

}