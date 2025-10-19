use axum::{Json, extract::Path, response::IntoResponse, http::StatusCode};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
#[derive(Debug)]
#[derive(Serialize, Clone)] 
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Clone)] 
pub struct Claims{
    pub sub: String,
    pub exp: usize
}

impl User {
    
    pub fn initialize()->Self{
        let user = User{
            id: Uuid::new_v4(),
            name: String::from("somewargorai"),
            email: String::from("somewar@klizos.com")
        };

        user
    }
    pub fn capitalize(&self)-> Self{
        
        let first_letter=self.email[0..1].to_string().to_uppercase();
        let rest=self.email[1..].to_string().to_uppercase();
        let concat=format!("{}{}",first_letter,rest);
        let user=User{
            email:concat,
            ..self.clone()
        };

        user
    }

}

#[derive(Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: Option<String>,
}

pub async fn login(Json(payload): Json<UserInput>) -> impl IntoResponse {
    let name = &payload.name;
    let claims = Claims {
        sub: name.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(12)).timestamp() as usize,
    };

    match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
        Ok(token) => (StatusCode::OK, Json(json!({"token": token}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to encode token: {}", e)})),
        ),
    }
}

pub async fn root() -> impl IntoResponse {
    Json(json!({"message": "Welcome to the API!", "version": "1.0"}))
}

pub async fn get_foo() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"foo": "get", "method": "GET"})))
}

pub async fn post_foo() -> impl IntoResponse {
    Json(json!({"foo": "post", "method": "POST"}))
}

pub async fn foo_bar() -> impl IntoResponse {
    Json(json!({"foo": "bar", "message": "Foo bar route!"}))
}

pub async fn get_users() -> impl IntoResponse {
    let users = vec![
        User {
            id: Uuid::new_v4(),
            name: "Alice".into(),
            email: "alice@example.com".into(),
        },
        User {
            id: Uuid::new_v4(),
            name: "Bob".into(),
            email: "bob@example.com".into(),
        },
    ];
    Json(users)
}

pub async fn get_user_by_id(Path(id): Path<String>) -> impl IntoResponse {
    match Uuid::parse_str(&id) {
        Ok(uuid) => {
            Json(User {
                id: uuid,
                name: "Placeholder User".to_string(),
                email: "placeholder@example.com".to_string(),
            })
        }
        Err(_) => {
           Json(User {
                id: Uuid::nil(),
                name: "ERROR".to_string(),
                email: "Invalid UUID".to_string(),
            })
        }
    }
}


pub async fn create_user(Json(payload): Json<UserInput> ) -> impl IntoResponse {
    let user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email.unwrap_or(String::from("user@gmail.com")),
    };
    
    (StatusCode::CREATED, Json(user))
}

pub async fn get_user_by_name(Json(payload): Json<UserInput>)-> impl IntoResponse {

    println!("{:?}",User::initialize());

    let searched_user= User{
        id: Uuid::new_v4(),
        name: payload.name,
        email:payload.email.unwrap_or(String::from("segseg@klizos.com"))
    };

    let uppercased_user=User::capitalize(&searched_user);
   
    (StatusCode::OK, Json(uppercased_user))
}
