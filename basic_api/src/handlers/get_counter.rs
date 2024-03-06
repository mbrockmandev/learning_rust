use super::utils::update_counter;

#[get("/id/count")]
pub async fn get_counter() -> Result<Json<CounterData>, Outcome> {
    match update_counter() {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(Status::InternalServerError.with_msg(err.to_string())),
    }
}
