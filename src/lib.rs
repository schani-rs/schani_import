extern crate amq_protocol;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate futures;
extern crate hyper;
extern crate lapin_futures as lapin;
#[macro_use]
extern crate log;
extern crate resolve;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tokio_core;

mod database;
mod messaging;
mod models;
mod service;
mod web;
