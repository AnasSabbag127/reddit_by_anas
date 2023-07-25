mod api;
mod model;

use actix_web::{web,App,HttpServer,HttpResponse,Responder,get};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger;
use sqlx::{postgres::PgPoolOptions,Pool,Postgres};

use api::{users,post_info};

#[get("/health_check")]
async fn health_check()->impl Responder{
    HttpResponse::Ok()
}

pub struct AppState{
    #[allow(unused)]
    db:Pool<Postgres>
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    if std::env::var_os("RUST_LOg").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

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
    
    println!("DATABASE CONNECTED ");

    HttpServer::new(move||{
        App::new()
        .service(health_check)
        .wrap(Logger::default())
        .app_data(web::Data::new(AppState{db:pool.clone()}))
        .configure(users::config)
        .configure(post_info::config)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await

}
