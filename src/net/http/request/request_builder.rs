//! A comprehensive builder for constructing HTTP requests.
//!
//! This module provides the `Builder` struct, which allows for a flexible and
//! powerful way to create HTTP requests. The `Builder` struct supports setting
//! various aspects of the request such as the method, URL, headers, body, and
//! content type. Additionally, it includes features for signing requests when
//! the `net-signature` feature is enabled.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```rust,no_run
//! use gearbox::net::http::request::{Builder, Method, Url};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), gearbox::net::http::request::Error> {
//!     let url = Url::from("https://example.com");
//!     let response = Builder::GET
//!         .url(url)
//!         .send()
//!         .await?;
//!
//!     println!("Response: {:?}", response);
//!     Ok(())
//! }
//! ```
//!
//! Setting headers and body:
//!
//! ```rust,no_run
//! use gearbox::net::http::request::{Builder, Method, HeaderMap, Header, Url};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), gearbox::net::http::request::Error> {
//!     let url = Url::from("https://example.com");
//!     let mut headers = HeaderMap::default();
//!     headers.insert(("Content-Type", "application/json").into());
//!     let response = Builder::POST
//!         .url(url)
//!         .headers(headers)
//!         .body(r#"{"key": "value"}"#)
//!         .send()
//!         .await?;
//!
//!     println!("Response: {:?}", response);
//!     Ok(())
//! }
//! ```

use super::{Body, Client};
use crate::net::http::request::body::BodyOwned;
#[cfg(feature = "net-signature")]
use crate::net::http::request::client::GLOBAL_SIGNING;
use crate::net::http::request::{Error, Header, HeaderMap, Response, Url};

#[cfg(feature = "net-signature")]
use crate::net::signature::Signature;

use crate::error::DynTracerError;
use crate::net::http::request::header::values::Values;
use crate::net::http::request::header::Name;
use crate::rails::ext::syn::{RailsMapErrInto, RailsMapInto};
use alloc::{string::String, sync::Arc};
use core::fmt;
use core::future::Future;
use crate_serde::ser::{self, SerializeStruct};
use crate_serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "net-signature")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone)]
/// A builder for constructing HTTP requests with optional signing capability.
pub struct Builder {
    client: Option<Arc<Client>>,
    method: Method,
    url: Option<Url>,
    headers: Option<HeaderMap>,
    body: Option<BodyOwned>,
    content_type: Option<String>,
    signature: Option<Signature>,
}

#[cfg(not(feature = "net-signature"))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone)]
/// A builder for constructing HTTP requests.
pub struct RequestBuilder {
    client: Option<Arc<Client>>,
    method: Method,
    uri: Option<Url>,
    headers: HeaderMap,
    body: None,
    content_type: String,
}

impl Builder {
    /// A constant for GET requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::GET;
    /// assert_eq!(builder.get_method(), Method::Get);
    /// ```
    pub const GET: Builder = Self::new(Method::Get);
    /// A constant for POST requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::POST;
    /// assert_eq!(builder.get_method(), Method::Post);
    /// ```
    pub const POST: Builder = Self::new(Method::Post);
    /// A constant for PUT requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::PUT;
    /// assert_eq!(builder.get_method(), Method::Put);
    /// ```
    pub const PUT: Builder = Self::new(Method::Put);
    /// A constant for DELETE requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::DELETE;
    /// assert_eq!(builder.get_method(), Method::Delete);
    /// ```
    pub const DELETE: Builder = Self::new(Method::Delete);
    /// A constant for PATCH requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::PATCH;
    /// assert_eq!(builder.get_method(), Method::Patch);
    /// ```
    pub const PATCH: Builder = Self::new(Method::Patch);
    /// A constant for HEADER requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::HEADER;
    /// assert_eq!(builder.get_method(), Method::Head);
    /// ```
    pub const HEADER: Builder = Self::new(Method::Head);
    /// A constant for OPTIONS requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::OPTIONS;
    /// assert_eq!(builder.get_method(), Method::Options);
    /// ```
    pub const OPTIONS: Builder = Self::new(Method::Options);
    /// A constant for CONNECT requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::CONNECT;
    /// assert_eq!(builder.get_method(), Method::Connect);
    /// ```
    pub const CONNECT: Builder = Self::new(Method::Connect);
    /// A constant for TRACE requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::TRACE;
    /// assert_eq!(builder.get_method(), Method::Trace);
    /// ```
    pub const TRACE: Builder = Self::new(Method::Trace);

