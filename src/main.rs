extern crate iron;
extern crate router;
extern crate logger;

#[macro_use] extern crate log;

extern crate simplelog;

mod controllers;

use iron::prelude::*;
use logger::Logger;
use router::Router;

use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use std::fs::File;

use controllers::MuralsController;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Debug, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("log/app.log").unwrap()),
        ]
    ).unwrap();

    let mut router = Router::new();
    let (logger_before, logger_after) = Logger::new(None);

    router.get("/murals", MuralsController::list, "murals_list");

    let mut chain = Chain::new(router);

    // Link logger_before as your first before middleware.
    chain.link_before(logger_before);

    // Link logger_after as your *last* after middleware.
    chain.link_after(logger_after);

    info!("Listening on localhost:3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}
