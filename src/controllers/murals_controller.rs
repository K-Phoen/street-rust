use iron::prelude::*;
use iron::status;

pub fn list(_ : &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "here is a list of saved murals... or not.")))
}
