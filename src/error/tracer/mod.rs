// Mods
pub mod error_macro;
pub mod extended_info;
// Local uses
#[cfg(feature = "with_serde")]
use crate::externs::serde::{
    de::{self, MapAccess, Visitor},
    derive,
    ser::{self, Error, SerializeStruct},
    Deserialize, Deserializer, Serialize, Serializer,
};
#[cfg(target_arch = "wasm32")]
use crate::serde::wasm_bindgen::{from_value, to_value};
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, boxed::Box, format, string::ToString};
use core::any::type_name;
use core::any::Any;
use core::fmt;
use core::fmt::Display;
use core::fmt::{Debug, Formatter};
use core::hash::Hash;
use erased_serde::serialize_trait_object;
use std::marker::PhantomData;
// Exported Uses
pub use extended_info::ErrorTracerExtInfo;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{
    convert::{FromWasmAbi, IntoWasmAbi},
    describe::{inform, WasmDescribe},
    prelude::*,
    JsValue,
};

pub trait ErrorDebug: fmt::Debug + Any {
    fn to_error_parts(&self) -> (String, Option<String>) {
        (format!("{:?}", self), None)
    }
}

impl<T: fmt::Debug + Any> ErrorDebug for T {}

#[cfg(feature = "with_serde")]
// Define a marker trait for types that implement Serialize
trait Serializable: erased_serde::Serialize {}

#[cfg(feature = "with_serde")]
impl<T> Serializable for T where T: Serialize {}

#[cfg(feature = "with_serde")]
serialize_trait_object!(Serializable);

pub trait AnyBoxError: Any {
    fn as_any(&self) -> Box<&dyn Any>;
}
pub trait AnyMutBoxError: Any {
    fn as_any_mut(&mut self) -> Box<&mut dyn Any>;
}

// #[cfg(target_arch = "wasm32")]
impl<T: ErrorDebug> AnyBoxError for T {
    fn as_any(&self) -> Box<&dyn Any> {
        Box::new(self)
    }
}
// #[cfg(target_arch = "wasm32")]
impl<T: ErrorDebug> AnyMutBoxError for T {
    fn as_any_mut(&mut self) -> Box<&mut dyn Any> {
        Box::new(self)
    }
}
//
// #[cfg(not(target_arch = "wasm32"))]
// impl AnyBoxError for Box<dyn ErrorDebug> {
//     fn as_any(&self) -> &Box<dyn Any> {
//         self
//     }
// }
//
// #[cfg(not(target_arch = "wasm32"))]
// impl AnyMutBoxError for Box<dyn ErrorDebug> {
//     fn as_any_mut(&mut self) -> &mut Box<dyn Any> {
//         self
//     }
// }

pub struct TracerError<T>
where
    T: ErrorDebug,
{
    error: Box<T>,
    type_name: Option<String>,
    info: ErrorTracerExtInfo,
    cause: Option<Vec<DynTracerError>>,
}

impl<T> TracerError<T>
where
    T: 'static + ErrorDebug,
{
    pub fn const_new(
        error: Box<T>,
        info: ErrorTracerExtInfo,
        cause: Option<Vec<DynTracerError>>,
    ) -> Self {
        Self {
            error: error,
            type_name: None,
            info,
            cause,
        }
    }
    pub fn new(
        error: Box<T>,
        info: ErrorTracerExtInfo,
        cause: Option<Vec<DynTracerError>>,
    ) -> Self {
        #[cfg(feature = "type-registry")]
        {
            use crate::error::type_registry::register_type;
            register_type::<T>();
        }

        Self {
            error,
            type_name: Some(type_name::<T>().to_string()),
            info,
            cause,
        }
    }
    pub fn kind(&self) -> &Box<T> {
        &self.error
    }

    pub fn cause(&self) -> Option<&Vec<DynTracerError>> {
        self.cause.as_ref()
    }

    pub fn cause_mut(&mut self) -> Option<&mut Vec<DynTracerError>> {
        self.cause.as_mut()
    }

    pub fn has_cause(&self) -> bool {
        self.cause.as_ref().map(|t| !t.is_empty()).unwrap_or(false)
    }

    pub fn err_to_string(&self) -> String {
        format!("{:?}", self.error)
    }

    pub fn digest(&self) -> ErrorDigest {
        let (message, detailed_msg) = self.to_error_parts();
        let stack = self
            .cause
            .as_ref()
            .map(|t| t.iter().map(|t| t.digest()).collect::<Vec<ErrorDigest>>());
        ErrorDigest {
            message,
            detailed_msg,
            line: self.info.line().map(|t| t.to_owned()),
            file: self.info.file().cloned(),
            subsystem: self.info.subsystem().cloned(),
            code: self.info.code().map(|t| t.to_owned()),
            stack,
        }
    }
}

