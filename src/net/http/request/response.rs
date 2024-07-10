use super::{Error, HeaderMap, StatusCode, Url};
use crate::net::http::request::body::BodyOwned;
use alloc::boxed::Box;
use core::result::Result;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    #[allow(unused)]
    headers: HeaderMap,
    #[allow(unused)]
    content_length: Option<u64>,
    #[allow(unused)]
    url: Url,
    #[allow(unused)]
    body: BodyOwned,
}

impl Response {
    pub fn status(&self) -> &super::StatusCode {
        &self.status
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn body(&self) -> &BodyOwned {
        &self.body
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
        let box_raw = Box::new(raw);
        let body = BodyOwned::from(box_raw);
        Self {
            headers,
            status,
            content_length,
            url,
            body,
        }
    }
}