    /// Creates a new `Builder` with the specified HTTP method.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method for the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::new(Method::Get);
    /// assert_eq!(builder.get_method(), Method::Get);
    /// ```
    pub const fn new(method: Method) -> Self {
        Self {
            client: None,
            method,
            url: None,
            headers: None,
            body: None,
            content_type: None,
            #[cfg(feature = "net-signature")]
            signature: None,
        }
    }

    /// Creates a new `Builder` with the specified client, method, and URL.
    ///
    /// # Arguments
    ///
    /// * `client` - An optional HTTP client to be used for the request.
    /// * `method` - The HTTP method for the request.
    /// * `uri` - The URL for the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Client, Method, Url};
    ///
    /// let client = Client::new();
    /// let url = Url::from("https://example.com");
    /// let builder = Builder::new_with_client(Some(client), Method::Get, url);
    /// assert_eq!(builder.get_method(), Method::Get);
    /// ```
    pub fn new_with_client(client: Option<Client>, method: Method, uri: Url) -> Self {
        Self {
            client: client.map(Arc::new),
            method,
            url: Option::from(uri),
            headers: None,
            body: None,
            content_type: None,
            #[cfg(feature = "net-signature")]
            signature: GLOBAL_SIGNING.read().clone(),
        }
    }

    /// Sets the content type of the request.
    ///
    /// # Arguments
    ///
    /// * `content_type` - A string slice that holds the content type of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Builder;
    ///
    /// let builder = Builder::GET.content_type("application/json");
    /// ```
    pub fn content_type(mut self, content_type: &str) -> Self {
        self.headers
            .get_or_insert(HeaderMap::default())
            .insert(("Content-Type", content_type).into());
        self
    }

    /// Enables signing for the request with a default signature.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Builder;
    ///
    /// let builder = Builder::GET.with_signing_default();
    /// ```
    #[cfg(feature = "net-signature")]
    pub fn with_signing_default(mut self) -> Self {
        self.signature = Some(Signature::default());
        self
    }

    /// Enables signing for the request with the specified signature.
    ///
    /// # Arguments
    ///
    /// * `signature` - A `Signature` struct that holds the signature for the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder};
    /// use gearbox::net::Signature;
    ///
    /// let signature = Signature::default();
    /// let builder = Builder::GET.with_signing(signature);
    /// ```
    #[cfg(feature = "net-signature")]
    pub fn with_signing(mut self, signature: Signature) -> Self {
        self.signature = Some(signature);
        self
    }

    /// Sends the constructed request and returns the response.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the request could not be constructed or sent.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use gearbox::net::http::request::{Builder, Url};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), gearbox::net::http::request::Error> {
    ///     let url = Url::from("https://example.com");
    ///     let response = Builder::GET
    ///         .url(url)
    ///         .send()
    ///         .await?;
    ///
    ///     println!("Response: {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub async fn send(mut self) -> Result<Response, Error> {
        let uri: reqwest::Url = self.url.as_ref().map(|t| t.into()).ok_or(Error::NoUrl)?;
        let request = (&self.method).into();
        let headers = self
            .headers
            .get_or_insert(HeaderMap::default())
            .clone()
            .try_into()
            .map_err_into::<Error>()?;
        let body = self
            .body
            .get_or_insert(BodyOwned::default())
            .into_bytes()
            .await
            .map_err(Error::BodyError)?;