impl<T> From<T> for TracerError<T>
where
    T: 'static + ErrorDebug,
{
    fn from(err: T) -> Self {
        Self::new(Box::new(err), ErrorTracerExtInfo::default(), None)
    }
}

#[cfg(feature = "with_serde")]
// Custom Serialize for TracerError
impl<T> Serialize for TracerError<T>
where
    T: ErrorDebug,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TracerError", 4)?;
        // TODO! Serialize the error field properly
        state.serialize_field("error", &format!("{:?}", *self.error))?;
        state.serialize_field("type_id", &self.type_name)?;
        state.serialize_field("info", &self.info)?;
        state.serialize_field("cause", &self.cause)?;
        state.end()
    }
}

#[cfg(feature = "with_serde")]
// Custom Deserialize for TracerError
impl<'de, T> Deserialize<'de> for TracerError<T>
where
    T: ErrorDebug + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(derive::Deserialize)]
        struct TracerErrorData<T>
        where
            T: ErrorDebug,
        {
            error: Box<T>,
            type_id: Option<String>,
            info: ErrorTracerExtInfo,
            cause: Option<Vec<Box<DynTracerError>>>,
        }

        struct TracerErrorVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for TracerErrorVisitor<T>
        where
            T: ErrorDebug + Deserialize<'de>,
        {
            type Value = TracerError<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct TracerError")
            }

            fn visit_map<V>(self, mut map: V) -> Result<TracerError<T>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut error = None;
                let mut type_id = None;
                let mut info = None;
                let mut cause = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "error" => {
                            if error.is_some() {
                                return Err(de::Error::duplicate_field("error"));
                            }
                            error = Some(map.next_value()?);
                        }
                        "type_id" => {
                            if type_id.is_some() {
                                return Err(de::Error::duplicate_field("type_id"));
                            }
                            type_id = Some(map.next_value()?);
                        }
                        "info" => {
                            if info.is_some() {
                                return Err(de::Error::duplicate_field("info"));
                            }
                            info = Some(map.next_value()?);
                        }
                        "cause" => {
                            if cause.is_some() {
                                return Err(de::Error::duplicate_field("cause"));
                            }
                            cause = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                let error = error.ok_or_else(|| de::Error::missing_field("error"))?;
                let info = info.ok_or_else(|| de::Error::missing_field("info"))?;
                Ok(TracerError {
                    error,
                    type_name: type_id,
                    info,
                    cause,
                })
            }
        }

        const FIELDS: &[&str] = &["error", "type_id", "info", "cause"];
        deserializer.deserialize_struct("TracerError", FIELDS, TracerErrorVisitor(PhantomData))
    }
}

unsafe impl<T> Send for TracerError<T> where T: ErrorDebug {}
unsafe impl<T> Sync for TracerError<T> where T: ErrorDebug {}

#[cfg(target_arch = "wasm32")]
impl<T> WasmDescribe for TracerError<T>
where
    T: ErrorDebug + WasmDescribe,
{
    fn describe() {
        use wasm_bindgen::describe::inform;
        inform(6); // 6 = Inform the type is an object

        inform(4); // 4 = Number of fields in the object
        inform("error".as_ptr() as u32); // Field 1 name
        T::describe(); // Field 1 type

        inform("type_id".as_ptr() as u32); // Field 2 name
        inform(4); // Field 2 type is a u32

        inform("info".as_ptr() as u32); // Field 3 name
        inform(1); // Field 3 type is a custom type, need to describe ErrorTracerExtInfo separately

        inform("cause".as_ptr() as u32); // Field 4 name
        inform(4); // Field 4 type is an array of custom type, need to describe Vec<Box<DynTracerError>> separately
    }
}
//
// #[cfg(target_arch = "wasm32")]
// impl<'de, T> FromWasmAbi for TracerError<T>
// where
//     T: ErrorDebug + FromWasmAbi + Deserialize<'de>,
// {
//     type Abi = JsValue;
//
//     unsafe fn from_abi(js_val: Self::Abi) -> Self {
//         from_value(js_val).unwrap()
//     }
// }
// #[cfg(target_arch = "wasm32")]
// impl<T> IntoWasmAbi for TracerError<T>
// where
//     T: ErrorDebug + IntoWasmAbi + Serializable,
// {
//     type Abi = JsValue;
//
//     fn into_abi(self) -> Self::Abi {
//         to_value(&self).unwrap()
//     }
// }

