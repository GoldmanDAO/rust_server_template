use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct User {
    id: Uuid,
    name: String,
}

pub mod getters;

pub use getters::get_user;
