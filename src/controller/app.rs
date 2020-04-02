use rocket::State;
use rocket_contrib::json::{JsonValue, Json};
use crate::model::request::AppRequest;
use crate::dao::app_dao::AppDao;

#[post("/register_app", format = "json", data = "<request>")]
pub fn create(dao: State<AppDao>, request: Json<AppRequest>) -> JsonValue {
    if dao.exists(request.0.name.clone()) {
        return json!({ "status": "NOK" });
    }
    return match dao.create(request.0.name.clone()) {
        None => { json!({ "status": "NOK" }) }
        Some(id) => {
            json!({ "status":id.as_str()})
        }
    };
}