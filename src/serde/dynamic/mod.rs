//! Simple serde is as its said, a simplified implementation of multiple repositories for
//! serialization and deserialization.
//!
//! In Short the goal is to have a single tool for serialization and deserialization, with a common
//! interface.
//!
//! ## Usage
//! Simple Serde uses `.encode` and `.decode` for encoding and decoding. Decode can be done on any
//! `Vec<u8>` or `&[u8]` this allows for the cleanest implementation.
//! The same goes for anything that needs to be serialized/encoded. Any type that implements the
//! `#[derive(Serialize)]` can easily be encoded using `.encode`
//!
//! ## Encode/Decode
//! `.encode` and `.decode` both takes a `ContentType` which defines what you are encoding/decoding
//! from/to.
//! an example would be `[some Vec<u8>].decode("bson")` or `my_struct.encode("bson")`.
//! This is possible as `ContentType` implements the `TryFrom` trait for `&str`, `String`.
//! In case the implementation is unable to decode what type you are trying to encode/decode from/to
//! an `Err` result with `Error::UnknownContentTypeMatchFromStr` will be returned from the
//! encoder/decoder
//!
//! Anything coming out of the encoder will be of type `Vec<u8>` further the `Vec<u8>` is wrapped in
//! a struct called `Encoded` this allow for further simplifications on implementation like,
//! `TryToString` which will automatically try to convert `Encoded` to a `String`, in addition
//! `Encoded` had implemented the `Deref` and `DerefMut` traits to make it easier to gain access to
//! encapsulated data.
//!
//! ## Supported formats
//! - Bson
//! - Cbor
//! - FlexBuffers
//! - Json
//! - Json5
//! - Lexpr
//! - MessagePack
//! - Pickle
//! - Postcard
//! - Ron
//! - Toml
//! - Url
//! - Yaml
//! - Xml (Awaiting serde-xml-rs v. >0.51)
//!
//! further all string definitions of `ContentType` is case insensitive, and has an alternate
//! - `application/[format]`
//! - `application/x-[format]`
//!
//! ## Serialization/Encode example
//! ```rust
//! use core::ops::Deref;
//! use serde::Serialize;
//! #[macro_use]
//! use serde_derive;
//! use gearbox::serde::dynamic::{Encoded, SimpleEncoder, TryToString};
//!
//! #[derive(Serialize)]
//! struct Foo {
//!     bar: String,
//! }
//!
//! let my_foo = Foo {
//!   bar: "foobar".to_string(),
//! };
//!
//! let encoded: Encoded = my_foo
//!   .encode("yaml")
//!   .expect("Should have been encoded in yaml");
//!
//! assert_eq!(
//!     &vec![98, 97, 114, 58, 32, 102, 111, 111, 98, 97, 114, 10],
//!     encoded.deref()
//! );
//! assert_eq!(r#"bar: foobar
//! "#, encoded.try_to_string().unwrap())
//! ```
//!
//! ## Deserialization/Decode example
//! ```rust
//! use core::ops::Deref;
//! use serde::Deserialize;
//! #[macro_use]
//! use serde_derive;
//! use gearbox::serde::dynamic::{Decoded, SimpleDecoder};
//!
//! #[derive(Deserialize, Debug, PartialEq)]
//! struct Foo {
//!     bar: String,
//! }
//!
//! let my_foo = Foo {
//!   bar: "foobar".to_string(),
//! };
//!
//! let v_u8_data = &vec![45, 45, 45, 10, 98, 97, 114, 58, 32, 102, 111, 111, 98, 97, 114, 10];
//! let string_data = r#"---
//! bar: foobar
//! "#;
//!
//! let decoded_from_v_u8: Decoded<Foo> = v_u8_data.decode("yaml").expect("Should have decoded the Vec<u8>");
//! let decoded_from_string: Decoded<Foo> = string_data.decode("yaml").expect("Should have decoded the String");
//!
//! assert_eq!(
//!     Foo{bar: "foobar".to_string()},
//!     decoded_from_v_u8.into()
//! );
//! assert_eq!(
//!     Foo{bar: "foobar".to_string()},
//!     decoded_from_string.into()
//! );
//! ```
use core::str::from_utf8;

pub mod prelude {
    #[cfg(feature = "serde-bson")]
    pub extern crate bson;
    #[cfg(feature = "serde-flexbuffers")]
    pub extern crate flexbuffers;
    #[cfg(feature = "serde-json5")]
    pub extern crate json5;
    #[cfg(feature = "serde-postcard")]
    pub extern crate postcard;
    #[cfg(feature = "serde-messagepack")]
    pub extern crate rmp_serde as messagepack;
    #[cfg(feature = "serde-ron")]
    pub extern crate ron;
    #[cfg(feature = "serde-cbor")]
    pub extern crate serde_cbor as cbor;
    pub extern crate serde_derive;
    #[cfg(feature = "serde-json")]
    pub extern crate serde_json as json;
    #[cfg(feature = "serde-lexpr")]
    pub extern crate serde_lexpr as lexpr;
    #[cfg(feature = "serde-pickle")]
    pub extern crate serde_pickle as pickle;
    #[cfg(feature = "serde-query-string")]
    pub extern crate serde_qs as qs;
    #[cfg(feature = "serde-accept-limited-xml-serialize")]
    pub extern crate serde_xml_rs as xml;
    #[cfg(feature = "serde-yaml")]
    pub extern crate serde_yaml as yaml;
}

