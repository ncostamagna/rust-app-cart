use rocket::{
    response::status::Created,
    serde::json::Json,
};

#[derive(Serialize)]
pub struct Get {
    pub title: String,
    pub price: Currency,
}

#[rocket::get("/")]
pub async fn get(app: AppRequest<'_>) -> Result<Json<Get>, Error> {
    app.transaction(|app| async move {
        Ok(Json(Get {
            title: String::from("title"),
            price: 123,
        }))
    })
    .await
}