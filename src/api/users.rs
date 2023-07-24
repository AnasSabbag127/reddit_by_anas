use uuid::Uuid;
use actix_web::{post,get,HttpResponse,web,Responder};
use log;

use crate::model::users::User;
use crate::AppState;


#[post("/create_user")]
pub async fn create_user(
    body: web::Json<User>,
    data: web::Data<AppState>
) -> impl Responder{
    todo!();


    HttpResponse::Ok()
}



#[get("/get_user/{user_id}")]
pub async fn get_user(
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> impl Responder{
    todo!();
    HttpResponse::Ok()
}



pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/api")
        .service(create_user)
        .service(get_user);
    conf.service(scope);
    
}