        reqwest::Client::new()
            .request(request, uri)
            .headers(headers)
            .body(body)
            .send()
            .await
            .map_err_into()
            .map_into()
    }
}

impl Builder {
    /// Sets the HTTP method of the request.
    ///
    /// # Arguments
    ///
    /// * `method` - A type that can be converted into `Method`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::GET.method(Method::Post);
    /// assert_eq!(builder.get_method(), Method::Post);
    /// ```
    pub fn method<T: Into<Method>>(mut self, method: T) -> Self {
        self.method = method.into();
        self
    }

    /// Gets a mutable reference to the HTTP method of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let mut builder = Builder::GET;
    /// let method = builder.method_mut();
    /// *method = Method::Post;
    /// assert_eq!(builder.get_method(), Method::Post);
    /// ```
    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }

    /// Adds a header to the request.
    ///
    /// # Arguments
    ///
    /// * `header` - A type that can be converted into `Header`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Header};
    ///
    /// let mut builder = Builder::GET;
    /// builder.header(("Content-Type", "application/json"));
    /// ```
    pub fn header<H: Into<Header>>(mut self, header: H) -> Self {
        self.headers
            .get_or_insert(HeaderMap::default())
            .insert(header.into());
        self
    }

    /// Gets a mutable reference to a header by name.
    ///
    /// # Arguments
    ///
    /// * `name` - A type that can be converted into `Name`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder};
    /// use gearbox::net::http::request::header::Name;
    ///
    /// let mut builder = Builder::GET.header(("Content-Type", "application/json"));
    /// let header = builder.header_mut("Content-Type");
    /// ```
    pub fn header_mut<N: Into<Name>>(&mut self, name: N) -> Option<&mut Values> {
        self.headers
            .get_or_insert(HeaderMap::default())
            .get_mut(name)
    }

    /// Sets multiple headers for the request.
    ///
    /// # Arguments
    ///
    /// * `headers` - A type that can be converted into `HeaderMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, HeaderMap};
    ///
    /// let mut headers = HeaderMap::default();
    /// headers.insert(("Content-Type", "application/json").into());
    /// let builder = Builder::GET.headers(headers);
    /// ```
    pub fn headers<H: Into<HeaderMap>>(mut self, headers: H) -> Self {
        self.headers
            .get_or_insert(HeaderMap::default())
            .extend(headers.into());
        self
    }

    /// Gets a mutable reference to the headers of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, HeaderMap};
    ///
    /// let mut builder = Builder::GET;
    /// let headers = builder.headers_mut();
    /// headers.insert(("Content-Type", "application/json").into());
    /// ```
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        self.headers.get_or_insert(HeaderMap::default())
    }

    /// Sets the body of the request.
    ///
    /// # Arguments
    ///
    /// * `body` - A type that can be converted into `Body`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Builder;
    ///
    /// let builder = Builder::POST.body(r#"{"key": "value"}"#);
    /// ```
    pub fn body<B: Into<Body>>(mut self, body: B) -> Self {
        self.body = Some(BodyOwned::from(body.into()));
        self
    }

    /// Gets a mutable reference to the body of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder};
    ///
    /// let mut builder = Builder::POST.body(r#"{"key": "value"}"#);
    /// let body = builder.body_mut();
    /// ```
    pub fn body_mut(&mut self) -> &mut BodyOwned {
        self.body.get_or_insert(BodyOwned::default())
    }

    pub async fn update_body<
        P: FnOnce(Box<Body>) -> O,
        O: Future<Output = Result<Box<Body>, DynTracerError>>,
    >(
        &mut self,
        o: P,
    ) -> Result<(), DynTracerError> {
        let new_body = o(self
            .body
            .get_or_insert(BodyOwned::default())
            .body
            .lock()
            .clone())
        .await?;

        *self.body.get_or_insert(BodyOwned::default()).body.lock() = new_body;
        Ok(())
    }

    /// Sets the URL of the request.
    ///
    /// # Arguments
    ///
    /// * `url` - A type that can be converted into `Url`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Url};
    ///
    /// let url = Url::from("https://example.com");
    /// let builder = Builder::GET.url(url);
    /// ```
    pub fn url<U: Into<Url>>(mut self, url: U) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Gets a mutable reference to the URL of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Url};
    ///
    /// let mut builder = Builder::GET.url("https://example.com");
    /// let url = builder.url_mut();
    /// ```
    pub fn url_mut(&mut self) -> &mut Option<Url> {
        &mut self.url
    }
}

