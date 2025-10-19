use axum::{handler::Handler, middleware, routing::{get,post}, Router};
use crate::{controller::user_controller::{create_user, foo_bar, get_foo, get_user_by_id, get_user_by_name, get_users, login, post_foo, root, register_user}, middleware::check_token_middleware};

pub fn create_app_routes() -> Router {
    
    let public_routes=Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
        .route("/users", get(get_users.layer(middleware::from_fn(check_token_middleware::check_token_middleware))).post(create_user))
        .route("/users/:id", get(get_user_by_id))
        .route("/users/details", post(get_user_by_name))
        .route("/users/register", post(post(register_user)));

        let user_routes=Router::new().route("/user/login", post(login).layer(middleware::from_fn(check_token_middleware::check_token_middleware)));

        public_routes.merge(user_routes)   
}