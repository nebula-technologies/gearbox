#![cfg_attr(not(feature = "std"), no_std)]
//! [![pipeline status](https://gitlab.nebula.technology/libraries/rust/gearbox/badges/main/pipeline.svg)](https://gitlab.nebula.technology/libraries/rust/gearbox/-/commits/main)
//! [![coverage report](https://gitlab.nebula.technology/libraries/rust/gearbox/badges/main/coverage.svg)](https://gitlab.nebula.technology/libraries/rust/gearbox/-/commits/main)
//! [![Latest Release](https://gitlab.nebula.technology/libraries/rust/gearbox/-/badges/release.svg)](https://gitlab.nebula.technology/libraries/rust/gearbox/-/releases)
//!
//! Gearbox is a versatile library that encompasses a wide array of functionalities, including networking, logging,
//! railway-oriented programming extensions, and time management. Initially designed as a collection of utilities, the
//! ultimate vision for Gearbox is to evolve into a highly optimized, standalone toolkit. The goal is to minimize external
//! dependencies progressively, leading to a library that is lightweight and efficient. By doing so, Gearbox aims to be
//! universally compatible, from embedded systems to WebAssembly (WASM) environments, all while maintaining simplicity and
//! minimizing the need for boilerplate code. This development strategy positions Gearbox as a comprehensive solution for
//! developers seeking to build efficient, scalable applications across a broad spectrum of platforms.
//!
//! # Features
//!
//! | Category   | Feature               | use                                     | Description                                                                                                                                                                                                                                                                                                                                                                                                     | Status |
//! |------------|-----------------------|-----------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------|
//! | Common     | TryDefault            | gearbox::common::TryDefault             | This is a trait used internally in `Gearbox` defining a `TryDefault` trait that returns a Result<T,Self::Error>. It can also be used in other systems.                                                                                                                                                                                                                                                          | ✅      |
//! |            | BoxedFuture           | gearbox::common::BoxedFuture            | Type alias for a pinned boxed future. Used for returning dynamically dispatched futures.                                                                                                                                                                                                                                                                                                                        | ✅      |
//! | Error      | ErrorTracer           | gearbox::error::tracer::*               | An error structure that builds a traceable stack of errors. It allows breaking down the error into a TypeId to define the encapsulated error for further operations. Contains information about file, line, module path, and optional error_code with display and debug. This also comes with the macro `Error!()` which sets up the `ErrorTracerExtInfo` with all the needed information (file, line, module). | ⚠️     |
//! |            | Rail ErrorTracer      | gearbox::rails::ext::map_err_tracer     | Simplification for `map_err` for operating with the ErrorTracer, allowing for passing an `Error!()` or an `ErrorTracerExtInfo` for collecting all the information.                                                                                                                                                                                                                                              | ✅      |
//! | Logging    | Tracing Log Formatter | gearbox::log::fmt::*                    | Custom subscriber for formatting logs when using the rust Tracing library.                                                                                                                                                                                                                                                                                                                                      | ⚠️     |
//! | Networking | hostname              | gearbox::net::hostname                  | Get the hostname of the local machine.                                                                                                                                                                                                                                                                                                                                                                          | ✅      |
//! |            | HTTP Request          | gearbox::net::http::request             | Send an HTTP request. This is an extension on top of `Reqwest` that simplifies the implementation of mTLS and payload signing.                                                                                                                                                                                                                                                                                  | ⚠️     |
//! |            | HTTP Request Chaining | gearbox::net::http::request_chain       | Chaining system for HTTP requests, allowing for chaining requests and responses for more advanced request/response handling.                                                                                                                                                                                                                                                                                    | ❌      |
//! | Paths      | Common Paths          | gearbox::path::*                        | Common paths under Windows, Linux, and more. For example, under Linux, the config path is usually `~/.config/`.                                                                                                                                                                                                                                                                                                 | ✅      |
//! | Rails      | Common Extensions     | gearbox::rails::ext::*                  | Various extension traits for operating on `Result`, `Option`, and other standard types, providing additional methods for error handling, merging results, and tapping into values.                                                                                                                                                                                                                              | ⚠️     |
//! |            | Future Extensions     | gearbox::rails::ext::future::*          | Extensions for working with `Future` types, providing methods for mapping, chaining, and merging futures.                                                                                                                                                                                                                                                                                                       | ✅      |
//! | serde      | Dynamic Serialization | gearbox::serde::dynamic::*              | Dynamic serialization system that allows for encoding and decoding of multiple formats. This is a simplified version of the serde library.                                                                                                                                                                                                                                                                      | ⚠️     |
//! |            | Wasm Bindgen Ser/de   | gearbox::serde::wasm_bindgen::*         | Implementation for WASM Bind Generator that allows for serialization/deserialization of JsValue.                                                                                                                                                                                                                                                                                                                | ✅      |
//! | Storage    | Web Storage           | gearbox::storage::web::local_storage::* | Interface for interacting with local storage in a web environment, including features for setting, getting, and deleting data with JSON serialization/deserialization support.                                                                                                                                                                                                                                  | 🚧     |
//! |            | File Storage          | gearbox::storage::io::file::*           | Interface for interacting with file storage, including features for setting, getting, and deleting data with JSON and YAML serialization/deserialization support.                                                                                                                                                                                                                                               | 🧪     |
//! |            | Selective Storage     | gearbox::storage::selective_storage     | Trait for selective storage operations, providing methods for creating, setting, getting, and deleting storage entries.                                                                                                                                                                                                                                                                                         | 🚧     |
//! | Time       | Time Stamps and more  | gearbox::time::*                        | Timestamp system similar to Chrono, handling times and time calculations. Used throughout Gearbox instead of Chrono.                                                                                                                                                                                                                                                                                            | ⚠️     |
//! | Template   | Template Engine       | gearbox::template::*                    | Template engine responsible for rendering templates using context data and applying pipelines for data transformations. It supports pipelines for operations like date formatting and string prefixing.                                                                                                                                                                                                         | ⚠️     |
//!
//! ### Status Icons Explanation
//!
//! - ✅ Completed: The feature is fully implemented and tested.
//! - ❌ Not Completed: The feature is not implemented.
//! - ⚠️ Partially: The feature is partially implemented.
//! - 🚧 In Development: The feature is currently being developed.
//! - 🧪 Missing Testing: The feature is implemented but lacks testing.
//!
//! # Test Status
//! [See Test Status](./TEST_STATUS.md)
//!
//!
//! # Http Request (gearbox::net::http::request)
//!
//! ## Complete architectural overview:
//! ```mermaid
//! classDiagram
//!     %% Package: request
//!     namespace request {
//!         class Client {
//!             +client: reqwest::Client
//!             +new()
//!             +with_client(reqwest::Client)
//!             +set_global_signing(Signature)
//!         }
//!
//!         class Error {
//!             +UrlParser(ParseError)
//!             +Request(reqwest::Error)
//!             +NoUrl
//!             +HeaderValue(reqwest::header::InvalidHeaderValue)
//!             +DeserializeContentType(String)
//!             +DeserializeJson(serde_json::Error)
//!             +BodyError(TracerError)
//!         }
//!         
//!         class Method {
//!             <<enumeration>>
//!             Get
//!             Post
//!             Put
//!             Delete
//!             Patch
//!             Head
//!             Options
//!             Connect
//!             Trace
//!             None
//!         }
//!
//!         class RequestBuilder {
//!             +client: Option<Arc<Client>>
//!             +method: Method
//!             +uri: Option<Url>
//!             +headers: HeaderMap
//!             +body: Body
//!             +content_type: String
//!             +signature: Option<Signature>
//!             +new_with_client(client: Option<Client>, method: Method, uri: Url)
//!             +method(T: Into<Method>)
//!             +uri(uri: &str)
//!             +header(H: Into<Header>)
//!             +headers(H: Into<HeaderMap>)
//!             +body(B: Into<Body>)
//!             +content_type(content_type: &str)
//!             +with_signing_default()
//!             +with_signing(signature: Signature)
//!             +send()
//!         }
//!
//!         class Response {
//!             +status: StatusCode
//!             +headers: HeaderMap
//!             +content_length: Option<u64>
//!             +url: Url
//!             +body: BodyOwned
//!             +status()
//!             +to(T: DeserializeOwned)
//!         }
//!
//!         class StatusCode {
//!             +code: u16
//!             +reason: &'static str
//!             +as_u16()
//!             +as_str()
//!         }
//!         
//!         class Url {
//!             +Simple(url: String)
//!         }
//!
//!         class Body {
//!             +Empty
//!         }
//!         
//!         class BodyOwned {
//!             +from(box_raw: Box<reqwest::Response>)
//!         }
//!     }
//!
//!     %% Relationships
//!     Client --> RequestBuilder
//!     RequestBuilder --> Response
//!     RequestBuilder --> Error
//!     Response --> StatusCode
//!     Response --> HeaderMap
//!     Response --> Url
//!     Response --> BodyOwned
//!     HeaderMap --> Header
//!     Header --> Name
//!     Header --> Values
//!     Values --> Value
//! ```
//!
//!
//! # Http Request Chaining (gearbox::net::http::request_chaining)
//!
//! ## Complete architectural overview:
//! ```mermaid
//! classDiagram
//!     %% Package: request
//!     namespace request {
//!         class Client {
//!             +client: reqwest::Client
//!             +new()
//!             +with_client(reqwest::Client)
//!             +set_global_signing(Signature)
//!         }
//!
//!         class Error {
//!             +UrlParser(ParseError)
//!             +Request(reqwest::Error)
//!             +NoUrl
//!             +HeaderValue(reqwest::header::InvalidHeaderValue)
//!             +DeserializeContentType(String)
//!             +DeserializeJson(serde_json::Error)
//!             +BodyError(TracerError)
//!         }
//!
//!         class Method {
//!             <<enumeration>>
//!             Get
//!             Post
//!             Put
//!             Delete
//!             Patch
//!             Head
//!             Options
//!             Connect
//!             Trace
//!             None
//!         }
//!
//!         class RequestBuilder {
//!             +client: Option<Arc<Client>>
//!             +method: Method
//!             +uri: Option<Url>
//!             +headers: HeaderMap
//!             +body: Body
//!             +content_type: String
//!             +signature: Option<Signature>
//!             +new_with_client(client: Option<Client>, method: Method, uri: Url)
//!             +method(T: Into<Method>)
//!             +uri(uri: &str)
//!             +header(H: Into<Header>)
//!             +headers(H: Into<HeaderMap>)
//!             +body(B: Into<Body>)
//!             +content_type(content_type: &str)
//!             +with_signing_default()
//!             +with_signing(signature: Signature)
//!             +send()
//!         }
//!
//!         class Response {
//!             +status: StatusCode
//!             +headers: HeaderMap
//!             +content_length: Option<u64>
//!             +url: Url
//!             +body: BodyOwned
//!             +status()
//!             +to(T: DeserializeOwned)
//!         }
//!
//!         class StatusCode {
//!             +code: u16
//!             +reason: &'static str
//!             +as_u16()
//!             +as_str()
//!         }
//!
//!         class Url {
//!             +Simple(url: String)
//!         }
//!
//!         class Body {
//!             +Empty
//!         }
//!
//!         class BodyOwned {
//!             +from(box_raw: Box<reqwest::Response>)
//!         }
//!     }
//!
//!     %% Package: request_chaining
//!     namespace request_chaining {
//!         class Header {
//!             +name: Name
//!             +values: Values
//!         }
//!
//!         class HeaderMap {
//!             +inner: HashMap<Name, Values>
//!             +get(K: Into<Name>)
//!             +insert(header: Header)
//!             +extend(headers: HeaderMap)
//!         }
//!
//!         class Name {
//!             +String name
//!         }
//!
//!         class Values {
//!             +Vec<Value> values
//!             +iter()
//!         }
//!
//!         class Value {
//!             +Vec<u8> value
//!             +as_bytes()
//!             +to_vec()
//!         }
//!
//!         class TracerError
//!         class Signature
//!         class ParseError
//!     }
//!
//!     %% Relationships
//!     Client --> RequestBuilder
//!     RequestBuilder --> Response
//!     RequestBuilder --> Error
//!     Response --> StatusCode
//!     Response --> HeaderMap
//!     Response --> Url
//!     Response --> BodyOwned
//!     HeaderMap --> Header
//!     Header --> Name
//!     Header --> Values
//!     Values --> Value
//! ```
//!
//! # Signature (gearbox::net::signature)
//! ([docs: gearbox::net::signature](./src/net/signature/mod.rs))
//!
//!
//! # Railway Future extension (gearbox::rails::ext::future)
//! ([docs: gearbox::serde::dynamic](./src/rails/ext/future/mod.rs))
//!
//! # Dynamic Serialization/Deserialization (gearbox::serde::dynamic)
//! ([docs: gearbox::serde::dynamic](./src/serde/dynamic/mod.rs))
//!
//! # RwArc (gearbox::sync::rw_arc)
//! ([docs: gearbox::sync::rw_arc](./src/sync/rw_arc/mod.rs))
//!
//! # RwArc (gearbox::template)
//! ([docs: gearbox::template](./src/template/mod.rs))
//!
//! ## TODO
//!
//! - [ ] ( gearbox::log::* ) Clean up Log fmt/syslog, some of the code can be combined and cleaned up a bit better, also the formatter supports syslog, and bunyan, this should probably be cleared up a bit more, and separated better.
//! - [ ] ( gearbox::path::* ) current this system is mainly just exposing the dirs::* library, this should be removed.
//! - [ ] ( gearbox::* ) Remove usage for Vec or move usage of std::vec::Vec to another no-std library
//!
//!
//!

