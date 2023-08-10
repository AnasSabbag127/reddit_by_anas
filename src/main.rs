mod api;
mod model;
mod authentication;
mod my_activity;
mod connections;
mod web_socket_connections;
mod multimedia_api;
use actix::Actor;
use api::{users,post_info,comment,basic_auth,post_images};
use my_activity::configure_activity;
use authentication::validator::validator;
use connections::create_connections;
use multimedia_api::post_image;

use web_socket_connections::{lobby::Lobby,start_connection::start_connection};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::{Serialize,Deserialize};


use actix_web::{
    web,App,
    HttpServer
};

use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger;
use sqlx::{postgres::PgPoolOptions,Pool,Postgres};

use uuid::Uuid;
pub struct AppState{
    #[allow(unused)]
    db:Pool<Postgres>
}


#[derive(Serialize,Deserialize,Clone)]
pub struct TokenClaims{
    id:Uuid
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "info");
    }
    dotenv().ok();
    env_logger::init();

    let chat_server = Lobby::default().start();
    let database_url = std::env::var("DATABASE_URL").expect("database url must be set");
    let pool = match PgPoolOptions::new()
                .max_connections(10)
                .connect(&database_url)
                .await
                {
                    Ok(pool)=>{
                        println!("connect to database successfull ");
                        pool
                    },
                    Err(err)=>{
                        println!("failed to connect !! {:?} ",err);
                        std::process::exit(1)
                    }
                };
    
    log::info!("DATABASE CONNECTED ");

    HttpServer::new(move||{
        let bearer_middleware = HttpAuthentication::bearer(validator);
        App::new()
        .wrap(Logger::default())
        .app_data(web::Data::new(AppState{db:pool.clone()}))
        .service(basic_auth::basic_auth)
        .service(users::create_user)
        .service(post_images::upload_image)
        .service(post_images::delete_image)
        .service(post_images::get_image)
        .service(
            web::scope("")
            .wrap(bearer_middleware)
            .configure(post_info::config)
            .configure(users::config)
            .configure(comment::config)
            .configure(configure_activity::config)
            .configure(create_connections::config)
        )
        .app_data(chat_server.clone())
        .service(start_connection)
        
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await

}


/*

*/