use core::convert::{Infallible, Into, TryFrom, TryInto};
use core::ops::{Deref, DerefMut};
use core::str::Utf8Error;
#[cfg(feature = "serde-ron")]
use ron::de::SpannedError;
use serde::de::DeserializeOwned;
use serde::Serialize;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(PartialEq, Eq, Debug)]
pub enum ContentType {
    #[cfg(feature = "serde-bson")]
    Bson,
    #[cfg(feature = "serde-cbor")]
    Cbor,
    #[cfg(feature = "serde-flexbuffers")]
    FlexBuffers,
    #[cfg(feature = "serde-json")]
    Json,
    #[cfg(feature = "serde-json5")]
    Json5,
    #[cfg(feature = "serde-lexpr")]
    Lexpr,
    #[cfg(feature = "serde-messagepack")]
    MessagePack,
    #[cfg(feature = "serde-pickle")]
    Pickle,
    #[cfg(feature = "serde-postcard")]
    Postcard,
    #[cfg(feature = "serde-ron")]
    Ron,
    #[cfg(feature = "serde-toml")]
    Toml,
    #[cfg(feature = "serde-query-string")]
    QueryString,
    #[cfg(feature = "serde-yaml")]
    Yaml,
    #[cfg(feature = "serde-accept-limited-xml-serialize")]
    Xml,
}

impl TryFrom<&str> for ContentType {
    type Error = Error;

    fn try_from(s: &str) -> core::result::Result<ContentType, Self::Error> {
        #[allow(unreachable_patterns)]
        match s.to_lowercase().as_str() {
            #[cfg(feature = "serde-bson")]
            "bson" => Ok(ContentType::Bson),
            #[cfg(feature = "serde-bson")]
            "application/bson" => Ok(ContentType::Bson),
            #[cfg(feature = "serde-bson")]
            "application/x-bson" => Ok(ContentType::Bson),
            #[cfg(feature = "serde-cbor")]
            "cbor" => Ok(ContentType::Cbor),
            #[cfg(feature = "serde-cbor")]
            "application/cbor" => Ok(ContentType::Cbor),
            #[cfg(feature = "serde-cbor")]
            "application/x-cbor" => Ok(ContentType::Cbor),
            #[cfg(feature = "serde-flexbuffers")]
            "flexbuffers" => Ok(ContentType::FlexBuffers),
            #[cfg(feature = "serde-flexbuffers")]
            "application/flexbuffers" => Ok(ContentType::FlexBuffers),
            #[cfg(feature = "serde-flexbuffers")]
            "application/x-flexbuffers" => Ok(ContentType::FlexBuffers),
            #[cfg(feature = "serde-json")]
            "json" => Ok(ContentType::Json),
            #[cfg(feature = "serde-json")]
            "application/json" => Ok(ContentType::Json),
            #[cfg(feature = "serde-json")]
            "application/x-json" => Ok(ContentType::Json),
            #[cfg(feature = "serde-json5")]
            "json5" => Ok(ContentType::Json5),
            #[cfg(feature = "serde-json5")]
            "application/json5" => Ok(ContentType::Json5),
            #[cfg(feature = "serde-json5")]
            "application/x-json5" => Ok(ContentType::Json5),
            #[cfg(feature = "serde-lexpr")]
            "lexpr" => Ok(ContentType::Lexpr),
            #[cfg(feature = "serde-lexpr")]
            "application/lexpr" => Ok(ContentType::Lexpr),
            #[cfg(feature = "serde-lexpr")]
            "application/x-lexpr" => Ok(ContentType::Lexpr),
            #[cfg(feature = "serde-messagepack")]
            "messagepack" => Ok(ContentType::MessagePack),
            #[cfg(feature = "serde-messagepack")]
            "application/messagepack" => Ok(ContentType::MessagePack),
            #[cfg(feature = "serde-messagepack")]
            "application/x-messagepack" => Ok(ContentType::MessagePack),
            #[cfg(feature = "serde-pickle")]
            "pickle" => Ok(ContentType::Pickle),
            #[cfg(feature = "serde-pickle")]
            "application/pickle" => Ok(ContentType::Pickle),
            #[cfg(feature = "serde-pickle")]
            "application/x-pickle" => Ok(ContentType::Pickle),
            #[cfg(feature = "serde-postcard")]
            "postcard" => Ok(ContentType::Postcard),
            #[cfg(feature = "serde-postcard")]
            "application/postcard" => Ok(ContentType::Postcard),
            #[cfg(feature = "serde-postcard")]
            "application/x-postcard" => Ok(ContentType::Postcard),
            #[cfg(feature = "serde-ron")]
            "ron" => Ok(ContentType::Ron),
            #[cfg(feature = "serde-ron")]
            "application/ron" => Ok(ContentType::Ron),
            #[cfg(feature = "serde-ron")]
            "application/x-ron" => Ok(ContentType::Ron),
            #[cfg(feature = "serde-toml")]
            "toml" => Ok(ContentType::Toml),
            #[cfg(feature = "serde-toml")]
            "application/toml" => Ok(ContentType::Toml),
            #[cfg(feature = "serde-toml")]
            "application/x-toml" => Ok(ContentType::Toml),
            #[cfg(feature = "serde-query-string")]
            "querystring" => Ok(ContentType::QueryString),
            #[cfg(feature = "serde-query-string")]
            "application/querystring" => Ok(ContentType::QueryString),
            #[cfg(feature = "serde-query-string")]
            "application/x-querystring" => Ok(ContentType::QueryString),
            #[cfg(feature = "serde-yaml")]
            "yaml" => Ok(ContentType::Yaml),
            #[cfg(feature = "serde-yaml")]
            "application/yaml" => Ok(ContentType::Yaml),
            #[cfg(feature = "serde-yaml")]
            "application/x-yaml" => Ok(ContentType::Yaml),
            #[cfg(feature = "serde-accept-limited-xml-serialize")]
            "xml" => Ok(ContentType::Xml),
            #[cfg(feature = "serde-accept-limited-xml-serialize")]
            "application/xml" => Ok(ContentType::Xml),
            #[cfg(feature = "serde-accept-limited-xml-serialize")]
            "application/x-xml" => Ok(ContentType::Xml),
            _ => Err(Error::UnknownContentTypeMatchFromStr(s.to_string())),
        }
    }
}

