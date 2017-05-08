extern crate serde;
extern crate serde_json;

use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn list(_ : &mut Request) -> IronResult<Response> {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, serialized)))
}
