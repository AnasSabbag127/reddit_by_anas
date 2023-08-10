use actix_multipart::Multipart;
use actix_web::{web,web::Buf,HttpResponse,post,get,delete, HttpRequest};
use uuid::Uuid;
use sqlx;
use crate::AppState;
use crate::model::post_info::PostImage;
use actix_web::http::header::CONTENT_LENGTH;
use mime::{Mime,IMAGE_PNG,IMAGE_JPEG, IMAGE_GIF};
use futures_util::TryStreamExt as _ ;

#[post("/post_images/{post_id}")]
pub async fn upload_image(
    mut payload:Multipart,
    req:HttpRequest,
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> HttpResponse {

    let content_length:usize = match req.headers().get(CONTENT_LENGTH){
        Some(header_value) => {
            header_value.to_str().unwrap_or("0").parse().unwrap()
        },
        None =>"0".parse().unwrap()
    };
    let max_file_size:usize = 10_000_00;
    let legal_filetype:[Mime;3] = [IMAGE_JPEG,IMAGE_PNG,IMAGE_GIF];
    if content_length > max_file_size {
        return   HttpResponse::BadRequest().json("too.. large file");
    }
    if let Ok(Some( field)) = payload.try_next().await {
        let file_type = field.content_type();
        if file_type.is_none() {
            return  HttpResponse::BadRequest().json("file type not none");
        }
        if !legal_filetype.contains(&file_type.unwrap()) {
            return  HttpResponse::BadRequest().json("not legal file type");                
        }
        
        let file_data = field
            .into_stream()
            .try_fold(Vec::new(), |mut acc, buf| async move {
                acc.extend_from_slice(buf.chunk());
                Ok(acc)
            })
            .await.expect("folding error");

        let post_id = path.into_inner();
        let query_result = sqlx::query_as::<_,PostImage>(
            "INSERT INTO posts_image(post_id,image) VALUES($1,$2) RETURNING *"
        )
        .bind(post_id)
        .bind(file_data)
        .fetch_one(&data.db)
        .await;
        
        match query_result{
            Ok(_img)=>{
                return HttpResponse::Ok().json("success upload immage  ")
            },
            Err(err)=>{
                return HttpResponse::InternalServerError().json(format!("{:?}",err))
            }
        } 

    }

    HttpResponse::InternalServerError().into()
}


#[get("/get_image/{id}")]
pub async fn get_image(
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> HttpResponse {

    let image_id = path.into_inner();
    let query_result  = sqlx::query_as::<_,PostImage>(
        "SELECT * FROM posts_image WHERE id =$1"
    )
    .bind(image_id)
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(res)=>{
            HttpResponse::Ok().body(res.image)
        },
        Err(err)=>{
            HttpResponse::InternalServerError().json(format!("failed:{:?}",err))
        }
    }    

}

#[delete("/delete_image/{id}")]
pub async fn delete_image(
    path:web::Path<Uuid>,
    data:web::Data<AppState>
) -> HttpResponse {

    let image_id = path.into_inner();
    let query_result  = sqlx::query::<_>(
        "Delete FROM posts_image WHERE id =$1 RETURNING image"
    )
    .bind(image_id)
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(_res)=>{
            HttpResponse::Ok().json("Deleted successfully")
        },
        Err(err)=>{
            HttpResponse::InternalServerError().json(format!("failed:{:?}",err))
        }
    }    

}

// pub fn config(conf: &mut web::ServiceConfig){
//     let scope = web::scope("/post_images")
//         .service(upload_image)
//         .service(get_image)
//         .service(delete_image);
//     conf.service(scope);
// } 