impl TryFrom<String> for ContentType {
    type Error = Error;

    fn try_from(s: String) -> core::result::Result<ContentType, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl TryFrom<&String> for ContentType {
    type Error = Error;

    fn try_from(s: &String) -> core::result::Result<ContentType, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl TryFrom<&ContentType> for ContentType {
    type Error = Error;

    fn try_from(h: &ContentType) -> core::result::Result<ContentType, Self::Error> {
        match h {
            #[cfg(feature = "serde-bson")]
            Self::Bson => Ok(Self::Bson),
            #[cfg(feature = "serde-cbor")]
            Self::Cbor => Ok(Self::Cbor),
            #[cfg(feature = "serde-flexbuffers")]
            Self::FlexBuffers => Ok(Self::FlexBuffers),
            #[cfg(feature = "serde-json")]
            Self::Json => Ok(Self::Json),
            #[cfg(feature = "serde-json5")]
            Self::Json5 => Ok(Self::Json5),
            #[cfg(feature = "serde-lexpr")]
            Self::Lexpr => Ok(Self::Lexpr),
            #[cfg(feature = "serde-messagepack")]
            Self::MessagePack => Ok(Self::MessagePack),
            #[cfg(feature = "serde-pickle")]
            Self::Pickle => Ok(Self::Pickle),
            #[cfg(feature = "serde-postcard")]
            Self::Postcard => Ok(Self::Postcard),
            #[cfg(feature = "serde-ron")]
            Self::Ron => Ok(Self::Ron),
            #[cfg(feature = "serde-toml")]
            Self::Toml => Ok(Self::Toml),
            #[cfg(feature = "serde-query-string")]
            Self::QueryString => Ok(Self::QueryString),
            #[cfg(feature = "serde-yaml")]
            Self::Yaml => Ok(Self::Yaml),
            #[cfg(feature = "serde-accept-limited-xml-serialize")]
            Self::Xml => Ok(Self::Xml),
            _ => Err(Self::Error::NoSerializersDeserializersSet),
        }
    }
}

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Infallible - This error should have been infallible")]
    Infallible,
    #[display(fmt = "Converting Raw Data to UTF8 failed: {}", _0)]
    ByteToUTF8ConversionFailure(Utf8Error),
    #[display(fmt = "Unknown content type match from str: {}", _0)]
    UnknownContentTypeMatchFromStr(String),
    #[cfg(feature = "serde-bson")]
    #[display(fmt = "BSON encoder/decoder error: {}", _0)]
    BsonSerializationFailure(bson::ser::Error),
    #[cfg(feature = "serde-bson")]
    #[display(fmt = "BSON encode/decoder error: {}", _0)]
    BsonDeserializationFailure(bson::de::Error),
    #[cfg(feature = "serde-cbor")]
    #[display(fmt = "CBOR encoder/decoder error: {}", _0)]
    CborFailure(serde_cbor::Error),
    #[cfg(feature = "serde-flexbuffers")]
    #[display(fmt = "Flexbuffers encoder/decoder error: {}", _0)]
    FlexBuffersSerializationFailure(flexbuffers::SerializationError),
    #[cfg(feature = "serde-flexbuffers")]
    #[display(fmt = "Flexbuffers encoder/decoder error: {}", _0)]
    FlexBuffersDeserializationFailure(flexbuffers::DeserializationError),
    #[cfg(feature = "serde-json")]
    #[display(fmt = "JSON encoder/decoder error: {}", _0)]
    JsonError(serde_json::Error),
    #[cfg(feature = "serde-json5")]
    #[display(fmt = "JSON5 encoder/decoder error: {}", _0)]
    Json5Error(json5::Error),
    #[cfg(feature = "serde-lexpr")]
    #[display(fmt = "LEXPR encoder/decoder error: {}", _0)]
    LexprError(serde_lexpr::Error),
    #[cfg(feature = "serde-messagepack")]
    #[display(fmt = "MessagePack encoder/decoder error: {}", _0)]
    MessagePackEncodeError(rmp_serde::encode::Error),
    #[cfg(feature = "serde-messagepack")]
    #[display(fmt = "MessagePack encoder/decoder error: {}", _0)]
    MessagePackDecodeError(rmp_serde::decode::Error),
    #[cfg(feature = "serde-pickle")]
    #[display(fmt = "Pickle encoder/decoder error: {}", _0)]
    PickleError(serde_pickle::Error),
    #[cfg(feature = "serde-postcard")]
    #[display(fmt = "Postcard encoder/decoder error: {}", _0)]
    PostcardError(postcard::Error),
    #[cfg(feature = "serde-ron")]
    #[display(fmt = "RON encoder/decoder error: {}", _0)]
    RonError(ron::Error),
    #[cfg(feature = "serde-ron")]
    #[display(fmt = "RON decoder error: {}", _0)]
    RonDecodeError(ron::de::SpannedError),
    #[cfg(feature = "serde-toml")]
    #[display(fmt = "TOML encoder/decoder error: {}", _0)]
    TomlSerializationFailure(toml::ser::Error),
    #[cfg(feature = "serde-toml")]
    #[display(fmt = "TOML encoder/decoder error: {}", _0)]
    TomlDeserializationFailure(toml::de::Error),
    #[cfg(feature = "serde-query-string")]
    #[display(fmt = "URL encoder/decoder error: {}", _0)]
    QueryStringEncodingFailure(serde_qs::Error),
    #[cfg(feature = "serde-yaml")]
    #[display(fmt = "YAML encoder/decoder error: {}", _0)]
    YamlError(serde_yaml::Error),
    #[cfg(feature = "serde-accept-limited-xml-serialize")]
    #[display(fmt = "XML encoder/decoder error: {}", _0)]
    XmlError(prelude::xml::Error),
    #[display(fmt = "Type is not supported for encoding/decoding: {:?}", _0)]
    TypeDoesNotSupportSerialization(ContentType),
    #[display(fmt = "This would only happen if no serializers/deserializers have been set")]
    NoSerializersDeserializersSet,
}

// Test for this from is disabled as its not possible to create the external
// `core::convert::Infallible` object
#[cfg(not(tarpaulin_include))]
impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Error::Infallible
    }
}