impl Builder {
    /// Sets the HTTP method of the request.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method for the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::GET.set_method(Method::Post);
    /// assert_eq!(builder.get_method(), Method::Post);
    /// ```
    pub fn set_method(mut self, method: Method) -> Self {
        self.method = method.into();
        self
    }

    /// Gets the HTTP method of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::GET;
    /// assert_eq!(builder.get_method(), Method::Get);
    /// ```
    pub fn get_method(&self) -> Method {
        self.method.clone()
    }

    /// Sets a header for the request.
    ///
    /// # Arguments
    ///
    /// * `header` - A `Header` struct that holds the header for the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Header};
    ///
    /// let mut builder = Builder::GET;
    /// builder.set_header(Header::new("Content-Type", "application/json"));
    /// ```
    pub fn set_header(mut self, header: Header) -> Self {
        self.headers
            .get_or_insert(HeaderMap::default())
            .insert(header.into());
        self
    }

    /// Sets multiple headers for the request.
    ///
    /// # Arguments
    ///
    /// * `headers` - A `HeaderMap` struct that holds multiple headers for the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, HeaderMap};
    ///
    /// let mut headers = HeaderMap::default();
    /// headers.insert(("Content-Type", "application/json").into());
    /// let builder = Builder::GET.set_headers(headers);
    /// ```
    pub fn set_headers(mut self, headers: HeaderMap) -> Self {
        self.headers
            .get_or_insert(HeaderMap::default())
            .extend(headers.into());
        self
    }

    /// Gets the headers of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, HeaderMap};
    ///
    /// let builder = Builder::GET;
    /// let headers = builder.get_headers();
    /// ```
    pub fn get_headers(&self) -> Option<HeaderMap> {
        self.headers.clone()
    }

    /// Adds a single header to the request.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the header.
    /// * `value` - The value of the header.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Builder;
    ///
    /// let builder = Builder::GET.add_header("Content-Type".into(), "application/json".into());
    /// ```
    pub fn add_header(mut self, name: String, value: String) -> Self {
        self.headers
            .get_or_insert(HeaderMap::default())
            .insert((name, value).into());
        self
    }

    /// Adds multiple headers to the request.
    ///
    /// # Arguments
    ///
    /// * `headers` - A vector of tuples containing header names and values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Builder;
    ///
    /// let headers = vec![("Content-Type".into(), "application/json".into())];
    /// let builder = Builder::GET.add_headers(headers);
    /// ```
    pub fn add_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.headers
            .get_or_insert(HeaderMap::default())
            .extend(headers.into());
        self
    }

    /// Gets a single header by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the header.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Builder;
    ///
    /// let builder = Builder::GET.add_header("Content-Type".into(), "application/json".into());
    /// let header = builder.get_header("Content-Type");
    /// ```
    pub fn get_header(&self, name: &str) -> Option<Header> {
        self.headers.as_ref().and_then(|headers| headers.get(name))
    }

    /// Sets the body of the request.
    ///
    /// # Arguments
    ///
    /// * `body` - A `Body` struct that holds the body of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Body};
    ///
    /// let body = Body::from(r#"{"key": "value"}"#);
    /// let builder = Builder::POST.set_body(body);
    /// ```
    pub fn set_body(mut self, body: Body) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Gets the body of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Builder;
    ///
    /// let builder = Builder::POST.body(r#"{"key": "value"}"#);
    /// let body = builder.get_body();
    /// ```
    pub fn get_body(&self) -> Option<&BodyOwned> {
        self.body.as_ref()
    }

    /// Sets the URL of the request.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URL as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Url};
    ///
    /// let url = Url::from("https://example.com");
    /// let builder = Builder::GET.set_uri("https://example.com");
    /// ```
    pub fn set_uri(mut self, uri: &str) -> Self {
        self.url = Some(Url::from(uri));
        self
    }

    /// Gets the URL of the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Builder;
    ///
    /// let builder = Builder::GET.url("https://example.com");
    /// let url = builder.get_uri();
    /// ```
    pub fn get_uri(&self) -> Option<&Url> {
        self.url.as_ref()
    }
}

