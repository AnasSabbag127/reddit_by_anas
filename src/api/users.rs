use uuid::Uuid;
use actix_web::{post,get,delete,HttpResponse,web,Responder};
use log;
use serde::{Serialize, Deserialize};
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
    
    log::info!("adding a new user in database with user name: {:?} and email: {:?} "
                ,body.user_name,body.user_email_id);
    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users(user_name,user_email_id) VALUES($1,$2) returning *",
        body.user_name,
        body.user_email_id)
        .fetch_one(&data.db)
        .await;
    
    match query_result{
        Ok(_user) =>{ 
            // log::info!(" user is adding...");
            // return HttpResponse::Ok();
            log::info!("New user details have been saved");
        
            return HttpResponse::Ok();
        },
        Err(_) => {
        return HttpResponse::InternalServerError();
        }
    }
 
}

#[get("/get_user/{user_id}")]
pub async fn get_user(
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> impl Responder {
    
    let user_id = path.into_inner();
    log::info!(" getting the user infomation with user_id {} ",user_id);
    
    let query_result = sqlx::query_as!(
        User,
        "SELECT * FROM users where user_id = $1",
        user_id
        )
        .fetch_one(&data.db)
        .await;

    match query_result{   
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No user found invalid id : "),
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
        Ok(_query) => return  HttpResponse::Ok(),
        Err(_) =>  {return HttpResponse::InternalServerError()},// here i can't the id not found error ...?
    }
  
}

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/account")
        .service(create_user)
        .service(get_user)
        .service(delete_user);
    conf.service(scope);
    
}

/*
    todo!()
    -> comments 
    -> post
    -> add match arms  --- done


    note**:
    migrations user_id replaced by id in users schema but not updated so ignore***

*/
