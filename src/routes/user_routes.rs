use axum::{routing::{get,post}, Router};
use crate::controller::user_controller::{get_foo, post_foo, foo_bar,get_user_by_id , get_users, create_user, root, get_user_by_name};

// // ✅ FIXED: This is your MAIN function - keep this!
// pub fn create_user_routes() -> Router {
//     Router::new()
//         .route("/users", get(get_users).post(create_user))
//         .route("/users/:id", get(get_user_by_id))
// }

// ✅ NEW: Add this if you want the root + foo routes
pub fn create_app_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user_by_id))
        .route("/users/details", post(get_user_by_name))
}