impl<T> Display for TracerError<T>
where
    T: ErrorDebug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let file = self.info.file().cloned().unwrap_or("<Unknown>".to_string());
        let line = self
            .info
            .line()
            .map(|t| t.to_string())
            .unwrap_or("-1".to_string());
        let error = format!("{:?}", self.error);
        let code = self.info.code().unwrap_or(&0);
        let module = self.info.subsystem().cloned().unwrap_or("<>".to_string());
        write!(f, "{}::{}:{} {}:Error: {}", file, module, line, code, error)
    }
}

impl<T> Debug for TracerError<T>
where
    T: ErrorDebug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let file = self.info.file().cloned().unwrap_or("<Unknown>".to_string());
        let line = self
            .info
            .line()
            .map(|t| t.to_string())
            .unwrap_or("-1".to_string());
        let error = format!("{:?}", self.error);
        let code = self.info.code().unwrap_or(&0);
        let module = self.info.subsystem().cloned().unwrap_or("<>".to_string());
        write!(f, "{}::{}:{} {}:Error: {}", file, module, line, code, error)
    }
}

/// DynTracerError:
///

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct DynTracerError {
    error: Box<dyn ErrorDebug>,
    type_name: Option<String>,
    info: ErrorTracerExtInfo,
    cause: Option<Vec<DynTracerError>>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl DynTracerError {
    pub fn err_to_string(&self) -> String {
        format!("{:?}", self.error)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl DynTracerError {
    pub fn new_str_err(
        error: String,
        info: ErrorTracerExtInfo,
        cause: Option<Vec<DynTracerError>>,
    ) -> Self {
        Self {
            error: Box::new(error),
            type_name: None,
            info,
            cause,
        }
    }

    pub fn kind_as_str(&self) -> String {
        format!("{:?}", self.error)
    }

    pub fn cause_vec_str(&self) -> Option<Vec<String>> {
        self.cause
            .as_ref()
            .map(|t| t.iter().map(|t| t.err_to_string()).collect())
    }

    pub fn has_cause(&self) -> bool {
        self.cause.as_ref().map(|t| !t.is_empty()).unwrap_or(false)
    }
}

impl DynTracerError {
    pub fn new<T: 'static + ErrorDebug>(
        error: Box<T>,
        info: ErrorTracerExtInfo,
        cause: Option<Vec<DynTracerError>>,
    ) -> Self {
        #[cfg(feature = "type-registry")]
        {
            use crate::error::type_registry::register_type;
            register_type::<T>();
        }

        Self {
            error,
            type_name: Some(type_name::<T>().to_string()),
            info,
            cause,
        }
    }
    pub fn kind(&self) -> &Box<dyn ErrorDebug> {
        &self.error
    }

    pub fn cause(&self) -> Option<&Vec<DynTracerError>> {
        self.cause.as_ref()
    }

    pub fn cause_mut(&mut self) -> Option<&mut Vec<DynTracerError>> {
        self.cause.as_mut()
    }

    pub fn downcast_ref<T: ErrorDebug + AnyBoxError>(&self) -> Option<&T> {
        let error = self.error.as_any();
        error.downcast_ref::<T>()
    }

    pub fn digest(&self) -> ErrorDigest {
        let (message, detailed_msg) = self.to_error_parts();
        let stack = self
            .cause
            .as_ref()
            .map(|t| t.iter().map(|t| t.digest()).collect::<Vec<ErrorDigest>>());
        ErrorDigest {
            message,
            detailed_msg,
            line: self.info.line().map(|t| t.to_owned()),
            file: self.info.file().cloned(),
            subsystem: self.info.subsystem().cloned(),
            code: self.info.code().map(|t| t.to_owned()),
            stack,
        }
    }
}

// impl<T> From<T> for DynTracerError
// where
//     T: 'static + ErrorDebug,
// {
//     fn from(err: T) -> Self {
//         Self::new(Box::new(err), ErrorTracerExtInfo::default(), None)
//     }
// }

#[cfg(feature = "with_serde")]
impl Serialize for DynTracerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DynTracerError", 4)?;

        // TODO! Serialize the error field properly
        state.serialize_field("error", &format!("{:?}", self.error))?;
        state.serialize_field("type_id", &self.type_name)?;
        state.serialize_field("info", &self.info)?;
        state.serialize_field("cause", &self.cause)?;
        state.end()
    }
}

