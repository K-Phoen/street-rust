extern crate iron;
extern crate router;
extern crate logger;

#[macro_use] extern crate log;

extern crate simplelog;

use iron::prelude::*;
use iron::status;
use logger::Logger;
use router::Router;

use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use std::fs::File;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Debug, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("log/app.log").unwrap()),
        ]
    ).unwrap();

    let mut router = Router::new();
    let (logger_before, logger_after) = Logger::new(None);

    router.get("/", handler, "index");
    router.get("/:query", handler, "query");

    let mut chain = Chain::new(router);

    // Link logger_before as your first before middleware.
    chain.link_before(logger_before);

    // Link logger_after as your *last* after middleware.
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}
