use super::{Body, Error, HeaderMap, StatusCode, Url};
use crate::net::http::request::body::BodyOwned;
use crate::net::http::request::header::header::HeaderTrait;
use alloc::sync::Arc;
use serde::de::DeserializeOwned;

pub struct Response {
    raw: Arc<reqwest::Response>,
    status: StatusCode,
    headers: HeaderMap,
    content_length: Option<u64>,
    url: Url,
    body: BodyOwned,
}

impl Response {
    pub fn status(&self) -> &super::StatusCode {
        &self.status
    }
    pub fn to<T>(self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        todo!();
    }
}

impl From<reqwest::Response> for Response {
    fn from(raw: reqwest::Response) -> Self {
        let headers = raw.headers().into();
        let status = raw.status().into();
        let content_length = raw.content_length();
        let url = raw.url().as_ref().into();
        let arc_raw = Arc::new(raw);
        let body = BodyOwned::from(arc_raw.clone());
        let raw = arc_raw;
        Self {
            headers,
            status,
            content_length,
            url,
            body,
            raw,
        }
    }
}
