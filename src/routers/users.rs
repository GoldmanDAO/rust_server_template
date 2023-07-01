use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
struct User {
    id: Uuid,
    name: String,
}

mod getters {
    use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
    use serde::Deserialize;
    use uuid::{uuid, Uuid};

    #[derive(Deserialize, Debug)]
    pub struct Id {
        pub id: Uuid,
    }

    pub async fn get_user(id: Query<Id>) -> Result<impl IntoResponse, StatusCode> {
        let Query(id) = id;
        println!("ID: {:?}", id);

        if id.id == uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8") {
            let user = super::User {
                id: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
                name: String::from("Mario"),
            };
            Ok(Json(user))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub use getters::get_user;