// Unable to test due to no access to object
#[cfg(not(tarpaulin_include))]
impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::ByteToUTF8ConversionFailure(e)
    }
}

#[cfg(feature = "serde-bson")]
impl From<bson::ser::Error> for Error {
    fn from(e: bson::ser::Error) -> Self {
        Error::BsonSerializationFailure(e)
    }
}

#[cfg(feature = "serde-bson")]
impl From<bson::de::Error> for Error {
    fn from(e: bson::de::Error) -> Self {
        Error::BsonDeserializationFailure(e)
    }
}

#[cfg(feature = "serde-cbor")]
impl From<serde_cbor::Error> for Error {
    fn from(e: serde_cbor::Error) -> Self {
        Error::CborFailure(e)
    }
}

#[cfg(feature = "serde-flexbuffers")]
impl From<flexbuffers::SerializationError> for Error {
    fn from(e: flexbuffers::SerializationError) -> Self {
        Error::FlexBuffersSerializationFailure(e)
    }
}

#[cfg(feature = "serde-flexbuffers")]
impl From<flexbuffers::DeserializationError> for Error {
    fn from(e: flexbuffers::DeserializationError) -> Self {
        Error::FlexBuffersDeserializationFailure(e)
    }
}

#[cfg(feature = "serde-json")]
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonError(e)
    }
}

#[cfg(feature = "serde-json5")]
impl From<json5::Error> for Error {
    fn from(e: json5::Error) -> Self {
        Error::Json5Error(e)
    }
}

#[cfg(feature = "serde-lexpr")]
impl From<serde_lexpr::Error> for Error {
    fn from(e: serde_lexpr::Error) -> Self {
        Error::LexprError(e)
    }
}

#[cfg(feature = "serde-messagepack")]
impl From<rmp_serde::encode::Error> for Error {
    fn from(e: rmp_serde::encode::Error) -> Self {
        Error::MessagePackEncodeError(e)
    }
}

#[cfg(feature = "serde-messagepack")]
impl From<rmp_serde::decode::Error> for Error {
    fn from(e: rmp_serde::decode::Error) -> Self {
        Error::MessagePackDecodeError(e)
    }
}

#[cfg(feature = "serde-pickle")]
impl From<serde_pickle::Error> for Error {
    fn from(e: serde_pickle::Error) -> Self {
        Error::PickleError(e)
    }
}

#[cfg(feature = "serde-postcard")]
impl From<postcard::Error> for Error {
    fn from(e: postcard::Error) -> Self {
        Error::PostcardError(e)
    }
}

#[cfg(feature = "serde-ron")]
impl From<ron::Error> for Error {
    fn from(e: ron::Error) -> Self {
        Error::RonError(e)
    }
}

#[cfg(feature = "serde-toml")]
impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error::TomlSerializationFailure(e)
    }
}

#[cfg(feature = "serde-toml")]
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::TomlDeserializationFailure(e)
    }
}

#[cfg(feature = "serde-query-string")]
impl From<serde_qs::Error> for Error {
    fn from(e: serde_qs::Error) -> Self {
        Error::QueryStringEncodingFailure(e)
    }
}

#[cfg(feature = "serde-yaml")]
impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::YamlError(e)
    }
}

#[cfg(feature = "serde-accept-limited-xml-serialize")]
impl From<prelude::xml::Error> for Error {
    fn from(e: prelude::xml::Error) -> Self {
        Error::XmlError(e)
    }
}

#[cfg(feature = "serde-ron")]
impl From<ron::de::SpannedError> for Error {
    fn from(e: ron::de::SpannedError) -> Self {
        Self::RonDecodeError(e)
    }
}

pub trait TryToString {
    type Error;
    fn try_to_string(&self) -> core::result::Result<String, Self::Error>;
}

pub trait SimpleEncoder
where
    Self: serde::Serialize,
{
    fn encode<F: TryInto<ContentType, Error = impl Into<Error>>>(
        &self,
        content_type: F,
    ) -> Result<Encoded>;
}