impl fmt::Debug for Builder {
    /// Formats the `Builder` for debugging purposes.
    ///
    /// This method provides a custom implementation for the `fmt::Debug` trait,
    /// allowing for more informative debug output.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let client_placeholder = match &self.client {
            Some(_) => "<Client>",
            None => "None",
        };
        let mut debug_struct = f.debug_struct("RequestBuilder");
        debug_struct
            .field("client", &client_placeholder)
            .field("method", &self.method)
            .field("uri", &self.url)
            .field("headers", &self.headers)
            .field("body", &"<Body>")
            .field("content_type", &self.content_type);
        #[cfg(feature = "net-signature")]
        debug_struct.field("signature", &self.signature);

        debug_struct.finish()
    }
}

impl Default for Builder {
    /// Provides a default instance of the `Builder`.
    ///
    /// This method initializes a `Builder` with default values for its fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::{Builder, Method};
    ///
    /// let builder = Builder::default();
    /// assert_eq!(builder.get_method(), Method::None);
    /// ```
    fn default() -> Self {
        Self {
            client: None,
            method: Method::None,
            url: None,
            headers: None,
            body: None,
            content_type: None,
            #[cfg(feature = "net-signature")]
            signature: GLOBAL_SIGNING.read().clone(),
        }
    }
}

// Manually implement Serialize
impl ser::Serialize for Builder {
    /// Serializes the `Builder` into a format suitable for storage or transmission.
    ///
    /// This method manually implements the `Serialize` trait for the `Builder` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Builder;
    /// use serde_json;
    ///
    /// let builder = Builder::GET;
    /// let serialized = serde_json::to_string(&builder).unwrap();
    /// println!("{}", serialized);
    /// ```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RequestBuilder", 6)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("uri", &self.url)?;
        state.serialize_field("headers", &self.headers)?;

        if let Some(ref body_owned) = self.body {
            let _t = body_owned
                .try_sync_into_string()
                .map(|t| state.serialize_field("body", &t))
                .map_err(|t| ser::Error::custom(t.to_string()))?;
        } else {
            state.serialize_field("body", &self.body)?;
        }

        if let Some(ref content_type) = self.content_type {
            state.serialize_field("content_type", &content_type)?;
        }

        #[cfg(feature = "net-signature")]
        if let Some(ref signature) = self.signature {
            state.serialize_field("signature", &signature)?;
        }

        state.end()
    }
}

// Manually implement Deserialize
impl<'de> Deserialize<'de> for Builder {
    /// Deserializes the `Builder` from a stored or transmitted format.
    ///
    /// This method manually implements the `Deserialize` trait for the `Builder` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Builder;
    /// use serde_json;
    ///
    /// let data = r#"{"method":"Get","uri":null,"headers":null,"body":null,"content_type":null}"#;
    /// let builder: Builder = serde_json::from_str(data).unwrap();
    /// println!("{:?}", builder);
    /// ```
    fn deserialize<D>(deserializer: D) -> Result<Builder, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct BuilderHelper {
            method: Method,
            uri: Option<Url>,
            headers: Option<HeaderMap>,
            body: Option<String>,
            content_type: Option<String>,
            #[cfg(feature = "net-signature")]
            signature: Option<Signature>,
        }

