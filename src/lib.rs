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

//
// Core System
//
extern crate core;

//
// Standard system
//
#[cfg(feature = "std")]
extern crate std;

//
// Dependencies already in CORE
//
extern crate alloc;

//
// Optional Dependencies
//
#[cfg(feature = "axum")]
extern crate axum;
#[cfg(feature = "base64")]
extern crate base64;
#[cfg(feature = "bs58")]
extern crate bs58;
#[cfg(feature = "bson")]
extern crate bson;
#[cfg(feature = "bytes")]
extern crate bytes;
#[cfg(feature = "derive_more")]
#[macro_use]
extern crate derive_more;
#[cfg(feature = "didkit")]
extern crate didkit;
#[cfg(feature = "erased_serde")]
extern crate erased_serde;
#[cfg(feature = "flexbuffers")]
extern crate flexbuffers;
#[cfg(feature = "futures")]
extern crate futures;
#[cfg(feature = "gearbox_macros")]
#[macro_use]
extern crate gearbox_macros;
#[cfg(feature = "hashbrown")]
extern crate hashbrown;
#[cfg(feature = "hex")]
extern crate hex;
#[cfg(feature = "hmac")]
extern crate hmac; // ## For Testing!
#[cfg(feature = "hyper")]
extern crate hyper;
#[cfg(feature = "hyper_util")]
extern crate hyper_util;
#[cfg(feature = "if_addrs")]
extern crate if_addrs;
#[cfg(feature = "json5")]
extern crate json5;
#[cfg(feature = "pnet")]
extern crate pnet;
#[cfg(feature = "postcard")]
extern crate postcard;
#[cfg(feature = "reqwest")]
extern crate reqwest;
#[cfg(feature = "rmp_serde")]
extern crate rmp_serde;
#[cfg(feature = "ron")]
extern crate ron;
#[cfg(feature = "semver")]
extern crate semver;
#[cfg(feature = "dep_serde")]
extern crate serde as crate_serde;
#[cfg(feature = "serde_cbor")]
extern crate serde_cbor;
#[cfg(feature = "serde_derive")]
extern crate serde_derive;
#[cfg(feature = "serde_json")]
extern crate serde_json;
#[cfg(feature = "serde_lexpr")]
extern crate serde_lexpr;
#[cfg(feature = "serde_pickle")]
extern crate serde_pickle as pickle;
#[cfg(feature = "serde_qs")]
extern crate serde_qs;
#[cfg(feature = "serde_xml_rs")]
extern crate serde_xml_rs;
#[cfg(feature = "serde_yaml")]
extern crate serde_yaml;
#[cfg(feature = "sha2")]
extern crate sha2;
#[cfg(feature = "spin")]
extern crate spin;
#[cfg(feature = "tokio")]
extern crate tokio;
#[cfg(feature = "tracing")]
extern crate tracing;
#[cfg(feature = "tracing_subscriber")]
extern crate tracing_subscriber;
#[cfg(feature = "uniffi")]
extern crate uniffi;
#[cfg(feature = "uniffi_macros")]
extern crate uniffi_macros;
#[cfg(test)]
#[macro_use]
extern crate wasm_bindgen_test;
#[cfg(all(target_arch = "wasm32", feature = "web"))]
extern crate web_sys;

#[cfg(all(feature = "syslog-macro", feature = "log-macro"))]
compile_error!("`syslog-macro` and `log-macro` cannot be enabled at the same time.");

// uniffi::setup_scaffolding!();
#[cfg(feature = "collections")]
pub mod collections;
#[cfg(feature = "common")]
pub mod common;
#[cfg(feature = "did")]
pub mod did;
#[cfg(feature = "error")]
pub mod error;
#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "net")]
pub mod net;
#[cfg(feature = "path")]
pub mod path;
#[cfg(feature = "rails")]
pub mod rails;
#[cfg(feature = "serde")]
pub mod serde;
#[cfg(feature = "service")]
pub mod service;
#[cfg(feature = "storage")]
pub mod storage;
#[cfg(feature = "sync")]
pub mod sync;
#[cfg(feature = "task")]
pub mod task;
#[cfg(feature = "template")]
pub mod template;
#[cfg(feature = "time")]
pub mod time;

#[allow(unused_imports)]
#[cfg(all(
    feature = "log-tracing-macros",
    not(feature = "log-tracing-macros-syslog")
))]
pub use crate::log::tracing::macros::common::*;

#[allow(unused_imports)]
#[cfg(feature = "log-tracing-macros-syslog")]
pub use crate::log::tracing::macros::syslog::*;

#[cfg(feature = "error-tracer-macros")]
#[allow(unused_imports)]
pub use crate::error::tracer::error_macro::*;

pub mod macros {
    pub mod loaders {
        #[cfg(feature = "gearbox_macros")]
        pub use gearbox_macros::load_consts;
    }
}

pub mod externs {

    pub mod collections {
        #[cfg(feature = "hashbrown")]
        pub use hashbrown::HashMap;
        #[cfg(feature = "hashbrown")]
        pub use hashbrown::HashSet;
    }

    #[cfg(feature = "spin")]
    pub mod spin {
        pub use spin::*;
    }

    #[cfg(target_arch = "wasm32")]
    pub mod wasm_bindgen {
        pub use wasm_bindgen::__rt;
        pub use wasm_bindgen::convert;
        pub use wasm_bindgen::describe;
        pub use wasm_bindgen::prelude;
    }
    #[cfg(feature = "dep_serde")]
    pub mod serde {
        pub use crate_serde::*;
        pub mod derive {
            pub use serde_derive::*;
        }
        #[cfg(feature = "with_json")]
        pub mod json {
            pub use serde_json::*;
        }
    }
    pub use core::{cell, fmt, marker, mem, ops, ptr};

    pub mod sync {
        pub use alloc::sync::Arc;
        pub use core::sync::atomic;
    }

    pub mod rc {
        pub use alloc::rc::Rc;
    }

    pub mod service {
        #[cfg(feature = "service-framework-axum")]
        pub mod axum {
            pub use axum::*;
        }
    }

    pub mod primitives {
        #[cfg(feature = "bytes")]
        pub mod bytes {
            pub use bytes::*;
        }
    }

    #[cfg(feature = "tracing")]
    pub mod tracing {
        pub use tracing::*;
    }
}

#[cfg(test)]
mod tests {}
