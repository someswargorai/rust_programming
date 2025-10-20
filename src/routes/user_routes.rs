use axum::{routing::{get,post}, Router};
use crate::{controller::user_controller::{create_user, foo_bar, get_foo, get_user_by_email, get_user_by_name, get_users, login, post_foo, register_user, root,update_user}};



pub fn create_app_routes() -> Router {
    
    let public_routes=Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:email", get(get_user_by_email))
        .route("/users/details", post(get_user_by_name))
        .route("/users/register", post(post(register_user)))
        .route("/users/update_user", post(post(update_user)));

        let user_routes=Router::new().route("/user/login", post(login));

        public_routes.merge(user_routes)   
}