        let data = BuilderHelper::deserialize(deserializer)?;
        let body = if let Some(body_str) = data.body {
            Some(BodyOwned::from(body_str))
        } else {
            None
        };

        Ok(Builder {
            client: None,
            method: data.method,
            url: data.uri,
            headers: data.headers,
            body,
            content_type: data.content_type,
            #[cfg(feature = "net-signature")]
            signature: data.signature,
        })
    }
}

/// Enumeration of supported HTTP methods.
// #[cfg_attr(feature = "std", derive(uniffi::Object))]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    /// Converts a string slice to a `Method`.
    ///
    /// This method allows for the conversion of a string slice to a `Method` enum
    /// variant. The string is matched case-insensitively to the known HTTP methods.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice representing the HTTP method.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Method;
    ///
    /// let method = Method::from("GET");
    /// assert_eq!(method, Method::Get);
    /// ```
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
    /// Converts a `String` to a `Method`.
    ///
    /// This method allows for the conversion of a `String` to a `Method` enum
    /// variant. The string is matched case-insensitively to the known HTTP methods.
    ///
    /// # Arguments
    ///
    /// * `s` - A `String` representing the HTTP method.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Method;
    ///
    /// let method = Method::from(String::from("POST"));
    /// assert_eq!(method, Method::Post);
    /// ```
    fn from(s: String) -> Self {
        Method::from(s.as_str())
    }
}

impl From<Method> for reqwest::Method {
    /// Converts an `Method` to a `reqwest::Method`.
    ///
    /// This method allows for the conversion of an `Method` enum variant to a
    /// `reqwest::Method`, which is used by the `reqwest` library to represent HTTP
    /// methods.
    ///
    /// # Arguments
    ///
    /// * `m` - An `Method` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Method;
    /// use reqwest::Method as ReqwestMethod;
    ///
    /// let method = Method::Get;
    /// let reqwest_method: ReqwestMethod = method.into();
    /// assert_eq!(reqwest_method, ReqwestMethod::GET);
    /// ```
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
    /// Converts a reference to an `Method` to a `reqwest::Method`.
    ///
    /// This method allows for the conversion of a reference to an `Method` enum
    /// variant to a `reqwest::Method`, which is used by the `reqwest` library to
    /// represent HTTP methods.
    ///
    /// # Arguments
    ///
    /// * `m` - A reference to an `Method` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Method;
    /// use reqwest::Method as ReqwestMethod;
    ///
    /// let method = Method::Post;
    /// let reqwest_method: ReqwestMethod = (&method).into();
    /// assert_eq!(reqwest_method, ReqwestMethod::POST);
    /// ```
    fn from(m: &Method) -> Self {
        reqwest::Method::from(m.clone())
    }
}

impl From<&Method> for Method {
    /// Converts a reference to an `Method` to an owned `Method`.
    ///
    /// This method allows for the conversion of a reference to an `Method` enum
    /// variant to an owned `Method`.
    ///
    /// # Arguments
    ///
    /// * `m` - A reference to an `Method` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use gearbox::net::http::request::Method;
    ///
    /// let method = Method::Get;
    /// let owned_method: Method = (&method).into();
    /// assert_eq!(owned_method, Method::Get);
    /// ```
    fn from(m: &Method) -> Self {
        m.clone()
    }
}

#[cfg(test)]
pub mod test {
    use super::Builder;
    use serde_json;

    #[test]
    fn test_builder_serialization() {
        let builder = Builder::GET;
        let serialized = serde_json::to_string(&builder).unwrap();
        println!("{}", serialized);
    }

    #[test]
    fn test_builder_deserialization() {
        let data = r#"{"method":"Get","uri":null,"headers":null,"body":null,"content_type":null}"#;
        let builder: Builder = serde_json::from_str(data).unwrap();
        println!("{:?}", builder);
    }
}