impl<T> SimpleEncoder for T
where
    T: Serialize,
{
    fn encode<F: TryInto<ContentType, Error = impl Into<Error>>>(
        &self,
        content_type: F,
    ) -> Result<Encoded> {
        #[cfg(feature = "serde-bson")]
        let bson = |o: &T| -> Result<Encoded> { bson::to_vec(o).try_into() };
        #[cfg(feature = "serde-cbor")]
        let cbor = |o: &T| -> Result<Encoded> { serde_cbor::to_vec(o).try_into() };
        #[cfg(feature = "serde-flexbuffers")]
        let flexbuffers = |o: &T| -> Result<Encoded> { flexbuffers::to_vec(o).try_into() };
        #[cfg(feature = "serde-json")]
        let json = |o: &T| -> Result<Encoded> { serde_json::to_vec(o).try_into() };
        #[cfg(feature = "serde-json5")]
        let json5 = |o: &T| -> Result<Encoded> { json5::to_string(o).try_into() };
        #[cfg(feature = "serde-lexpr")]
        let lexpr = |o: &T| -> Result<Encoded> { serde_lexpr::to_vec(o).try_into() };
        #[cfg(feature = "serde-messagepack")]
        let message_pack = |o: &T| -> Result<Encoded> { rmp_serde::to_vec(o).try_into() };
        #[cfg(feature = "serde-pickle")]
        let pickle =
            |o: &T| -> Result<Encoded> { serde_pickle::to_vec(o, Default::default()).try_into() };
        #[cfg(feature = "serde-postcard")]
        let postcard = |o: &T| -> Result<Encoded> { postcard::to_allocvec(o).try_into() };
        #[cfg(feature = "serde-ron")]
        let ron = |o: &T| -> Result<Encoded> { ron::to_string(o).try_into() };
        #[cfg(feature = "serde-toml")]
        let toml = |o: &T| -> Result<Encoded> { toml::to_string(o).try_into() };
        #[cfg(feature = "serde-query-string")]
        let querystring = |o: &T| -> Result<Encoded> { serde_qs::to_string(o).try_into() };
        #[cfg(feature = "serde-yaml")]
        let yaml = |o: &T| -> Result<Encoded> { serde_yaml::to_string(o).try_into() };
        #[cfg(feature = "serde-accept-limited-xml-serialize")]
        let xml = |o: &T| -> Result<Encoded> { prelude::xml::to_string(o).try_into() };
        match content_type.try_into().map_err(|e| e.into())? {
            #[cfg(feature = "serde-bson")]
            ContentType::Bson => bson(self),
            #[cfg(feature = "serde-cbor")]
            ContentType::Cbor => cbor(self),
            #[cfg(feature = "serde-flexbuffers")]
            ContentType::FlexBuffers => flexbuffers(self),
            #[cfg(feature = "serde-json")]
            ContentType::Json => json(self),
            #[cfg(feature = "serde-json5")]
            ContentType::Json5 => json5(self),
            #[cfg(feature = "serde-lexpr")]
            ContentType::Lexpr => lexpr(self),
            #[cfg(feature = "serde-messagepack")]
            ContentType::MessagePack => message_pack(self),
            #[cfg(feature = "serde-pickle")]
            ContentType::Pickle => pickle(self),
            #[cfg(feature = "serde-postcard")]
            ContentType::Postcard => postcard(self),
            #[cfg(feature = "serde-ron")]
            ContentType::Ron => ron(self),
            #[cfg(feature = "serde-toml")]
            ContentType::Toml => toml(self),
            #[cfg(feature = "serde-query-string")]
            ContentType::QueryString => querystring(self),
            #[cfg(feature = "serde-yaml")]
            ContentType::Yaml => yaml(self),
            #[cfg(feature = "serde-accept-limited-xml-serialize")]
            ContentType::Xml => xml(self),
        }
    }
}

pub trait SimpleDecoder<T> {
    fn decode<F: TryInto<ContentType, Error = impl Into<Error>>>(
        &self,
        content_type: F,
    ) -> Result<T>;
}

impl<T> SimpleDecoder<Decoded<T>> for &[u8]
where
    T: DeserializeOwned,
{
    fn decode<F: TryInto<ContentType, Error = impl Into<Error>>>(
        &self,
        content_type: F,
    ) -> Result<Decoded<T>> {
        #[cfg(feature = "serde-bson")]
        let bson = |o: &[u8]| -> Result<Decoded<T>> { bson::from_slice(o).try_into() };
        #[cfg(feature = "serde-cbor")]
        let cbor = |o: &[u8]| -> Result<Decoded<T>> { serde_cbor::from_slice(o).try_into() };
        #[cfg(feature = "serde-flexbuffers")]
        let flexbuffers =
            |o: &[u8]| -> Result<Decoded<T>> { flexbuffers::from_slice(o).try_into() };
        #[cfg(feature = "serde-json")]
        let json = |o: &[u8]| -> Result<Decoded<T>> { serde_json::from_slice(o).try_into() };
        #[cfg(feature = "serde-json5")]
        let json5 = |o: &[u8]| -> Result<Decoded<T>> {
            from_utf8(o)
                .map_err(Error::from)
                .and_then(|str| json5::from_str(str).try_into())
        };
        #[cfg(feature = "serde-lexpr")]
        let lexpr = |o: &[u8]| -> Result<Decoded<T>> { serde_lexpr::from_slice(o).try_into() };
        #[cfg(feature = "serde-messagepack")]
        let message_pack = |o: &[u8]| -> Result<Decoded<T>> { rmp_serde::from_slice(o).try_into() };
        #[cfg(feature = "serde-pickle")]
        let pickle = |o: &[u8]| -> Result<Decoded<T>> {
            serde_pickle::from_slice(o, Default::default()).try_into()
        };
        #[cfg(feature = "serde-postcard")]
        let postcard = |o: &[u8]| -> Result<Decoded<T>> { postcard::from_bytes(o).try_into() };
        #[cfg(feature = "serde-ron")]
        let ron = |o: &[u8]| -> Result<Decoded<T>> {
            from_utf8(o)
                .map_err(Error::from)
                .and_then(|str| ron::from_str(str).try_into())
        };
        #[cfg(feature = "serde-toml")]
        let toml = |o: &[u8]| -> Result<Decoded<T>> {
            from_utf8(o)
                .map_err(Error::from)
                .and_then(|t| toml::from_str(t).try_into())
        };
        #[cfg(feature = "serde-query-string")]
        let querystring = |o: &[u8]| -> Result<Decoded<T>> { serde_qs::from_bytes(o).try_into() };
        #[cfg(feature = "serde-yaml")]
        let yaml = |o: &[u8]| -> Result<Decoded<T>> { serde_yaml::from_slice(o).try_into() };
        #[cfg(feature = "serde-accept-limited-xml-serialize")]
        let xml = |o: &[u8]| -> Result<Decoded<T>> {
            from_utf8(o)
                .map_err(Error::from)
                .and_then(|str| prelude::xml::de::from_str(str).try_into())
        };
        match content_type.try_into().map_err(|e| e.into())? {
            #[cfg(feature = "serde-bson")]
            ContentType::Bson => bson(self),
            #[cfg(feature = "serde-cbor")]
            ContentType::Cbor => cbor(self),
            #[cfg(feature = "serde-flexbuffers")]
            ContentType::FlexBuffers => flexbuffers(self),
            #[cfg(feature = "serde-json")]
            ContentType::Json => json(self),
            #[cfg(feature = "serde-json5")]
            ContentType::Json5 => json5(self),
            #[cfg(feature = "serde-lexpr")]
            ContentType::Lexpr => lexpr(self),
            #[cfg(feature = "serde-messagepack")]
            ContentType::MessagePack => message_pack(self),
            #[cfg(feature = "serde-pickle")]
            ContentType::Pickle => pickle(self),
            #[cfg(feature = "serde-postcard")]
            ContentType::Postcard => postcard(self),
            #[cfg(feature = "serde-ron")]
            ContentType::Ron => ron(self),
            #[cfg(feature = "serde-toml")]
            ContentType::Toml => toml(self),
            #[cfg(feature = "serde-query-string")]
            ContentType::QueryString => querystring(self),
            #[cfg(feature = "serde-yaml")]
            ContentType::Yaml => yaml(self),
            #[cfg(feature = "serde-accept-limited-xml-serialize")]
            ContentType::Xml => xml(self),
        }
    }
}

