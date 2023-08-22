use crate::http::{Request, Response, StatusCode};
use super::server::Handler;

pub struct Websitehandler;

impl Handler for Websitehandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>Great Success!</h1>".to_string()))
    }
}