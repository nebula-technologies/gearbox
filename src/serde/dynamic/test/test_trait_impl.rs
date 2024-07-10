use super::super::Error;

#[cfg(not(tarpaulin_include))]
impl Eq for Error {}

#[cfg(not(tarpaulin_include))]
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Error::Infallible => match other {
                Error::Infallible => true,
                _ => false,
            },
            Error::ByteToUTF8ConversionFailure(e) => match other {
                Error::ByteToUTF8ConversionFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::UnknownContentTypeMatchFromStr(e) => match other {
                Error::UnknownContentTypeMatchFromStr(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-bson")]
            Error::BsonSerializationFailure(e) => match other {
                Error::BsonSerializationFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-bson")]
            Error::BsonDeserializationFailure(e) => match other {
                Error::BsonDeserializationFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-cbor")]
            Error::CborFailure(e) => match other {
                Error::CborFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-flexbuffers")]
            Error::FlexBuffersSerializationFailure(e) => match other {
                Error::FlexBuffersSerializationFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-flexbuffers")]
            Error::FlexBuffersDeserializationFailure(e) => match other {
                Error::FlexBuffersDeserializationFailure(ee) => {
                    format!("{}", e) == format!("{}", ee)
                }
                _ => false,
            },
            #[cfg(feature = "serde-json")]
            Error::JsonError(e) => match other {
                Error::JsonError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-json5")]
            Error::Json5Error(e) => match other {
                Error::Json5Error(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-lexpr")]
            Error::LexprError(e) => match other {
                Error::LexprError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-messagepack")]
            Error::MessagePackEncodeError(e) => match other {
                Error::MessagePackEncodeError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-messagepack")]
            Error::MessagePackDecodeError(e) => match other {
                Error::MessagePackDecodeError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-pickle")]
            Error::PickleError(e) => match other {
                Error::PickleError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-postcard")]
            Error::PostcardError(e) => match other {
                Error::PostcardError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-ron")]
            Error::RonError(e) => match other {
                Error::RonError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-toml")]
            Error::TomlSerializationFailure(e) => match other {
                Error::TomlSerializationFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-toml")]
            Error::TomlDeserializationFailure(e) => match other {
                Error::TomlDeserializationFailure(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-query-string")]
            Error::QueryStringEncodingFailure(e) => match other {
                Error::QueryStringEncodingFailure(ee) => format!("{:?}", e) == format!("{:?}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-yaml")]
            Error::YamlError(e) => match other {
                Error::YamlError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            #[cfg(feature = "serde-accept-limited-xml-serialize")]
            Error::XmlError(e) => match other {
                Error::XmlError(ee) => format!("{:?}", e) == format!("{:?}", ee),
                _ => false,
            },
            Error::TypeDoesNotSupportSerialization(e) => match other {
                Error::TypeDoesNotSupportSerialization(ee) => {
                    format!("{:?}", e) == format!("{:?}", ee)
                }
                _ => false,
            },
            #[cfg(feature = "serde-ron")]
            Error::RonDecodeError(e) => match other {
                Error::RonDecodeError(ee) => format!("{}", e) == format!("{}", ee),
                _ => false,
            },
            Error::NoSerializersDeserializersSet => match other {
                Error::NoSerializersDeserializersSet => true,
                _ => false,
            },
        }
    }
}
