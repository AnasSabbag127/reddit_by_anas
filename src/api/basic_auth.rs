use actix_web::{
    get,
    web::Data,
    HttpResponse,Responder
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::Verifier;
use hmac::{Hmac,Mac};
use jwt::SignWithKey;
use sha2::Sha256;

use crate::{
    AppState,TokenClaims,
   model::users::AccountUser
};

#[get("/auth")]//this will return token... 
async fn basic_auth(
    state:Data<AppState>,
    credentials:BasicAuth
)-> impl Responder{

    let jwt_secret:Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set !")
        .as_bytes()
    ).unwrap();
    let username = credentials.user_id();
    let password = credentials.password();

    match password{
        None => {
            HttpResponse::Unauthorized().json("password must be set")
        },
        Some(pass) => {
            let query_result = sqlx::query_as::<_,AccountUser>(
                "SELECT id,username,password FROM account_user WHERE username = $1"
            )
            .bind(username.to_string())
            .fetch_one(&state.db)
            .await;
            match query_result{
                Ok(user)=>{
                    let hash_secret = std::env::var("HASH_SECRET")
                        .expect("HASH SECRET must be set");
                    let mut verifier = Verifier::default();

                    let is_valid = verifier
                        .with_hash(user.password.unwrap())
                        .with_password(pass)
                        .with_secret_key(hash_secret)
                        .verify()
                        .unwrap();
                    
                    if is_valid {
                        let claims = TokenClaims{id:user.id};
                        let token_str = claims.sign_with_key(
                            &jwt_secret
                        ).unwrap();
                        HttpResponse::Ok().json(token_str)
                    }else{
                        HttpResponse::Unauthorized().json("Incorrect Username or Password")
                    }
                },
                Err(err)=>{
                    HttpResponse::InternalServerError().json(format!("here is the error {:?}",err))
                }

            }

        }
    }
}