impl<T> SimpleDecoder<Decoded<T>> for Vec<u8>
where
    T: DeserializeOwned,
{
    fn decode<F: TryInto<ContentType, Error = impl Into<Error>>>(
        &self,
        content_type: F,
    ) -> Result<Decoded<T>> {
        self.as_slice().decode(content_type)
    }
}

impl<T> SimpleDecoder<Decoded<T>> for &str
where
    T: DeserializeOwned,
{
    fn decode<F: TryInto<ContentType, Error = impl Into<Error>>>(
        &self,
        content_type: F,
    ) -> Result<Decoded<T>> {
        self.as_bytes().decode(content_type)
    }
}

impl<T> SimpleDecoder<Decoded<T>> for String
where
    T: DeserializeOwned,
{
    fn decode<F: TryInto<ContentType, Error = impl Into<Error>>>(
        &self,
        content_type: F,
    ) -> Result<Decoded<T>> {
        self.as_bytes().decode(content_type)
    }
}

pub struct Encoded {
    inner: Vec<u8>,
}

impl PartialEq<Self> for Encoded {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl PartialEq<String> for Encoded {
    fn eq(&self, other: &String) -> bool {
        if let Ok(self_string) = self.try_to_string() {
            self_string == *other
        } else {
            false
        }
    }
}

impl PartialEq<&str> for Encoded {
    fn eq(&self, other: &&str) -> bool {
        if let Ok(self_string) = self.try_to_string() {
            self_string == *other
        } else {
            false
        }
    }
}

impl Eq for Encoded {}

impl From<Vec<u8>> for Encoded {
    fn from(v: Vec<u8>) -> Self {
        Encoded { inner: v }
    }
}

impl From<String> for Encoded {
    fn from(s: String) -> Self {
        Encoded {
            inner: s.as_bytes().to_vec(),
        }
    }
}

impl From<&str> for Encoded {
    fn from(s: &str) -> Self {
        Encoded {
            inner: s.as_bytes().to_vec(),
        }
    }
}

impl Deref for Encoded {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Encoded {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<E> TryFrom<core::result::Result<Vec<u8>, E>> for Encoded
where
    E: Into<Error>,
{
    type Error = Error;
    fn try_from(
        value: core::result::Result<Vec<u8>, E>,
    ) -> core::result::Result<Self, Self::Error> {
        value.map(Encoded::from).map_err(|e| e.into())
    }
}

impl<E> TryFrom<core::result::Result<String, E>> for Encoded
where
    E: Into<Error>,
{
    type Error = Error;
    fn try_from(value: core::result::Result<String, E>) -> core::result::Result<Self, Self::Error> {
        value.map(Encoded::from).map_err(|e| e.into())
    }
}

impl<E> TryFrom<core::result::Result<&str, E>> for Encoded
where
    E: Into<Error>,
{
    type Error = Error;
    fn try_from(value: core::result::Result<&str, E>) -> core::result::Result<Self, Self::Error> {
        value.map(Encoded::from).map_err(|e| e.into())
    }
}

impl TryToString for Encoded {
    type Error = Error;
    fn try_to_string(&self) -> core::result::Result<String, Self::Error> {
        from_utf8(self).map_err(Error::from).map(|s| s.to_string())
    }
}

pub struct Decoded<T>
where
    T: DeserializeOwned,
{
    pub(crate) inner: T,
}

impl<T, E> TryFrom<core::result::Result<T, E>> for Decoded<T>
where
    T: DeserializeOwned,
    E: Into<Error>,
{
    type Error = Error;

    fn try_from(res: core::result::Result<T, E>) -> core::result::Result<Self, Self::Error> {
        res.map_err(|e| e.into()).map(Decoded::from)
    }
}

impl<T: DeserializeOwned> From<T> for Decoded<T> {
    fn from(t: T) -> Self {
        Decoded { inner: t }
    }
}

impl<T: DeserializeOwned> Deref for Decoded<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: DeserializeOwned> DerefMut for Decoded<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: DeserializeOwned> Decoded<T> {
    pub fn into(self) -> T {
        self.inner
    }
}

#[cfg(test)]
mod test {
    mod test_constants;
    mod test_trait_impl;

