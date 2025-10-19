use std::fs::{self};

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

#[derive(Serialize, Clone)] 
pub struct RegisterClaims<'a>{
    pub sub: &'a Register,
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

#[derive(Serialize, Clone, Deserialize)] 
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct RegisterInput{
    pub name: String,
    pub email: String,
    pub password:String
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



pub async fn login(Json(payload): Json<UserInput>) -> impl IntoResponse {

let name = &payload.name;
let claims = Claims {
    sub: name.clone(),
    exp: (chrono::Utc::now() + chrono::Duration::hours(12)).timestamp() as usize,
};

match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
    Ok(token) => (StatusCode::OK, Json(json!({"token": token, "message":"login successful"}))).into_response(),
    Err(e) => (
        StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to encode token: {}", e)})),
        ).into_response(),
    }
}

pub async fn register_user(Json(payload): Json<RegisterInput>) -> impl IntoResponse {

    let mut vec: Vec<Register> = match fs::read_to_string("db.json"){
        Ok(item)=> match serde_json::from_str(&item){
            Ok(data)=>data,
            Err(_)=>Vec::new()
        }
        Err(_)=> Vec::new()
    };
      

    let registered_user = Register {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };

    if vec.iter().any(|f| f.email == registered_user.email) {
        return (StatusCode::CONFLICT, Json(json!({"message": "User already exists"}))).into_response();
    }

    vec.push(registered_user.clone());

    let register_claims = RegisterClaims {
        sub: &registered_user,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    fs::write(
        String::from("db.json"),
        serde_json::to_string_pretty(&vec).unwrap(),
    ).unwrap();

    match encode(
        &Header::default(),
        &register_claims,
        &EncodingKey::from_secret("rust_register".as_ref()),
    ) {
        Ok(token) => 
        
        return (
            StatusCode::OK,
            Json(json!({"token": token, "message": "User registered successful!", "data": registered_user})),
        ).into_response(),

        Err(_) => 
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to generate token"})),
        ).into_response(),

    }
}