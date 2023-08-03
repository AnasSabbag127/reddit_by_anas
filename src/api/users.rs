use actix_web::web::ReqData;
// use log::Record;
use uuid::Uuid;
use actix_web::{post,get,patch,delete,HttpResponse,web,Responder};
use serde::{Serialize, Deserialize};
use serde_json;
use argonautica::Hasher;

use crate::model::users::AccountUser;
use crate::{AppState, TokenClaims};

#[derive(Serialize, Deserialize)]
pub struct AccountUserInputData {
    username: String,
    password:String,
}

#[post("/register_user")]
pub async fn create_user(
    body: web::Json<AccountUserInputData>,
    data: web::Data<AppState>
) -> impl Responder{

    let user = body.into_inner();
    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password.clone())
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

    let query_result = sqlx::query_as::<_,AccountUser>(
        "INSERT INTO account_user(username,password) VALUES($1,$2) returning *"
        )
        .bind(user.username)
        .bind(hash)
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
    
    let id = path.into_inner();    
    let query_result = sqlx::query_as!(
        AccountUser,
        "SELECT * FROM account_user where id = $1",
        id
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
    req_user:Option<ReqData<TokenClaims>>,
    data:web::Data<AppState>
) ->impl Responder{
    
    match req_user{
        Some(_user) => {   
            let user_id = path.into_inner();
            let query_result = sqlx::query_as!(
                AccountUser,
                "DELETE FROM account_user WHERE id = $1 returning *",
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
        },
        None =>{
            HttpResponse::Unauthorized().json("Unable to verify identity")
        }
  }
}

#[patch("/update_user/{user_id}")]
async fn update_user(
    body:web::Json<AccountUserInputData>,
    req_user:Option<ReqData<TokenClaims>>,
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> impl Responder{

    match req_user{
        Some(_user)=> {
            let user = body.into_inner();
            let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
            let mut hasher = Hasher::default();
            let hash = hasher
                .with_password(user.password.clone())
                .with_secret_key(hash_secret)
                .hash()
                .unwrap();
            
            let user_id = path.into_inner();
            let query_result = sqlx::query_as!(
            AccountUser,
            "update account_user set username = $1,password=$2 where id = $3 returning *",
            user.username,
            hash,
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
        },
        None => {
            HttpResponse::Unauthorized().json("unable to verfiy identity..")
        }
    }

}

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/account")
        .service(get_user)
        .service(update_user)
        .service(delete_user);
    conf.service(scope);
    
}

