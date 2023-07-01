pub mod foo;
pub mod root;
pub mod users;

pub async fn foo_bar() -> &'static str {
    "Hello from subpath"
}