// Custom Deserialize for DynTracerError
#[cfg(feature = "with_serde")]
impl<'de> Deserialize<'de> for DynTracerError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(derive::Deserialize)]
        struct DynTracerErrorData {
            error: String,
            type_name: Option<String>,
            info: ErrorTracerExtInfo,
            cause: Option<Vec<Box<DynTracerError>>>,
        }

        struct DynTracerErrorVisitor;

        impl<'de> Visitor<'de> for DynTracerErrorVisitor {
            type Value = DynTracerError;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct DynTracerError")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DynTracerError, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut error = None;
                let mut type_name = None;
                let mut info = None;
                let mut cause = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "error" => {
                            if error.is_some() {
                                return Err(de::Error::duplicate_field("error"));
                            }
                            error = Some(map.next_value()?);
                        }
                        "type_name" => {
                            if type_name.is_some() {
                                return Err(de::Error::duplicate_field("type_name"));
                            }
                            type_name = Some(map.next_value()?);
                        }
                        "info" => {
                            if info.is_some() {
                                return Err(de::Error::duplicate_field("info"));
                            }
                            info = Some(map.next_value()?);
                        }
                        "cause" => {
                            if cause.is_some() {
                                return Err(de::Error::duplicate_field("cause"));
                            }
                            cause = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                let error = error.ok_or_else(|| de::Error::missing_field("error"))?;
                let info = info.ok_or_else(|| de::Error::missing_field("info"))?;
                Ok(DynTracerError {
                    error: Box::new(error),
                    type_name,
                    info,
                    cause,
                })
            }
        }

        const FIELDS: &[&str] = &["error", "type_name", "info", "cause"];
        deserializer.deserialize_struct("DynTracerError", FIELDS, DynTracerErrorVisitor)
    }
}

unsafe impl Send for DynTracerError {}
unsafe impl Sync for DynTracerError {}

impl<T> From<(T, ErrorTracerExtInfo)> for DynTracerError
where
    T: 'static + ErrorDebug,
{
    fn from((err, info): (T, ErrorTracerExtInfo)) -> Self {
        #[cfg(feature = "type-registry")]
        {
            use crate::error::type_registry::register_type;
            register_type::<T>();
        }
        let type_name = Some(type_name::<T>().to_string());
        DynTracerError {
            error: Box::new(err),
            type_name,
            info,
            cause: None,
        }
    }
}

impl Display for DynTracerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let file = self.info.file().cloned().unwrap_or("<Unknown>".to_string());
        let line = self
            .info
            .line()
            .map(|t| t.to_string())
            .unwrap_or("-1".to_string());
        let error = format!("{:?}", self.error);
        let code = self.info.code().unwrap_or(&0);
        let module = self.info.subsystem().cloned().unwrap_or("<>".to_string());
        write!(f, "{}::{}:{} {}:Error: {}", file, module, line, code, error)
    }
}

impl Debug for DynTracerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let file = self.info.file().cloned().unwrap_or("<Unknown>".to_string());
        let line = self
            .info
            .line()
            .map(|t| t.to_string())
            .unwrap_or("-1".to_string());
        let error = format!("{:?}", self.error);
        let code = self.info.code().unwrap_or(&0);
        let module = self.info.subsystem().cloned().unwrap_or("<>".to_string());
        write!(f, "{}::{}:{} {}:Error: {}", file, module, line, code, error)
    }
}

impl<T> From<TracerError<T>> for DynTracerError
where
    T: ErrorDebug,
{
    fn from(err: TracerError<T>) -> Self {
        let info = err.info.clone();
        let cause = err.cause;

        DynTracerError::new(err.error, info, cause)
    }
}

pub struct ErrorDigest {
    pub message: String,
    pub detailed_msg: Option<String>,
    pub line: Option<u32>,
    pub file: Option<String>,
    pub subsystem: Option<String>,
    pub code: Option<u16>,
    pub stack: Option<Vec<ErrorDigest>>,
}

