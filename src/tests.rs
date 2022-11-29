use super::rocket;

use rocket::http::{RawStr, Status, Method::*};
use rocket::local::blocking::Client;
use rocket_dyn_templates::{Template, context};


#[test]
fn test_index() {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
