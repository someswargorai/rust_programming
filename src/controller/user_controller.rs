use axum::{Json, extract::Path, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

// -------------------- MODELS --------------------
#[derive(Serialize, Clone)] 
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: Option<String>,
}

// -------------------- ROUTE HANDLERS --------------------

// ✅ GET / (Root)
pub async fn root() -> impl IntoResponse {
    Json(json!({"message": "Welcome to the API!", "version": "1.0"}))
}

// ✅ GET /foo
pub async fn get_foo() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"foo": "get", "method": "GET"})))
}


// ✅ POST /foo
pub async fn post_foo() -> impl IntoResponse {
    Json(json!({"foo": "post", "method": "POST"}))
}

// ✅ GET /foo/bar
pub async fn foo_bar() -> impl IntoResponse {
    Json(json!({"foo": "bar", "message": "Foo bar route!"}))
}

// ✅ GET /users
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

// ✅ FIXED: GET /users/:id
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

// ✅ POST /users
pub async fn create_user(Json(payload): Json<UserInput> ) -> impl IntoResponse {
    let user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email.unwrap_or(String::from("user@gmail.com")),
    };
    // ✅ FIX 2: Add 201 status
    (StatusCode::CREATED, Json(user))
}

pub async fn get_user_by_name(Json(payload): Json<UserInput>)-> impl IntoResponse {

    let searched_user= User{
        id: Uuid::new_v4(),
        name: payload.name,
        email:payload.email.unwrap_or(String::from("segseg@klizos.com"))
    };

   
    (StatusCode::OK, Json(searched_user))
}