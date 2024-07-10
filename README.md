# gearbox

![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)
[![pipeline status](https://gitlab.nebula.technology/libraries/rust/gearbox/badges/main/pipeline.svg)](https://gitlab.nebula.technology/libraries/rust/gearbox/-/commits/main)
[![coverage report](https://gitlab.nebula.technology/libraries/rust/gearbox/badges/main/coverage.svg)](https://gitlab.nebula.technology/libraries/rust/gearbox/-/commits/main)
[![Latest Release](https://gitlab.nebula.technology/libraries/rust/gearbox/-/badges/release.svg)](https://gitlab.nebula.technology/libraries/rust/gearbox/-/releases)

Gearbox is a versatile library that encompasses a wide array of functionalities, including networking, logging,
railway-oriented programming extensions, and time management. Initially designed as a collection of utilities, the
ultimate vision for Gearbox is to evolve into a highly optimized, standalone toolkit. The goal is to minimize external
dependencies progressively, leading to a library that is lightweight and efficient. By doing so, Gearbox aims to be
universally compatible, from embedded systems to WebAssembly (WASM) environments, all while maintaining simplicity and
minimizing the need for boilerplate code. This development strategy positions Gearbox as a comprehensive solution for
developers seeking to build efficient, scalable applications across a broad spectrum of platforms.

## Features

| Category   | Feature               | use                                     | Description                                                                                                                                                                                                                                                                                                                                                                                                     | Status |
|------------|-----------------------|-----------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------|
| Common     | TryDefault            | gearbox::common::TryDefault             | This is a trait used internally in `Gearbox` defining a `TryDefault` trait that returns a Result<T,Self::Error>. It can also be used in other systems.                                                                                                                                                                                                                                                          | ✅      |
|            | BoxedFuture           | gearbox::common::BoxedFuture            | Type alias for a pinned boxed future. Used for returning dynamically dispatched futures.                                                                                                                                                                                                                                                                                                                        | ✅      |
| Error      | ErrorTracer           | gearbox::error::tracer::*               | An error structure that builds a traceable stack of errors. It allows breaking down the error into a TypeId to define the encapsulated error for further operations. Contains information about file, line, module path, and optional error_code with display and debug. This also comes with the macro `Error!()` which sets up the `ErrorTracerExtInfo` with all the needed information (file, line, module). | ⚠️     |
|            | Rail ErrorTracer      | gearbox::rails::ext::map_err_tracer     | Simplification for `map_err` for operating with the ErrorTracer, allowing for passing an `Error!()` or an `ErrorTracerExtInfo` for collecting all the information.                                                                                                                                                                                                                                              | ✅      |
| Logging    | Tracing Log Formatter | gearbox::log::fmt::*                    | Custom subscriber for formatting logs when using the rust Tracing library.                                                                                                                                                                                                                                                                                                                                      | ⚠️     |
| Networking | hostname              | gearbox::net::hostname                  | Get the hostname of the local machine.                                                                                                                                                                                                                                                                                                                                                                          | ✅      |
|            | HTTP Request          | gearbox::net::http::request             | Send an HTTP request. This is an extension on top of `Reqwest` that simplifies the implementation of mTLS and payload signing.                                                                                                                                                                                                                                                                                  | ⚠️     |
|            | HTTP Request Chaining | gearbox::net::http::request_chain       | Chaining system for HTTP requests, allowing for chaining requests and responses for more advanced request/response handling.                                                                                                                                                                                                                                                                                    | ❌      |
| Paths      | Common Paths          | gearbox::path::*                        | Common paths under Windows, Linux, and more. For example, under Linux, the config path is usually `~/.config/`.                                                                                                                                                                                                                                                                                                 | ✅      |
| Rails      | Common Extensions     | gearbox::rails::ext::*                  | Various extension traits for operating on `Result`, `Option`, and other standard types, providing additional methods for error handling, merging results, and tapping into values.                                                                                                                                                                                                                              | ⚠️     |
|            | Future Extensions     | gearbox::rails::ext::future::*          | Extensions for working with `Future` types, providing methods for mapping, chaining, and merging futures.                                                                                                                                                                                                                                                                                                       | ✅      |
| serde      | Dynamic Serialization | gearbox::serde::dynamic::*              | Dynamic serialization system that allows for encoding and decoding of multiple formats. This is a simplified version of the serde library.                                                                                                                                                                                                                                                                      | ⚠️     |
|            | Wasm Bindgen Ser/de   | gearbox::serde::wasm_bindgen::*         | Implementation for WASM Bind Generator that allows for serialization/deserialization of JsValue.                                                                                                                                                                                                                                                                                                                | ✅      |
| Storage    | Web Storage           | gearbox::storage::web::local_storage::* | Interface for interacting with local storage in a web environment, including features for setting, getting, and deleting data with JSON serialization/deserialization support.                                                                                                                                                                                                                                  | 🚧     |
|            | File Storage          | gearbox::storage::io::file::*           | Interface for interacting with file storage, including features for setting, getting, and deleting data with JSON and YAML serialization/deserialization support.                                                                                                                                                                                                                                               | 🧪     |
|            | Selective Storage     | gearbox::storage::selective_storage     | Trait for selective storage operations, providing methods for creating, setting, getting, and deleting storage entries.                                                                                                                                                                                                                                                                                         | 🚧     |
| Time       | Time Stamps and more  | gearbox::time::*                        | Timestamp system similar to Chrono, handling times and time calculations. Used throughout Gearbox instead of Chrono.                                                                                                                                                                                                                                                                                            | ⚠️     |
| Template   | Template Engine       | gearbox::template::*                    | Template engine responsible for rendering templates using context data and applying pipelines for data transformations. It supports pipelines for operations like date formatting and string prefixing.                                                                                                                                                                                                         | ⚠️     |

#### Status Icons Explanation

- ✅ Completed: The feature is fully implemented and tested.
- ❌ Not Completed: The feature is not implemented.
- ⚠️ Partially: The feature is partially implemented.
- 🚧 In Development: The feature is currently being developed.
- 🧪 Missing Testing: The feature is implemented but lacks testing.

## Test Status
| File | Coverage Bar | Line Coverage | Lines Covered | Lines Total |
|------|--------------|---------------|---------------|-------------|
| src | ![](https://geps.dev/progress/0) | 0.0% | 0 | 2 |
| src/collections/const_hash_map | ![](https://geps.dev/progress/0) | 0.0% | 0 | 240 |
| src/collections/hash_map | ![](https://geps.dev/progress/26) | 25.76% | 51 | 198 |
| src/collections/simple_linked_list | ![](https://geps.dev/progress/100) | 100.0% | 146 | 146 |
| src/collections/vec_deque | ![](https://geps.dev/progress/96) | 96.02% | 193 | 201 |
| src/common | ![](https://geps.dev/progress/91) | 91.3% | 21 | 23 |
| src/error | ![](https://geps.dev/progress/35) | 35.14% | 13 | 37 |
| src/error/tracer | ![](https://geps.dev/progress/41) | 41.47% | 265 | 639 |
| src/log/fmt | ![](https://geps.dev/progress/36) | 36.42% | 409 | 1123 |
| src/log/fmt/layer | ![](https://geps.dev/progress/0) | 0.0% | 0 | 130 |
| src/log/syslog | ![](https://geps.dev/progress/100) | 100.0% | 166 | 166 |
| src/net | ![](https://geps.dev/progress/74) | 74.29% | 52 | 70 |
| src/net/http/request | ![](https://geps.dev/progress/42) | 42.33% | 527 | 1245 |
| src/net/http/request/header | ![](https://geps.dev/progress/47) | 47.48% | 151 | 318 |
| src/net/http/request_chaining | ![](https://geps.dev/progress/95) | 95.04% | 613 | 645 |
| src/net/http/test | ![](https://geps.dev/progress/87) | 86.76% | 59 | 68 |
| src/net/signature | ![](https://geps.dev/progress/83) | 83.3% | 404 | 485 |
| src/rails/ext/blocking | ![](https://geps.dev/progress/33) | 32.71% | 174 | 532 |
| src/rails/ext/future | ![](https://geps.dev/progress/98) | 98.49% | 718 | 729 |
| src/rails/ext/future/ext/option | ![](https://geps.dev/progress/84) | 84.19% | 378 | 449 |
| src/rails/ext/future/ext/result | ![](https://geps.dev/progress/81) | 80.98% | 315 | 389 |
| src/rails/tracing | ![](https://geps.dev/progress/100) | 100.0% | 115 | 115 |
| src/serde/dynamic | ![](https://geps.dev/progress/76) | 76.09% | 261 | 343 |
| src/serde/dynamic/test | ![](https://geps.dev/progress/15) | 14.71% | 5 | 34 |
| src/storage | ![](https://geps.dev/progress/0) | 0.0% | 0 | 39 |
| src/storage/io/file | ![](https://geps.dev/progress/58) | 58.44% | 180 | 308 |
| src/sync | ![](https://geps.dev/progress/81) | 81.15% | 538 | 663 |
| src/task | ![](https://geps.dev/progress/97) | 96.84% | 306 | 316 |
| src/task/multicommand | ![](https://geps.dev/progress/63) | 62.64% | 114 | 182 |
| src/template | ![](https://geps.dev/progress/84) | 84.38% | 281 | 333 |
| src/template/pipelines | ![](https://geps.dev/progress/81) | 81.16% | 56 | 69 |
| src/time | ![](https://geps.dev/progress/47) | 47.11% | 1116 | 2369 |



## Http Request (gearbox::net::http::request)

### Complete architectural overview:
```mermaid
classDiagram
    %% Package: request
    namespace request {
        class Client {
            +client: reqwest::Client
            +new()
            +with_client(reqwest::Client)
            +set_global_signing(Signature)
        }

        class Error {
            +UrlParser(ParseError)
            +Request(reqwest::Error)
            +NoUrl
            +HeaderValue(reqwest::header::InvalidHeaderValue)
            +DeserializeContentType(String)
            +DeserializeJson(serde_json::Error)
            +BodyError(TracerError)
        }

        class Method {
            <<enumeration>>
            Get
            Post
            Put
            Delete
            Patch
            Head
            Options
            Connect
            Trace
            None
        }

        class RequestBuilder {
            +client: Option<Arc<Client>>
            +method: Method
            +uri: Option<Url>
            +headers: HeaderMap
            +body: Body
            +content_type: String
            +signature: Option<Signature>
            +new_with_client(client: Option<Client>, method: Method, uri: Url)
            +method(T: Into<Method>)
            +uri(uri: &str)
            +header(H: Into<Header>)
            +headers(H: Into<HeaderMap>)
            +body(B: Into<Body>)
            +content_type(content_type: &str)
            +with_signing_default()
            +with_signing(signature: Signature)
            +send()
        }

        class Response {
            +status: StatusCode
            +headers: HeaderMap
            +content_length: Option<u64>
            +url: Url
            +body: BodyOwned
            +status()
            +to(T: DeserializeOwned)
        }

        class StatusCode {
            +code: u16
            +reason: &'static str
            +as_u16()
            +as_str()
        }

        class Url {
            +Simple(url: String)
        }

        class Body {
            +Empty
        }

        class BodyOwned {
            +from(box_raw: Box<reqwest::Response>)
        }
    }

    %% Relationships
    Client --> RequestBuilder
    RequestBuilder --> Response
    RequestBuilder --> Error
    Response --> StatusCode
    Response --> HeaderMap
    Response --> Url
    Response --> BodyOwned
    HeaderMap --> Header
    Header --> Name
    Header --> Values
    Values --> Value
```


## Http Request Chaining (gearbox::net::http::request_chaining)

### Complete architectural overview:
```mermaid
classDiagram
    %% Package: request
    namespace request {
        class Client {
            +client: reqwest::Client
            +new()
            +with_client(reqwest::Client)
            +set_global_signing(Signature)
        }

        class Error {
            +UrlParser(ParseError)
            +Request(reqwest::Error)
            +NoUrl
            +HeaderValue(reqwest::header::InvalidHeaderValue)
            +DeserializeContentType(String)
            +DeserializeJson(serde_json::Error)
            +BodyError(TracerError)
        }

        class Method {
            <<enumeration>>
            Get
            Post
            Put
            Delete
            Patch
            Head
            Options
            Connect
            Trace
            None
        }

        class RequestBuilder {
            +client: Option<Arc<Client>>
            +method: Method
            +uri: Option<Url>
            +headers: HeaderMap
            +body: Body
            +content_type: String
            +signature: Option<Signature>
            +new_with_client(client: Option<Client>, method: Method, uri: Url)
            +method(T: Into<Method>)
            +uri(uri: &str)
            +header(H: Into<Header>)
            +headers(H: Into<HeaderMap>)
            +body(B: Into<Body>)
            +content_type(content_type: &str)
            +with_signing_default()
            +with_signing(signature: Signature)
            +send()
        }

        class Response {
            +status: StatusCode
            +headers: HeaderMap
            +content_length: Option<u64>
            +url: Url
            +body: BodyOwned
            +status()
            +to(T: DeserializeOwned)
        }

        class StatusCode {
            +code: u16
            +reason: &'static str
            +as_u16()
            +as_str()
        }

        class Url {
            +Simple(url: String)
        }

        class Body {
            +Empty
        }

        class BodyOwned {
            +from(box_raw: Box<reqwest::Response>)
        }
    }

    %% Package: request_chaining
    namespace request_chaining {
        class Header {
            +name: Name
            +values: Values
        }

        class HeaderMap {
            +inner: HashMap<Name, Values>
            +get(K: Into<Name>)
            +insert(header: Header)
            +extend(headers: HeaderMap)
        }

        class Name {
            +String name
        }

        class Values {
            +Vec<Value> values
            +iter()
        }

        class Value {
            +Vec<u8> value
            +as_bytes()
            +to_vec()
        }

        class TracerError
        class Signature
        class ParseError
    }

    %% Relationships
    Client --> RequestBuilder
    RequestBuilder --> Response
    RequestBuilder --> Error
    Response --> StatusCode
    Response --> HeaderMap
    Response --> Url
    Response --> BodyOwned
    HeaderMap --> Header
    Header --> Name
    Header --> Values
    Values --> Value
```

## Signature (gearbox::net::signature)
Payload Signature Config/Generator
This object is for creating a API key signature.

This this example a static nonce is used to generate a API signature. This is to confirm the signature is as expected.
The example is also using the default signature configuration.
```rust
extern crate alloc;

use alloc::sync::Arc;
use gearbox::net::signature::Signature;
use base64;

let mut signing = Signature::default();
let nonce = 1616492376594usize;

let validated_sign = base64::decode("4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==").unwrap();

let cal_sign = signing
  .var("payload", "ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25")
  .var("secret_key", "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==")
  .var("url", "/0/private/AddOrder")
  .nonce(Arc::new(move || -> Vec<u8> {nonce.to_string().as_bytes().to_vec()}))
  .sign();

assert_eq!(validated_sign, cal_sign)
```

At the time of signing is might be usefull to locking the nonce. By locking the nonce you will prevent
change in the next signing.
This is usefull in the default signing configuration, and if the nonce is not predictable.

In this example the signature will only generate a base64 encoded value.

```rust
extern crate alloc;

use alloc::sync::Arc;
use gearbox::net::signature::*;
use base64;

let mut signing = Signature::default();

let cal_sign = signing
    .config(SignCal::Base64Encode(SignCal::VarString("nonce".to_string()).into())).nonce_default();
let nonce = cal_sign.nonce_lock();

let b64_nonce = base64::encode(nonce.unwrap()).into_bytes();


assert_eq!(b64_nonce, cal_sign.sign());
```
> Note:
> Using nonce_lock will lock the nonce until the next signing, as soon as a signing has happened the lock will be removed!
> Also running the lock multiple times will force the signature generator to create new nonce values.



## Dynamic Serialization/Deserialization (gearbox::serde::dynamic)
Simple serde is as its said, a simplified implementation of multiple repositories for
serialization and deserialization.

In Short the goal is to have a single tool for serialization and deserialization, with a common
interface.

### Usage
Simple Serde uses `.encode` and `.decode` for encoding and decoding. Decode can be done on any
`Vec<u8>` or `&[u8]` this allows for the cleanest implementation.
The same goes for anything that needs to be serialized/encoded. Any type that implements the
`#[derive(Serialize)]` can easily be encoded using `.encode`

### Encode/Decode
`.encode` and `.decode` both takes a `ContentType` which defines what you are encoding/decoding
from/to.
an example would be `[some Vec<u8>].decode("bson")` or `my_struct.encode("bson")`.
This is possible as `ContentType` implements the `TryFrom` trait for `&str`, `String`.
In case the implementation is unable to decode what type you are trying to encode/decode from/to
an `Err` result with `Error::UnknownContentTypeMatchFromStr` will be returned from the
encoder/decoder

Anything coming out of the encoder will be of type `Vec<u8>` further the `Vec<u8>` is wrapped in
a struct called `Encoded` this allow for further simplifications on implementation like,
`TryToString` which will automatically try to convert `Encoded` to a `String`, in addition
`Encoded` had implemented the `Deref` and `DerefMut` traits to make it easier to gain access to
encapsulated data.

### Supported formats
- Bson
- Cbor
- FlexBuffers
- Json
- Json5
- Lexpr
- MessagePack
- Pickle
- Postcard
- Ron
- Toml
- Url
- Yaml
- Xml (Awaiting serde-xml-rs v. >0.51)

further all string definitions of `ContentType` is case insensitive, and has an alternate
- `application/[format]`
- `application/x-[format]`

### Serialization/Encode example
```rust
use core::ops::Deref;
use serde::Serialize;
#[macro_use]
use serde_derive;
use gearbox::serde::dynamic::{Encoded, SimpleEncoder, TryToString};

#[derive(Serialize)]
struct Foo {
    bar: String,
}

let my_foo = Foo {
  bar: "foobar".to_string(),
};

let encoded: Encoded = my_foo
  .encode("yaml")
  .expect("Should have been encoded in yaml");

assert_eq!(
    &vec![98, 97, 114, 58, 32, 102, 111, 111, 98, 97, 114, 10],
    encoded.deref()
);
assert_eq!(r#"bar: foobar
"#, encoded.try_to_string().unwrap())
```

### Deserialization/Decode example
```rust
use core::ops::Deref;
use serde::Deserialize;
#[macro_use]
use serde_derive;
use gearbox::serde::dynamic::{Decoded, SimpleDecoder};

#[derive(Deserialize, Debug, PartialEq)]
struct Foo {
    bar: String,
}

let my_foo = Foo {
  bar: "foobar".to_string(),
};

let v_u8_data = &vec![45, 45, 45, 10, 98, 97, 114, 58, 32, 102, 111, 111, 98, 97, 114, 10];
let string_data = r#"---
bar: foobar
"#;

let decoded_from_v_u8: Decoded<Foo> = v_u8_data.decode("yaml").expect("Should have decoded the Vec<u8>");
let decoded_from_string: Decoded<Foo> = string_data.decode("yaml").expect("Should have decoded the String");

assert_eq!(
    Foo{bar: "foobar".to_string()},
    decoded_from_v_u8.into()
);
assert_eq!(
    Foo{bar: "foobar".to_string()},
    decoded_from_string.into()
);
```


## Railway Future extension (gearbox::rails::ext::future)
### FutureOptional and FutureResult Documentation

#### FutureOptional

An extension trait for `Future`s that yield `Option<T>` that provides a variety of convenient adapters.

##### `map`

Map this future's optional output to a different type, returning a new future of the resulting type.

This function is similar to the `Option::map` where it will change the type of the underlying future. This is useful to chain along a computation once a future has been resolved and if it is `Some`.

###### Example

```rust
use gearbox::rails::ext::future::FutureOptional;

let future_opt = async { Some(1) };
let res = future_opt.map(|t| async move { 5 });
let final_res = res.await;
assert_eq!(final_res, Some(5));
```

##### `and_then`

Chains this future with another future if the output is `Some`, returning a new future of the resulting type.

This function is similar to the `Option::and_then` where it will chain another computation if the future resolves to `Some`.

###### Example

```rust
use gearbox::rails::ext::future::FutureOptional;

let future_opt = async { Some(1) };
let res = future_opt.and_then(|t| async move { Some(t + 1) });
let final_res = res.await;
assert_eq!(final_res, Some(2));
```

##### `filter`

Filters the output of this future, returning `None` if the predicate returns `false`.

This function is similar to the `Option::filter` where it will return `None` if the predicate returns `false`.

###### Example

```rust
use gearbox::rails::ext::future::FutureOptional;

let future_opt = async { Some(4) };
let res = future_opt.filter(|x| *x > 2);
let final_res = res.await;
assert_eq!(final_res, Some(4));
```

##### `or`

Returns this future's output if it is `Some`, otherwise returns the provided fallback.

This function is similar to the `Option::or` where it will return the provided fallback if the future resolves to `None`.

###### Example

```rust
use gearbox::rails::ext::future::FutureOptional;

let future_opt = async { Some(4) };
let res = future_opt.or(Some(10));
let final_res = res.await;
assert_eq!(final_res, Some(4));

let future_opt = async { None };
let res = future_opt.or(Some(10));
let final_res = res.await;
assert_eq!(final_res, Some(10));
```

##### `or_else`

Returns this future's output if it is `Some`, otherwise calls the provided fallback function.

This function is similar to the `Option::or_else` where it will call the provided fallback function if the future resolves to `None`.

###### Example

```rust
use gearbox::rails::ext::future::FutureOptional;

let future_opt = async { Some(4) };
let res = future_opt.or_else(|| async { Some(10) });
let final_res = res.await;
assert_eq!(final_res, Some(4));

let future_opt = async { None };
let res = future_opt.or_else(|| async { Some(10) });
let final_res = res.await;
assert_eq!(final_res, Some(10));
```

##### `unwrap_or`

Returns this future's output if it is `Some`, otherwise returns the provided default.

This function is similar to the `Option::unwrap_or` where it will return the provided default if the future resolves to `None`.

###### Example

```rust
use gearbox::rails::ext::future::FutureOptional;

let future_opt = async { Some(4) };
let res = future_opt.unwrap_or(10);
let final_res = res.await;
assert_eq!(final_res, 4);

let future_opt = async { None };
let res = future_opt.unwrap_or(10);
let final_res = res.await;
assert_eq!(final_res, 10);
```

##### `unwrap_or_else`

Returns this future's output if it is `Some`, otherwise calls the provided fallback function.

This function is similar to the `Option::unwrap_or_else` where it will call the provided fallback function if the future resolves to `None`.

###### Example

```rust
use gearbox::rails::ext::future::FutureOptional;

let future_opt = async { Some(4) };
let res = future_opt.unwrap_or_else(|| async { 10 });
let final_res = res.await;
assert_eq!(final_res, 4);

let future_opt = async { None };
let res = future_opt.unwrap_or_else(|| async { 10 });
let final_res = res.await;
assert_eq!(final_res, 10);
```

##### `merge`

Merges this future with an optional value, producing a new future.

This function takes an additional option and a function to combine the resolved value of the future and the option into a new future.

###### Example

```rust
use gearbox::rails::ext::future::FutureOptional;

async fn func(x: u32, y: u32) -> Option<u32> {
    Some(x + y)
}

let x = async { Some(1) };
let y = Some(2);

let res = x.merge(y, |var_x, var_y| func(var_x, var_y));
assert_eq!(res.await, Some(3));
```

##### `merge2`

Merges this future with two optional values, producing a new future.

This function takes two additional options and a function to combine the resolved value of the future and the options into a new future.

###### Example

```rust
use gearbox::rails::ext::future::FutureOptional;

async fn func(x: u32, y: u32, z: u32) -> Option<u32> {
    Some(x + y + z)
}

let x = async { Some(1) };
let y = Some(2);
let z = Some(3);

let res = x.merge2(y, z, |var_x, var_y, var_z| func(var_x, var_y, var_z));
assert_eq!(res.await, Some(6));
```

##### `merge3`

Merges this future with three optional values, producing a new future.

This function takes three additional options and a function to combine the resolved value of the future and the options into a new future.

###### Example

```rust
use gearbox::rails::ext::future::FutureOptional;

async fn func(x: u32, y: u32, z: u32, a: u32) -> Option<u32> {
    Some(x + y + z + a)
}

let x = async { Some(1) };
let y = Some(2);
let z = Some(3);
let a = Some(4);

let res = x.merge3(y, z, a, |var_x, var_y, var_z, var_a| func(var_x, var_y, var_z, var_a));
assert_eq!(res.await, Some(10));
```

##### `merge4`

Merges this future with four optional values, producing a new future.

This function takes four additional options and a function to combine the resolved value of the future and the options into a new future.

###### Example

```rust
use gearbox::rails::ext::future::FutureOptional;

async fn func(x: u32, y: u32, z: u32, a: u32, b: u32) -> Option<u32> {
    Some(x + y + z + a + b)
}

let x = async { Some(1) };
let y = Some(2);
let z = Some(3);
let a = Some(4);
let b = Some(5);

let res = x.merge4(y, z, a, b, |var_x, var_y, var_z, var_a, var_b| func(var_x, var_y, var_z, var_a, var_b));
assert_eq!(res.await, Some(15));
```

#### FutureResult

An extension trait for `Future`s that yield `Result<T, E>` that provides a variety of convenient adapters.

##### `map`

Map this future's result output to a different type, returning a new future of the resulting type.

This function is similar to the `Result::map` where it will change the type of the underlying future. This is useful to chain along a computation once a future has been resolved and if it is `Ok`.

###### Example

```rust
use gearbox::rails::ext::future::FutureResult;

let future_res = async { Ok::<_, ()>(1) };
let res = future_res.map(|t| async move { 5 });
let final_res = res.await;
assert_eq!(final_res, Ok(5));
```

##### `map_or`

Maps a `Result` by applying a function to the contained `Ok` value, or a default value if it is `Err`.

This function is similar to the `Result::map_or`.

###### Example

```rust

 {
use gearbox::rails::ext::future::FutureResult;

let future_res = async { Ok::<_, ()>(1) };
let res = future_res.map_or(10, |t| async move { t + 1 });
let final_res = res.await;
assert_eq!(final_res, 2);

let future_res = async { Err::<i32, _>(()) };
let res = future_res.map_or(10, |t| async move { t + 1 });
let final_res = res.await;
assert_eq!(final_res, 10);
```

##### `map_err`

Maps a `Result` by applying a function to the contained `Err` value.

This function is similar to the `Result::map_err`.

###### Example

```rust
use gearbox::rails::ext::future::FutureResult;

let future_res = async { Err::<u32, _>(1) };
let res = future_res.map_err(|e| async move { e + 1 });
let final_res = res.await;
assert_eq!(final_res, Err(2));
```

##### `and_then`

Chains this future with another future if the output is `Ok`, returning a new future of the resulting type.

This function is similar to the `Result::and_then` where it will chain another computation if the future resolves to `Ok`.

###### Example

```rust
use gearbox::rails::ext::future::FutureResult;

let future_res = async { Ok::<_, ()>(1) };
let res = future_res.and_then(|t| async move { Ok(t + 1) });
let final_res = res.await;
assert_eq!(final_res, Ok(2));
```

##### `or_else`

Returns this future's result if it is `Ok`, otherwise calls the provided fallback function.

This function is similar to the `Result::or_else` where it will call the provided fallback function if the future resolves to `Err`.

###### Example

```rust
use gearbox::rails::ext::future::FutureResult;

let future_res = async { Ok::<_, ()>(4) };
let res = future_res.or_else(|_| async { Ok(10) });
let final_res = res.await;
assert_eq!(final_res, Ok(4));

let future_res = async { Err::<i32, _>(()) };
let res = future_res.or_else(|_| async { Ok(10) });
let final_res = res.await;
assert_eq!(final_res, Ok(10));
```

##### `unwrap_or_else`

Returns this future's result if it is `Ok`, otherwise calls the provided fallback function.

This function is similar to the `Result::unwrap_or_else` where it will call the provided fallback function if the future resolves to `Err`.

###### Example

```rust
use gearbox::rails::ext::future::FutureResult;

let future_res = async { Ok::<_, ()>(4) };
let res = future_res.unwrap_or_else(|_| async { 10 });
let final_res = res.await;
assert_eq!(final_res, 4);

let future_res = async { Err::<i32, _>(()) };
let res = future_res.unwrap_or_else(|_| async { 10 });
let final_res = res.await;
assert_eq!(final_res, 10);
```

##### `merge`

Merges this future with a result value, producing a new future.

This function takes an additional result and a function to combine the resolved value of the future and the result into a new future.

###### Example

```rust
use gearbox::rails::ext::future::FutureResult;

async fn func(x: u32, y: u32) -> Result<u32, ()> {
    Ok(x + y)
}

let x = async { Ok::<_, ()>(1) };
let y = Ok(2);

let res = x.merge(y, |var_x, var_y| func(var_x, var_y));
assert_eq!(res.await, Ok(3));
```

##### `merge2`

Merges this future with two result values, producing a new future.

This function takes two additional results and a function to combine the resolved value of the future and the results into a new future.

###### Example

```rust
use gearbox::rails::ext::future::FutureResult;

async fn func(x: u32, y: u32, z: u32) -> Result<u32, ()> {
    Ok(x + y + z)
}

let x = async { Ok::<_, ()>(1) };
let y = Ok(2);
let z = Ok(3);

let res = x.merge2(y, z, |var_x, var_y, var_z| func(var_x, var_y, var_z));
assert_eq!(res.await, Ok(6));
```

##### `merge3`

Merges this future with three result values, producing a new future.

This function takes three additional results and a function to combine the resolved value of the future and the results into a new future.

###### Example

```rust
use gearbox::rails::ext::future::FutureResult;

async fn func(x: u32, y: u32, z: u32, a: u32) -> Result<u32, ()> {
    Ok(x + y + z + a)
}

let x = async { Ok::<_, ()>(1) };
let y = Ok(2);
let z = Ok(3);
let a = Ok(4);

let res = x.merge3(y, z, a, |var_x, var_y, var_z, var_a| func(var_x, var_y, var_z, var_a));
assert_eq!(res.await, Ok(10));
```

##### `merge4`

Merges this future with four result values, producing a new future.

This function takes four additional results and a function to combine the resolved value of the future and the results into a new future.

###### Example

```rust
use gearbox::rails::ext::future::FutureResult;

async fn func(x: u32, y: u32, z: u32, a: u32, b: u32) -> Result<u32, ()> {
    Ok(x + y + z + a + b)
}

let x = async { Ok::<_, ()>(1) };
let y = Ok(2);
let z = Ok(3);
let a = Ok(4);
let b = Ok(5);

let res = x.merge4(y, z, a, b, |var_x, var_y, var_z, var_a, var_b| func(var_x, var_y, var_z, var_a, var_b));
assert_eq!(res.await, Ok(15));
```


### TODO

- [ ] ( gearbox::log::* ) Clean up Log fmt/syslog, some of the code can be combined and cleaned up a bit better, also the formatter supports syslog, and bunyan, this should probably be cleared up a bit more, and separated better.
- [ ] ( gearbox::path::* ) current this system is mainly just exposing the dirs::* library, this should be removed.
- [ ] ( gearbox::* ) Remove usage for Vec or move usage of std::vec::Vec to another no-std library




Current version: 2.0.0

License: MIT
