use rocket::{Response, State};
use rocket::http::ContentType;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::dao::app_dao::AppDao;
use crate::model::request::AppRequest;
use crate::model::response::AppResponse;

#[post("/register_app", format = "json", data = "<request>")]
pub fn create(dao: State<AppDao>, request: Json<AppRequest>) -> Result<Json<AppResponse>, Response> {
    if dao.exists(request.name.clone()) {
        return Err(Response::build()
            .status(Status::Conflict)
            .header(ContentType::JSON)
            .finalize());
    }
    return match dao.create(request.name.clone()) {
        None => {
            Err(Response::build()
                .status(Status::InternalServerError)
                .header(ContentType::JSON)
                .finalize())
        }
        Some(id) => {
            Ok(Json(AppResponse { id }))
        }
    };
}