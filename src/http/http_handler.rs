use crate::http::http_request::Request;
use crate::http::http_response::Response;

pub trait Handler {
    fn service(&self, rq: Request, response: Response);
}