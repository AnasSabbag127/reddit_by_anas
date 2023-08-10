use actix_web::web::ReqData;
use uuid::Uuid;
use actix_web::{get,HttpResponse,web,Responder};
use sqlx;
use serde_json;
use crate::model::post_info::Post;
use crate::{AppState, TokenClaims};


#[get("/my_posts/{id}")]
async fn post_by_me(
    path:web::Path<Uuid>,
    user_req:Option<ReqData<TokenClaims>>,
    data:web::Data<AppState>
) -> impl Responder{
    
    match user_req{
        Some(_user) => {
            let user_id = path.into_inner();
            let query_result = sqlx::query_as::<_,Post>(
                "SELECT * FROM posts WHERE user_id = $1 "
            )
            .bind(user_id)
            .fetch_all(&data.db)
            .await;

            match query_result{
                Ok(posts) => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "status":"success",
                        "posts": serde_json::json!(posts) 
                    }))
                },
                Err(err) => {
                    HttpResponse::InternalServerError().json(format!("{:?}",err))
                }
            }

        
        },
        None => {
            HttpResponse::Unauthorized().json("failed to verify identity")
        }
    }

}

