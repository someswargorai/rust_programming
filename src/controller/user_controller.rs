use axum::{extract::{Path}, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{EncodingKey, Header, encode};
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use futures_util::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use crate::db::db_connection;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Debug, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Clone)]
pub struct RegisterClaims<'a> {
    pub sub: &'a Register,
    pub exp: usize,
}

impl User {
    pub fn initialize() -> Self {
        let user = User {
            id: Uuid::new_v4(),
            name: String::from("somewargorai"),
            email: String::from("somewar@klizos.com"),
        };

        user
    }
    pub fn capitalize(&self) -> Self {
        let first_letter = self.email[0..1].to_string().to_uppercase();
        let rest = self.email[1..].to_string().to_uppercase();
        let concat = format!("{}{}", first_letter, rest);
        let user = User {
            email: concat,
            ..self.clone()
        };

        user
    }
}

#[derive(Serialize, Clone, Deserialize)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserInput {  
    pub name: String,
    pub email: Option<String>,
  
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
}

#[derive(Deserialize)]
pub struct RegisterInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize,Deserialize)]
#[derive(Debug)]
pub struct UserDocument {

    pub _id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize,Deserialize)]
#[derive(Debug)]
pub struct InsertUserDocument {
     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String
}


#[derive(Serialize, Deserialize, Clone)]

pub struct UpdatePayload{
    pub _id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String
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
    // Get the database
    let db = db_connection::get_db().await;
    let collection: Collection<UserDocument> = db.collection("users");

    // Find all users
    let mut cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            eprintln!("Error querying users: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ).into_response();
        }
    };

    let mut users: Vec<UserDocument> = Vec::new();

    while let Some(user) = match cursor.try_next().await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Error iterating cursor: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ).into_response();
        }
    } {
        println!("Found user: {:?}", user); // Print each user
        users.push(user);
    }

    // Handle empty result
    if users.is_empty() {
        println!("No users found in the collection");
        return (
            StatusCode::OK, // or StatusCode::NOT_FOUND
            Json(json!({"message": "No users found", "users": []})),
        ).into_response();
    }
    // Return users as JSON
    (StatusCode::OK, Json(json!({"users":users})).into_response()).into_response()
}

pub async fn get_user_by_email(Path(email): Path<String>) -> impl IntoResponse {
    // Get the database
    let db = db_connection::get_db().await;
    let collection: Collection<UserDocument> = db.collection("users");

    // Create filter for email
    let filter = doc! {"email": email.clone()};

    // Find user by email
    let user_by_id = match collection.find_one(filter, None).await {
        Ok(Some(user)) => {
            println!("Found user: {:?}", user); // Print for debugging
            user
        }
        Ok(None) => {
            println!("No user found for email: {}", email); // Print for debugging
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Not Found"})),
            ).into_response();
        }
        Err(e) => {
            eprintln!("Error querying user: {:?}", e); // Log error for debugging
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ).into_response();
        }
    };

    (StatusCode::OK, Json(user_by_id)).into_response()
}

pub async fn create_user(Json(payload): Json<UpdatePayload>) -> impl IntoResponse {
    let db = db_connection::get_db().await;
    let collection: Collection<InsertUserDocument> = db.collection("users");
    let filter= doc! {"email": &payload.email};
    
    let result = match collection.find_one(filter, None).await{
        Ok(Some(user))=>{
             return (
                StatusCode::FOUND,
                Json(json!({"message":"Already exists user with this mail","user":user}))
            ).into_response()
        },
        Ok(None)=>{
           
        },
        Err(_)=>{
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    println!("227 {:?}",result);
    
    
    let hashed_password = match hash(&payload.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(e) => {
            eprintln!("Error hashing password: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to hash password"})),
            ).into_response();
        }
    };

    let user = InsertUserDocument {
        _id: None,
        email: payload.email,
        password: hashed_password,
        name: payload.name
    };

    let result = match collection.insert_one(user, None).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error inserting user: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ).into_response();
        }
    };

    // Fetch the inserted document using the inserted_id
    let inserted_id = result.inserted_id;
    let inserted_user = match collection.find_one(doc! { "_id": &inserted_id }, None).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            println!("Inserted user not found for id: {:?}", inserted_id);
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Inserted user not found"})),
            ).into_response();
        }
        Err(e) => {
            eprintln!("Error fetching inserted user: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ).into_response();
        }
    };

    println!("Inserted user: {:?}", inserted_user);

    (
        StatusCode::OK,
        Json(json!({
            "message": "User inserted successfully",
            "user": inserted_user
        })),
    ).into_response()
}

