#![allow(unused_variables)]

#[macro_use]
extern crate rocket;

use rocket::http::Method;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Value};
use rocket_cors::{AllowedOrigins, CorsOptions};

pub mod functions;

/*
curl --request POST http://localhost:8000/
*/
#[get("/")]
fn execute_js_function() -> status::Custom<Value> {
    let resp = functions::execute_js();
    match resp {
        Ok(result) => return status::Custom(Status::Ok, json!({"result": true, "error": ""})),
        Err(err) => {
            return status::Custom(
                Status::BadRequest,
                json!({"message": "Execution error", "errorCode": "execution_error"}),
            );
        }
    }
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("CORS failed to build");

    rocket::build()
        .mount("/", routes![execute_js_function])
        .attach(cors)
}
