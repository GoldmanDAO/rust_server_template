use axum::{routing::get, Router};

mod routers;

#[tokio::main]
async fn main() {
    // build our application with a single route

    let app = Router::new()
        .route("/", get(routers::root::get))
        .route("/user", get(routers::users::get_user))
        .route(
            "/foo",
            get(routers::foo::get_foo).post(routers::foo::post_foo),
        )
        .route("/foo/bar", get(routers::foo_bar));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