pub extern crate alloc;
#[cfg(feature = "net-signature")]
pub extern crate base64;
#[cfg(feature = "net-signature")]
pub extern crate bs58;
#[cfg(all(feature = "serde-bson", feature = "serde-dynamic"))]
pub extern crate bson;
pub extern crate bytes;
pub extern crate core;
#[macro_use]
pub extern crate derive_more;
#[cfg(feature = "did")]
pub extern crate didkit;
pub extern crate erased_serde;
#[cfg(all(feature = "serde-flexbuffers", feature = "serde-dynamic"))]
pub extern crate flexbuffers;
pub extern crate futures;
#[macro_use]
pub extern crate gearbox_macros;
pub extern crate hashbrown;
#[cfg(feature = "net-signature")]
pub extern crate hex;
#[cfg(feature = "net-signature")]
pub extern crate hmac; // ## For Testing!
#[cfg(test)]
pub extern crate hyper;
#[cfg(feature = "std")]
extern crate if_addrs;
#[cfg(all(feature = "serde-json5", feature = "serde-dynamic"))]
pub extern crate json5;
#[cfg(feature = "std")]
extern crate pnet;
#[cfg(all(feature = "serde-postcard", feature = "serde-dynamic"))]
pub extern crate postcard;
#[cfg(feature = "http")]
pub extern crate reqwest;
#[cfg(all(feature = "serde-messagepack", feature = "serde-dynamic"))]
pub extern crate rmp_serde;
#[cfg(all(feature = "serde-ron", feature = "serde-dynamic"))]
pub extern crate ron;
pub extern crate semver;
#[cfg(feature = "http")]
pub extern crate serde as crate_serde;
#[cfg(all(feature = "serde-cbor", feature = "serde-dynamic"))]
pub extern crate serde_cbor;
pub extern crate serde_derive;
#[cfg(all(feature = "serde-json", feature = "serde-dynamic"))]
pub extern crate serde_json;
#[cfg(all(feature = "serde-lexpr", feature = "serde-dynamic"))]
pub extern crate serde_lexpr;
#[cfg(all(feature = "serde-pickle", feature = "serde-dynamic"))]
pub extern crate serde_pickle as pickle;
#[cfg(all(feature = "serde-query-string", feature = "serde-dynamic"))]
pub extern crate serde_qs;
#[cfg(all(
    feature = "serde-accept-limited-xml-serialize",
    feature = "serde-dynamic"
))]
pub extern crate serde_xml_rs;
#[cfg(all(feature = "serde-yaml", feature = "serde-dynamic"))]
pub extern crate serde_yaml;
#[cfg(feature = "net-signature")]
pub extern crate sha2;
pub extern crate spin;
#[cfg(feature = "std")]
pub extern crate std;
#[cfg(not(target_arch = "wasm32"))]
pub extern crate tokio;
pub extern crate tracing;
pub extern crate uniffi;
pub extern crate uniffi_macros;
#[cfg(test)]
#[macro_use]
pub extern crate wasm_bindgen_test;
#[cfg(all(target_arch = "wasm32", feature = "web"))]
pub extern crate web_sys;
#[cfg(all(feature = "syslog-macro", feature = "log-macro"))]
compile_error!("`syslog-macro` and `log-macro` cannot be enabled at the same time.");