impl ErrorDigest {
    pub fn new(
        message: String,
        detailed_msg: Option<String>,
        line: Option<u32>,
        file: Option<String>,
        subsystem: Option<String>,
        code: Option<u16>,
        stack: Option<Vec<ErrorDigest>>,
    ) -> Self {
        Self {
            message,
            detailed_msg,
            line,
            file,
            subsystem,
            code,
            stack,
        }
    }

    pub fn to_stack_line(&self) -> String {
        format!(
            "{file}::{subsystem}:{line} {code}{message}",
            message = self.message,
            code = self
                .code
                .map(|t| format!("[code: {}] ", t.to_string()))
                .unwrap_or("".to_string()),
            file = self.file.as_deref().unwrap_or("<Unknown>"),
            line = self.line.map(|t| t.to_string()).unwrap_or("-1".to_string()),
            subsystem = self.subsystem.as_deref().unwrap_or("<Unknown>")
        )
    }

    pub fn to_simple(&self) -> String {
        format!("Error: {}", self.message)
    }

    pub fn to_detailed(&self) -> String {
        format!(
            "Error: {message}\nCode: {code}\nFile: {file}\nLine: {line}\nSubsystem: {subsystem}",
            message = self.message,
            code = self.code.unwrap_or(0),
            file = self.file.as_deref().unwrap_or("<Unknown>"),
            line = self.line.map(|t| t.to_string()).unwrap_or("-1".to_string()),
            subsystem = self.subsystem.as_deref().unwrap_or("<Unknown>")
        )
    }

    pub fn to_stack(&self) -> String {
        format!(
            "Error: {message}\nCode: {code}\nFile: {file}\nLine: {line}\nSubsystem: {subsystem}\nStack trace:\n{stack}",
            message = self.message,
            code = self.code.unwrap_or(0),
            file = self.file.as_deref().unwrap_or("<Unknown>"),
            line = self.line.map(|t| t.to_string()).unwrap_or("-1".to_string()),
            subsystem = self.subsystem.as_deref().unwrap_or("<Unknown>"),
            stack = self
                .stack
                .as_ref()
                .map(|t| t.iter().map(|t| t.to_stack_line()).collect::<Vec<String>>().join("\n"))
                .unwrap_or("".to_string())
        )
    }

    pub fn to_stack_detailed(&self) -> String {
        format!(
            "Error: {message}\nCode: {code}\nFile: {file}\nLine: {line}\nSubsystem: {subsystem}\nStack trace:\n{stack}",
            message = self.message,
            code = self.code.unwrap_or(0),
            file = self.file.as_deref().unwrap_or("<Unknown>"),
            line = self.line.map(|t| t.to_string()).unwrap_or("-1".to_string()),
            subsystem = self.subsystem.as_deref().unwrap_or("<Unknown>"),
            stack = self
                .stack
                .as_ref()
                .map(|t| t.iter().map(|t| t.to_detailed()).collect::<Vec<_>>().join("\n"))
                .unwrap_or("".to_string())
        )
    }
}

impl Display for ErrorDigest {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}:{}:{} {}:Error: {}",
            self.file.as_deref().unwrap_or("<Unknown>"),
            self.subsystem.as_deref().unwrap_or("<Unknown>"),
            self.line.map(|t| t.to_string()).unwrap_or("-1".to_string()),
            self.code.unwrap_or(0),
            self.message
        )
    }
}

