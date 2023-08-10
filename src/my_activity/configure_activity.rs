use actix_web::web;
use crate::my_activity::{my_posts,comments_by_me};

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/my_activity")
        .service(my_posts::post_by_me)
        .service(comments_by_me::comment_by_me);
    conf.service(scope);
} 