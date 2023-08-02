use uuid::Uuid;
use actix_web::{post,get,patch,delete,HttpResponse,web,Responder};
use serde::{Serialize, Deserialize};
use serde_json;

use crate::model::users::User;
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct UserInputData {
    user_name: String,
    user_email_id: String,
}

#[post("/create_user")]
pub async fn create_user(
    body: web::Json<UserInputData>,
    data: web::Data<AppState>
) -> impl Responder{

    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users(user_name,user_email_id) VALUES($1,$2) returning *",
        body.user_name,
        body.user_email_id)
        .fetch_one(&data.db)
        .await;
    
    match query_result{
        Ok(user) =>{ 
            let query_response = serde_json::json!({
                "status":"success",
                "data":serde_json::json!({"user":user})
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

#[get("/get_user/{user_id}")]
pub async fn get_user(
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> impl Responder {
    
    let user_id = path.into_inner();    
    let query_result = sqlx::query_as!(
        User,
        "SELECT * FROM users where user_id = $1",
        user_id
        )
        .fetch_one(&data.db)
        .await;

    match query_result{   
        Ok(user) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "status":"success",
                "data":serde_json::json!({"user":user})
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

#[delete("/delete_user/{user_id}")]
pub async fn delete_user(
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) ->impl Responder{
    
    let user_id = path.into_inner();
    let query_result = sqlx::query!(
        "DELETE FROM users WHERE user_id = $1 returning *",
        user_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(_query) => {
            return  HttpResponse::Ok().json(serde_json::json!({
                "status":"success",
                "message":"user deleted.."
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

#[patch("/update_user/{user_id}")]
async fn update_user(
    body:web::Json<UserInputData>,
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> impl Responder{

    let user_id = path.into_inner();
    let query_result = sqlx::query_as!(
    User,
    "update users set user_name = $1,user_email_id = $2 where user_id = $3 returning *",
    body.user_name,
    body.user_email_id,
    user_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(user) =>{
            let query_response = serde_json::json!({
                "status":"success",
                "data":serde_json::json!({"user":user})
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
    let scope = web::scope("/account")
        .service(create_user)
        .service(get_user)
        .service(update_user)
        .service(delete_user);
    conf.service(scope);
    
}

