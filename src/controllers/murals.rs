extern crate serde;
extern crate serde_json;

use iron::headers::ContentType;
use iron::prelude::{IronResult, Request, Response};
use iron::status;

#[derive(Serialize, Deserialize, Debug)]
struct Coordinates {
    lat: f32,
    long: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mural {
    coord: Coordinates,
    title: String,
}

pub fn list(_ : &mut Request) -> IronResult<Response> {
    let mural = Mural {
        coord: Coordinates { lat: 5.55, long: 6.66 },
        title: String::from("Some awesome mural")
    };
    let other_mural = Mural {
        coord: Coordinates { lat: 5.55, long: 6.66 },
        title: String::from("Some awesome mural")
    };

    let murals = vec![mural, other_mural];

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&murals).unwrap();

    Ok(Response::with((ContentType::json().0, status::Ok, serialized)))
}