impl Debug for ErrorDigest {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_stack())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestError {
        message: &'static str,
    }

    impl TestError {
        fn new(message: &'static str) -> Self {
            Self { message }
        }
    }

    impl Display for TestError {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    //
    // #[test]
    // fn test_tracer_error_creation() {
    //     let error = TestError::new("Test Error");
    //     let info = ErrorTracerExtInfo::default();
    //     let tracer_error = TracerError::from((error, info));
    //
    //     assert_eq!(tracer_error.len(), 0);
    //     assert_eq!(tracer_error.is_empty(), true);
    // }
    //
    // #[test]
    // fn test_push_front() {
    //     let mut tracer_error = TracerError::new(
    //         TypeId::of::<TestError>(),
    //         Box::new(TestError::new("Test Error")),
    //         ErrorTracerExtInfo::default(),
    //         None,
    //     );
    //
    //     tracer_error.push_front(TestError::new("Another Test Error"));
    //     assert_eq!(tracer_error.len(), 1);
    //     assert_eq!(
    //         tracer_error.pop_front().unwrap().message,
    //         "Another Test Error"
    //     );
    // }

    // #[test]
    // fn test_push_back() {
    //     let mut tracer_error = TracerError::new(
    //         TypeId::of::<TestError>(),
    //         Box::new(TestError::new("Test Error")),
    //         ErrorTracerExtInfo::default(),
    //         None,
    //     );
    //
    //     tracer_error.push_back(TestError::new("Another Test Error"));
    //     assert_eq!(tracer_error.len(), 1);
    //     assert_eq!(tracer_error.pop_front().unwrap().message, "Test Error");
    //     assert_eq!(
    //         tracer_error.pop_front().unwrap().message,
    //         "Another Test Error"
    //     );
    // }

    // #[test]
    // fn test_pop_front() {
    //     let mut tracer_error = TracerError::new(
    //         TypeId::of::<TestError>(),
    //         Box::new(TestError::new("Test Error")),
    //         ErrorTracerExtInfo::default(),
    //         None,
    //     );
    //
    //     tracer_error.push_front(TestError::new("Another Test Error"));
    //     assert_eq!(
    //         tracer_error.pop_front().unwrap().message,
    //         "Another Test Error"
    //     );
    //     assert_eq!(tracer_error.pop_front().unwrap().message, "Test Error");
    //     assert_eq!(tracer_error.pop_front(), None);
    // }

    // #[test]
    // fn test_is_empty() {
    //     let mut tracer_error = TracerError::new(
    //         TypeId::of::<TestError>(),
    //         Box::new(TestError::new("Test Error")),
    //         ErrorTracerExtInfo::default(),
    //         None,
    //     );
    //
    //     assert!(tracer_error.is_empty());
    //     tracer_error.push_front(TestError::new("Another Test Error"));
    //     assert!(!tracer_error.is_empty());
    // }

    // #[test]
    // fn test_len() {
    //     let mut tracer_error = TracerError::new(
    //         TypeId::of::<TestError>(),
    //         Box::new(TestError::new("Test Error")),
    //         ErrorTracerExtInfo::default(),
    //         None,
    //     );
    //
    //     assert_eq!(tracer_error.len(), 0);
    //     tracer_error.push_front(TestError::new("Another Test Error"));
    //     assert_eq!(tracer_error.len(), 1);
    //     tracer_error.pop_front();
    //     assert_eq!(tracer_error.len(), 0);
    // }

    #[test]
    fn test_display() {
        let error = TestError::new("Test Error");
        let info = ErrorTracerExtInfo::default();
        let tracer_error = TracerError::from((error, info));

        let display = format!("{}", tracer_error);
        assert!(display.contains("Test Error"));
    }

    #[test]
    fn test_debug() {
        let error = TestError::new("Test Error");
        let info = ErrorTracerExtInfo::default();
        let tracer_error = TracerError::from((error, info));

        let debug = format!("{:?}", tracer_error);
        assert!(debug.contains("Test Error"));
    }

    // #[test]
    // fn test_from_tuple() {
    //     let error = TestError::new("Test Error");
    //     let info = ErrorTracerExtInfo::default();
    //     let tracer_error: TracerError = (error, info).into();
    //
    //     assert!(tracer_error.kind().message == "Test Error");
    // }

    #[test]
    fn test_has_cause() {
        let cause_error =
            TracerError::from((TestError::new("Cause Error"), ErrorTracerExtInfo::default()));
        let tracer_error = TracerError::new(
            Box::new(TestError::new("Test Error")),
            ErrorTracerExtInfo::default(),
            Some(vec![cause_error.into()]),
        );

        assert!(tracer_error.has_cause());
    }

    // #[test]
    // fn test_cause() {
    //     let cause_error =
    //         TracerError::from((TestError::new("Cause Error"), ErrorTracerExtInfo::default()));
    //     let mut tracer_error = TracerError::new(
    //         TypeId::of::<TestError>(),
    //         Box::new(TestError::new("Test Error")),
    //         ErrorTracerExtInfo::default(),
    //         Some(vec![Box::new(cause_error)]),
    //     );
    //
    //     assert_eq!(tracer_error.cause().unwrap().len(), 1);
    //     assert!(tracer_error.cause().unwrap()[0].kind().message == "Cause Error");
    // }
}
