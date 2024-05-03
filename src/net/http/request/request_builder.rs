use super::Client;
#[cfg(feature = "net-signature")]
use crate::net::http::request::client::GLOBAL_SIGNING;
use crate::net::http::request::{Error, Header, HeaderMap, Response, Url};
#[cfg(feature = "net-signature")]
use crate::net::signature::Signature;
use crate::rails::ext::{RailsMapErrInto, RailsMapInto};
use tokio::join;

pub struct RequestBuilder {
    pub client: Option<Client>,
    pub method: Method,
    pub uri: Option<Url>,
    pub headers: HeaderMap,
    pub body: String,
    pub content_type: String,
    #[cfg(feature = "net-signature")]
    pub signature: Option<Signature>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            client: None,
            method: Method::None,
            uri: None,
            headers: HeaderMap::default(),
            body: "".to_string(),
            content_type: "".to_string(),
            #[cfg(feature = "net-signature")]
            signature: GLOBAL_SIGNING.read().clone(),
        }
    }

    pub fn new_with_client(client: Option<Client>, method: Method, uri: Url) -> Self {
        Self {
            client,
            method,
            uri: Option::from(uri),
            headers: HeaderMap::default(),
            body: "".to_string(),
            content_type: "".to_string(),
            #[cfg(feature = "net-signature")]
            signature: GLOBAL_SIGNING.read().clone(),
        }
    }

    pub fn method(&mut self, method: &str) -> &mut Self {
        self.method = Method::from(method);
        self
    }

    pub fn uri(&mut self, uri: &str) -> &mut Self {
        self.uri = Some(Url::from(uri));
        self
    }

    pub fn header<H: Into<Header>>(&mut self, header: H) -> &mut Self {
        self.headers.insert(header.into());
        self
    }

    pub fn body(&mut self, body: &str) -> &mut Self {
        self.body = body.to_string();
        self
    }

    pub fn content_type(&mut self, content_type: &str) -> &mut Self {
        self.headers.insert(("Content-Type", content_type).into());
        self
    }

    #[cfg(feature = "net-signature")]
    pub fn with_signing_default(&mut self) -> &mut Self {
        self.signature = Some(Signature::default());
        self
    }

    #[cfg(feature = "net-signature")]
    pub fn with_signing(&mut self, signature: Signature) -> &mut Self {
        self.signature = Some(signature);
        self
    }

    pub async fn send(&self) -> Result<Response, Error> {
        let uri: reqwest::Url = self.uri.as_ref().map(|t| t.into()).ok_or(Error::NoUrl)?;

        reqwest::Client::new()
            .request((&self.method).into(), uri)
            .headers((&self.headers).try_into().map_err_into::<Error>()?)
            .body(self.body.clone())
            .send()
            .await
            .map_err_into()
            .map_into()
    }

    // #[cfg(feature = "net-signature")]
    // async fn signature_builder(&self) -> Header {
    //     self.signature.and_then(|t| t.)
    // }
}

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Connect,
    Trace,
    None,
}

impl Method {}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "get" => Method::Get,
            "post" => Method::Post,
            "put" => Method::Put,
            "delete" => Method::Delete,
            "patch" => Method::Patch,
            "head" => Method::Head,
            "option" => Method::Options,
            "connect" => Method::Connect,
            "trace" => Method::Trace,
            _ => Method::None,
        }
    }
}
impl From<String> for Method {
    fn from(s: String) -> Self {
        Method::from(s.as_str())
    }
}

impl From<Method> for reqwest::Method {
    fn from(m: Method) -> Self {
        match m {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Patch => reqwest::Method::PATCH,
            Method::Head => reqwest::Method::HEAD,
            Method::Options => reqwest::Method::OPTIONS,
            Method::Connect => reqwest::Method::CONNECT,
            Method::Trace => reqwest::Method::TRACE,
            Method::None => reqwest::Method::GET,
        }
    }
}
impl From<&Method> for reqwest::Method {
    fn from(m: &Method) -> Self {
        match m {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Patch => reqwest::Method::PATCH,
            Method::Head => reqwest::Method::HEAD,
            Method::Options => reqwest::Method::OPTIONS,
            Method::Connect => reqwest::Method::CONNECT,
            Method::Trace => reqwest::Method::TRACE,
            Method::None => reqwest::Method::GET,
        }
    }
}