pub async fn get_user_by_name(Json(payload): Json<UserInput>) -> impl IntoResponse {
    println!("{:?}", User::initialize());

    let searched_user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email.unwrap_or(String::from("segseg@klizos.com")),
    };

    let uppercased_user = User::capitalize(&searched_user);

    (StatusCode::OK, Json(uppercased_user))
}

pub async fn login(Json(payload): Json<LoginInput>) -> impl IntoResponse {
   // Replace "mydb" with your database name

    let db = db_connection::get_db().await;
    let collection: Collection<UserDocument>=db.collection("users");

    let email = doc! {"email": &payload.email};

    let user: UserDocument = match collection.find_one(email, None).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message":"Unauthorized User!!"})),
            )
                .into_response();
        }
        Err(_) => return Json(json!({"message":"Interval server error"})).into_response(),
    };

    let claims = Claims {
        sub: user.email,
        exp: (chrono::Utc::now() + chrono::Duration::hours(12)).timestamp() as usize,
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    ) {
        Ok(token) => (
            StatusCode::OK,
            Json(json!({"token": token, "message":"login successful"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to encode token: {}", e)})),
        )
            .into_response(),
    }
}

pub async fn register_user(Json(payload): Json<RegisterInput>) -> impl IntoResponse {
    // MongoDB connection (replace with your connection string)
    let client = match Client::with_uri_str("mongodb://localhost:27017").await {
        Ok(client) => client,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to connect to MongoDB"})),
            )
                .into_response();
        }
    };

    // Get the collection (equivalent to a table in SQL)
    let db = client.database("mydb"); // Replace "mydb" with your database name
    let collection: Collection<Register> = db.collection("users"); // Replace "users" with your collection name

    // Create the user object
    let registered_user = Register {
        name: payload.name,
        email: payload.email,
        password: payload.password, // Note: In production, hash the password!
    };

    // Check if user with the same email exists
    let filter = doc! {"email": &registered_user.email};
    if collection
        .find_one(filter, None)
        .await
        .unwrap_or(None)
        .is_some()
    {
        return (
            StatusCode::CONFLICT,
            Json(json!({"message": "User already exists"})),
        )
            .into_response();
    }

    // Insert the new user into MongoDB
    match collection.insert_one(registered_user.clone(), None).await {
        Ok(_) => (),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to register user"})),
            )
                .into_response();
        }
    }

    // Create JWT claims
    let register_claims = RegisterClaims {
        sub: &registered_user,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    // Generate JWT
    match encode(
        &Header::default(),
        &register_claims,
        &EncodingKey::from_secret("rust_register".as_ref()),
    ) {
        Ok(token) => (
            StatusCode::OK,
            Json(json!({
                "token": token,
                "message": "User registered successfully!",
                "data": registered_user
            })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to generate token"})),
        )
            .into_response(),
    }
}

pub async fn update_user(Json(payload): Json<UpdatePayload>) -> impl IntoResponse {
    let db = db_connection::get_db().await;
    let collection: Collection<UserDocument> = db.collection("users");
    let filter = doc! { "_id": &payload._id };
    let update = doc! { "$set": { "password": &payload.password, "email": &payload.email, "name": &payload.name } };

    let result = match collection.find_one_and_update(filter, update, None).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            println!("No user found for email: {}", &payload.email);
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "User not found"})),
            ).into_response();
        }
        Err(e) => {
            eprintln!("Error updating user: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ).into_response();
        }
    };

    println!("Updated user: {:?}", result);

    (
        StatusCode::OK,
        Json(json!({
            "message": "User updated successfully",
            "user": result
        })),
    ).into_response()
}