    use super::{ContentType, Decoded, Encoded, Error, SimpleDecoder, SimpleEncoder, TryToString};
    use core::ops::Deref;
    use serde::{Deserialize, Serialize};
    use test_constants::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct MyStruct {
        unquoted: String,
        #[serde(rename = "singleQuotes")]
        single_quotes: String,
        #[serde(rename = "lineBreaks")]
        line_breaks: String,
        hexadecimal: i32,
        #[serde(rename = "leadingDecimalPoint")]
        leading_decimal_point: f64,
        #[serde(rename = "andTrailing")]
        and_trailing: f64,
        #[serde(rename = "positiveSign")]
        positive_sign: i32,
        #[serde(rename = "trailingComma")]
        trailing_comma: String,
        #[serde(rename = "andIn")]
        and_in: Vec<String>,
        #[serde(rename = "backwardsCompatible")]
        backwards_compatible: String,
    }

    impl Default for MyStruct {
        fn default() -> Self {
            MyStruct {
                unquoted: "and you can quote me on that".to_string(),
                single_quotes: "I can use \"double quotes\" here".to_string(),
                line_breaks: "Look, Mom! No \\n's!".to_string(),
                hexadecimal: 0xdecaf,
                leading_decimal_point: 0.8675309,
                and_trailing: 8675309.0,
                positive_sign: 1,
                trailing_comma: "in objects".to_string(),
                and_in: vec!["arrays".to_string(), "arrays-2".to_string()],
                backwards_compatible: "with JSON".to_string(),
            }
        }
    }

    fn deserialize_test(ser_type: &str, compare_object: &[u8]) {
        for i in ["", "application/", "application/x-"] {
            let content_type = format!("{}{}", i, ser_type);
            let my_struct: Decoded<MyStruct> = compare_object.decode(content_type).unwrap();
            println!("Deserialize {} -> {:?}", ser_type, my_struct.deref());
            assert_eq!(my_struct.into(), MyStruct::default());
        }
    }

    fn serialize_test(ser_type: &str, compare_object: &[u8]) {
        for i in ["", "application/", "application/x-"] {
            let my_struct = MyStruct::default();
            let serialized = my_struct.encode(format!("{}{}", i, ser_type)).unwrap();
            if let Ok(s) = serialized.try_to_string() {
                println!("Serialize {} -> {}", ser_type, s);
            } else {
                println!("Serialize {} -> {:?}", ser_type, serialized.deref());
            }

            if compare_object != serialized.deref() {
                println!(
                    "Expected: {}",
                    String::from_utf8(compare_object.to_vec()).unwrap()
                );
                println!(
                    "Actual: {}",
                    String::from_utf8(serialized.deref().to_vec()).unwrap()
                );
            }

            assert_eq!(compare_object, serialized.deref());
        }
    }

    #[test]
    fn unknown_content() {
        assert_eq!(
            Error::UnknownContentTypeMatchFromStr("Foobar".into()),
            ContentType::try_from("Foobar").unwrap_err()
        );
    }

    #[test]
    #[cfg(feature = "serde-bson")]
    fn test_from_str() {
        assert_eq!(ContentType::Bson, "Bson".try_into().unwrap());
    }

    #[test]
    #[cfg(feature = "serde-bson")]
    fn test_from_ref_string() {
        assert_eq!(ContentType::Bson, (&"Bson".to_string()).try_into().unwrap());
    }

    #[test]
    fn test_from_ref_self() {
        #[cfg(feature = "serde-bson")]
        assert_eq!(
            ContentType::Bson,
            ContentType::try_from(&ContentType::Bson).unwrap()
        );
        #[cfg(feature = "serde-cbor")]
        assert_eq!(
            ContentType::Cbor,
            ContentType::try_from(&ContentType::Cbor).unwrap()
        );
        #[cfg(feature = "serde-flexbuffers")]
        assert_eq!(
            ContentType::FlexBuffers,
            ContentType::try_from(&ContentType::FlexBuffers).unwrap()
        );
        #[cfg(feature = "serde-json")]
        assert_eq!(
            ContentType::Json,
            ContentType::try_from(&ContentType::Json).unwrap()
        );
        #[cfg(feature = "serde-json5")]
        assert_eq!(
            ContentType::Json5,
            ContentType::try_from(&ContentType::Json5).unwrap()
        );
        #[cfg(feature = "serde-lexpr")]
        assert_eq!(
            ContentType::Lexpr,
            ContentType::try_from(&ContentType::Lexpr).unwrap()
        );
        #[cfg(feature = "serde-messagepack")]
        assert_eq!(
            ContentType::MessagePack,
            ContentType::try_from(&ContentType::MessagePack).unwrap()
        );
        #[cfg(feature = "serde-postcard")]
        assert_eq!(
            ContentType::Postcard,
            ContentType::try_from(&ContentType::Postcard).unwrap()
        );
        #[cfg(feature = "serde-ron")]
        assert_eq!(
            ContentType::Ron,
            ContentType::try_from(&ContentType::Ron).unwrap()
        );
        #[cfg(feature = "serde-toml")]
        assert_eq!(
            ContentType::Toml,
            ContentType::try_from(&ContentType::Toml).unwrap()
        );
        #[cfg(feature = "serde-query-string")]
        assert_eq!(
            ContentType::QueryString,
            ContentType::try_from(&ContentType::QueryString).unwrap()
        );
        #[cfg(feature = "serde-pickle")]
        assert_eq!(
            ContentType::Yaml,
            ContentType::try_from(&ContentType::Yaml).unwrap()
        );

        #[cfg(feature = "serde-pickle")]
        assert_eq!(
            ContentType::Pickle,
            ContentType::try_from(&ContentType::Pickle).unwrap()
        );
        #[cfg(feature = "serde-accept-limited-xml-serialize")]
        assert_eq!(
            ContentType::Xml,
            ContentType::try_from(&ContentType::Xml).unwrap()
        );
    }

