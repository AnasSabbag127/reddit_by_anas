// use std::path::Path;

use actix_web::{web,post,HttpResponse,HttpRequest};
use actix_multipart::Multipart;
use mime::{Mime,IMAGE_PNG,IMAGE_JPEG, IMAGE_GIF};
use uuid::Uuid;
use image::{DynamicImage,imageops::FilterType};
use actix_web::http::header::CONTENT_LENGTH;
// use std::fs;
use tokio::fs;
use futures_util::TryStreamExt as _ ;
use tokio::io::AsyncWriteExt as _;

#[post("/upload")]
pub async fn upload_image(
    mut payload:Multipart,
    req:HttpRequest
)-> HttpResponse{
    
    let content_length:usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap()
    };
    let max_file_size =10_000_00;
    let legal_filetype:[Mime;3] = [IMAGE_JPEG,IMAGE_PNG,IMAGE_GIF];
    let dir:&str = "./upload_saved_image/";
    if content_length > max_file_size{
        return   HttpResponse::BadRequest().json("too.. large file");
    }
    if let Ok(Some(mut field)) = payload.try_next().await{
        
        let file_type = field.content_type();
        if file_type.is_none(){
            return  HttpResponse::BadRequest().json("file type not none");
        }
        if !legal_filetype.contains(&file_type.unwrap()){
            return  HttpResponse::BadRequest().json("not legal file type");                
        }
        
        let destination = format!(
            "{}-{}",
            dir,
            field.content_disposition().get_filename().unwrap()
        );
        let mut saved_file = fs::File::create(&destination).await.unwrap();
        while let Ok(Some(chunk)) = field.try_next().await {
            let _ = saved_file.write_all(&chunk).await.unwrap();
        }       
        web::block(move||async move{
            let uploaded_image = image::open(&destination).unwrap();
            let _  = fs::remove_file(&destination).await.unwrap();
            uploaded_image
                .resize_exact(200,200,FilterType::Gaussian)
                .save(format!("{}{}.gif",dir,Uuid::new_v4()))
                .unwrap();
        })
        .await
        .unwrap()
        .await;

        return HttpResponse::Ok().json("success");
    } 
    else {
        return  HttpResponse::InternalServerError().json("error from server");
    }
    
}
