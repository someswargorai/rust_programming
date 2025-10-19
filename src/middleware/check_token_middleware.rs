use axum::{extract::Request, middleware, response::IntoResponse, Json};
use hyper::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use serde_json::json;
use serde::{Deserialize}; 

#[derive(Serialize, Clone, Deserialize)] 
#[derive(Debug)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn check_token_middleware(
    req: Request<axum::body::Body>,
    next: middleware::Next,
) -> impl IntoResponse {

    match req.headers().get("Authorization") {
      
      Some(auth_header)=>{

        let auth_str= auth_header.to_str();

        match auth_str {

            Ok(tok) => {
                let token_str = tok.replace("Bearer ", "");
                
                if token_str.is_empty() || token_str == tok {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({ "error": "Authorization header must contain a valid Bearer token" })),
                    ).into_response();
                }
                println!("Token: {}", token_str);

                let mut validation = Validation::default();
                validation.validate_exp = true;

                match decode::<Claims>(
                    &token_str,
                    &DecodingKey::from_secret("secret".as_ref()),
                    &validation,
                ) {
                    Ok(token_data) => {
                        println!("✅ Decoded claims: {:?}", token_data);
                        let mut req = req;
                        req.extensions_mut().insert(token_data);
                        next.run(req).await
                    }
                    Err(e) => (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "error": format!("Failed to decode token: {}", e) })),
                    ).into_response(),
                }
            }
            Err(_) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid Authorization header format" })),
            ).into_response(),
        }
      },
      None=>{
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "No token provided" })),
        ).into_response()
    }
}
}