    #[test]
    #[cfg(feature = "serde-json")]
    fn test_simple_serialization() {
        let my_struct = MyStruct::default();
        assert_eq!(
            EXAMPLE_JSON_SERIALIZE.as_bytes(),
            my_struct.encode("json").unwrap().deref()
        );
        assert_eq!(
            EXAMPLE_JSON_SERIALIZE.as_bytes(),
            my_struct.encode("application/json").unwrap().deref()
        );
    }

    #[test]
    #[cfg(feature = "serde-json")]
    fn test_simple_deserialization() {
        let my_struct: Decoded<MyStruct> =
            EXAMPLE_JSON_DESERIALIZE.as_bytes().decode("json").unwrap();
        assert_eq!(my_struct.into(), MyStruct::default());
    }

    #[test]
    #[cfg(feature = "serde-yaml")]
    fn test_yaml() {
        deserialize_test("yaml", EXAMPLE_YAML_DESERIALIZE.as_bytes());
        serialize_test("yaml", EXAMPLE_YAML_SERIALIZE.as_bytes());
    }

    #[test]
    #[cfg(feature = "serde-json5")]
    fn test_json5() {
        deserialize_test("json5", EXAMPLE_JSON5_DESERIALIZE.as_bytes());
        serialize_test("json5", EXAMPLE_JSON5_SERIALIZE.as_bytes());
    }

    #[test]
    #[cfg(feature = "serde-json")]
    fn test_json() {
        deserialize_test("json", EXAMPLE_JSON_DESERIALIZE.as_bytes());
        serialize_test("json", EXAMPLE_JSON_SERIALIZE.as_bytes());
    }

    #[test]
    #[cfg(feature = "accept-limited-xml-serialize")]
    fn test_xml() {
        serialize_test("xml", XML_SERIALIZE.as_bytes());
        deserialize_test("xml", XML_DESERIALIZE.as_bytes());
    }

    #[test]
    #[cfg(feature = "serde-cbor")]
    fn test_cbor() {
        serialize_test("cbor", CBOR_SERIALIZE);
        deserialize_test("cbor", CBOR_SERIALIZE);
    }

    #[test]
    #[cfg(feature = "serde-bson")]
    fn test_bson() {
        serialize_test("bson", BSON_SERIALIZE);
        deserialize_test("bson", BSON_SERIALIZE);
    }

    #[test]
    #[cfg(feature = "serde-ron")]
    fn test_ron() {
        serialize_test("ron", RON_SERIALIZE.as_bytes());
        deserialize_test("ron", RON_DESERIALIZE.as_bytes());
    }

    #[test]
    #[cfg(feature = "serde-toml")]
    fn test_toml() {
        serialize_test("toml", TOML_SERIALIZE.as_bytes());
        deserialize_test("toml", TOML_SERIALIZE.as_bytes());
    }

    #[test]
    #[cfg(feature = "serde-flexbuffers")]
    fn test_flex_buffers() {
        serialize_test("flexbuffers", FLEXBUFFERS_SERIALIZE);
        deserialize_test("flexbuffers", FLEXBUFFERS_SERIALIZE);
    }

    #[test]
    #[cfg(feature = "serde-lexpr")]
    fn test_lexpr() {
        serialize_test("lexpr", LEXPR_SERIALIZE.as_bytes());
        deserialize_test("lexpr", LEXPR_DESERIALIZE.as_bytes());
    }

    #[test]
    #[cfg(feature = "serde-messagepack")]
    fn test_messagepack() {
        serialize_test("messagepack", MESSAGEPACK_SERIALIZE);
        deserialize_test("messagepack", MESSAGEPACK_SERIALIZE);
    }

    #[test]
    #[cfg(feature = "serde-pickle")]
    fn test_pickle() {
        serialize_test("pickle", PICKLE_SERIALIZE);
        deserialize_test("pickle", PICKLE_SERIALIZE);
    }

    #[test]
    #[cfg(feature = "serde-postcard")]
    fn test_postcard() {
        serialize_test("postcard", POSTCARD_SERIALIZE);
        deserialize_test("postcard", POSTCARD_SERIALIZE);
    }

    #[test]
    #[cfg(feature = "serde-query-string")]
    fn test_url() {
        serialize_test("querystring", URL_SERIALIZE.as_bytes());
        deserialize_test("querystring", URL_SERIALIZE.as_bytes());
    }

    #[test]
    #[cfg(feature = "serde-bson")]
    fn test_error_from_bson_error() {
        let err = Error::from(bson::ser::Error::UnsignedIntegerExceededRange(0));
        assert!(matches!(err, Error::BsonSerializationFailure(_)));
        let err = Error::from(bson::de::Error::EndOfStream);
        assert!(matches!(err, Error::BsonDeserializationFailure(_)));
    }

    #[test]
    fn test_documentation_example() {
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
        assert_eq!(
            r#"bar: foobar
"#,
            encoded.try_to_string().unwrap()
        )
    }
}