#[cfg(feature = "std")]
// uniffi::setup_scaffolding!();
pub mod collections;
pub mod common;
#[cfg(feature = "did")]
pub mod did;
pub mod error;
pub mod log;
pub mod net;
pub mod path;
pub mod rails;
pub mod serde;
pub mod storage;
pub mod sync;
#[cfg(not(target_arch = "wasm32"))]
pub mod task;
pub mod template;
pub mod time;

#[allow(unused_imports)]
#[cfg(feature = "log-macro")]
pub use crate::log::tracing::macros::common::*;

#[allow(unused_imports)]
#[cfg(feature = "syslog-macro")]
pub use crate::log::tracing::macros::syslog::*;

#[allow(unused_imports)]
pub use crate::error::tracer::error_macro::*;

pub mod macros {
    pub mod loaders {
        pub use gearbox_macros::load_consts;
    }
}

pub mod externs {

    pub mod collections {
        pub use hashbrown::HashMap;
        pub use hashbrown::HashSet;
    }

    #[cfg(target_arch = "wasm32")]
    pub mod wasm_bindgen {
        pub use wasm_bindgen::__rt;
        pub use wasm_bindgen::convert;
        pub use wasm_bindgen::describe;
        pub use wasm_bindgen::prelude;
    }
    pub mod serde {
        pub use crate_serde::{Deserialize, Deserializer, Serialize, Serializer};
    }
    pub use core::{cell, fmt, marker, mem, ops, ptr};
    pub mod sync {
        pub use alloc::sync::Arc;
        pub use core::sync::atomic;
    }
    pub mod rc {
        pub use alloc::rc::Rc;
    }
}

#[cfg(test)]
mod tests {}
