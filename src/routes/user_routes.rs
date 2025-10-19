use axum::{http::StatusCode, middleware, routing::{get, head, post}, Json, Router};
use jsonwebtoken::{decode, DecodingKey, Header, Validation};
use crate::controller::user_controller::{get_foo, post_foo, foo_bar,get_user_by_id , get_users, create_user, root, get_user_by_name, login};
use axum::http::Request;
use axum::response::IntoResponse;
use serde_json::json;
use serde::{Deserialize, Serialize}; 

async fn auth_middleware(req: Request<axum::body::Body>, next: middleware::Next) -> impl IntoResponse {
   if let Some(auth) = req.headers().get("Authorization") {
        println!("🔐 Token: {:?}", auth);
        // Add JWT decode here!
        

        next.run(req).await
    } else {
        (StatusCode::UNAUTHORIZED, Json("No token!")).into_response()
    }
    
}

#[derive(Serialize, Clone,Deserialize)] 
#[derive(Debug)]
struct Claims {
    sub: String,
    exp: usize,
    // Add other claims as needed
}


async fn check_token_middleware(
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

// ✅ NEW: Add this if you want the root + foo routes
pub fn create_app_routes() -> Router {
    
    let public_routes=Router::new()
        .route("/", get(root).layer(middleware::from_fn(auth_middleware)))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user_by_id))
        .route("/users/details", post(get_user_by_name));

        let user_routes=Router::new().route("/user/login", post(login).layer(middleware::from_fn(check_token_middleware)));

        public_routes.merge(user_routes)    
}