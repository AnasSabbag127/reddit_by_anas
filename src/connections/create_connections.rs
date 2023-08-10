use actix_web::web::ReqData;
use uuid::Uuid;
use actix_web::{get,post,delete,HttpResponse,web,Responder};
use sqlx;
use serde_json;
use crate:: model::users::{ToFollow,Follower, FollowerNames};
use crate::{AppState,TokenClaims};

//make connections by 
#[post("/follow/{by_user_id}")]
pub async fn follow_user(
    path:web::Path<Uuid>,
    req_user:Option<ReqData<TokenClaims>>,
    body:web::Json<ToFollow>,
    data:web::Data<AppState>
)->impl Responder{
    match req_user{
        Some(_user)=>{
            let user_id = path.into_inner();
            let query_result = sqlx::query_as::<_,Follower>(
                "INSERT INTO followers(user_id,follow) VALUES($1,$2) RETURNING *"
            )
            .bind(user_id)
            .bind(body.follow_id)
            .fetch_one(&data.db)
            .await;
            
            match query_result{
                Ok(user) =>{
                    let query_response = serde_json::json!({
                        "status":"success",
                        "connections":serde_json::json!(user)
                    });        
                    return HttpResponse::Ok().json(query_response);
                },
                Err(err)=>{
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "status":"failed",
                        "message": format!("{:?}",err)
                    }))
                }
            }

        },
        None =>{
            HttpResponse::Unauthorized().json("Unable to identify user")
        }
    }


}


#[get("/show_followers/{user_id}")]
pub async fn show_followers(
    path:web::Path<Uuid>,
    req_user:Option<ReqData<TokenClaims>>,
    data:web::Data<AppState>
)->impl Responder{
    match req_user{
        Some(_user)=>{
            let user_id = path.into_inner();
            let query_result = sqlx::query_as::<_,FollowerNames>(
                "select username  from followers inner join account_user on 
                followers.follow = account_user.id
                where followers.user_id =  $1"
            )
            .bind(user_id)
            .fetch_all(&data.db)
            .await;
            
            match query_result{
                Ok(user) =>{
                    let query_response = serde_json::json!({
                        "status":"success",
                        "connections":serde_json::json!(user)
                    });        
                    return HttpResponse::Ok().json(query_response);
                },
                Err(err)=>{
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "status":"failed",
                        "message": format!("{:?}",err)
                    }))
                }
            }

        },
        None =>{
            HttpResponse::Unauthorized().json("Unable to identify user")
        }
    }


}

//for unfollow users

#[delete("/unfollow/{user_id}")]
pub async fn unfollow_user(
    path:web::Path<Uuid>,
    req_user:Option<ReqData<TokenClaims>>,
    data:web::Data<AppState>
)-> impl Responder{
        match req_user {
            Some(_user)=>{
                let unfollow_id = path.into_inner();
                let query_result = sqlx::query_as::<_,Follower>(
                    "delete from followers where follow =$1"
                )
                .bind(unfollow_id)
                .fetch_one(&data.db)
                .await;
                match query_result{
                    Ok(_follow)=>{
                        HttpResponse::Ok().json(serde_json::json!({
                            "status":"success",
                            "message":"unfollow the user"
                        }))
                    },
                    Err(err)=>{
                        HttpResponse::InternalServerError().json(serde_json::json!({
                            "status":"failed",
                            "message":format!("{:?}",err)
                        }))
                    }
                }


                
            },
            None =>{
                HttpResponse::Unauthorized().json("Unable to identify user/follower")
            }
        }
}


/*
--> and update for social media for sharing videos and photos file; 
--> comment expand..    
*/




pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/make_connections")
        .service(follow_user)
        .service(show_followers);
    conf.service(scope);
} 