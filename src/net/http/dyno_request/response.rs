use crate::net::http::dyno_request::header::HeaderMap;
use crate::net::http::dyno_request::{status_code, Matcher};

pub struct ExpectedResponses(pub Vec<ExpectedResponse>);

pub struct ExpectedResponse {
    pub status_code: status_code::StatusCode,
    pub headers: HeaderMap,
    pub body: Option<ResponseBody>,
}

pub struct ResponseBody {
    pub example: String,
    pub matcher: Matcher,
}
