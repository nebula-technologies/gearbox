//! Payload Signature Config/Generator
//! This object is for creating a API key signature.
//!
//! This this example a static nonce is used to generate a API signature. This is to confirm the signature is as expected.
//! The example is also using the default signature configuration.
//! ```rust
//! extern crate alloc;
//!
//! use alloc::sync::Arc;
//! use gearbox::net::signature::Signature;
//! use base64;
//!
//! let mut signing = Signature::default();
//! let nonce = 1616492376594usize;
//!
//! let validated_sign = base64::decode("4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==").unwrap();
//!
//! let cal_sign = signing
//!   .var("payload", "ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25")
//!   .var("secret_key", "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==")
//!   .var("url", "/0/private/AddOrder")
//!   .nonce(Arc::new(move || -> Vec<u8> {nonce.to_string().as_bytes().to_vec()}))
//!   .sign();
//!
//! assert_eq!(validated_sign, cal_sign)
//! ```
//!
//! At the time of signing is might be usefull to locking the nonce. By locking the nonce you will prevent
//! change in the next signing.
//! This is usefull in the default signing configuration, and if the nonce is not predictable.
//!
//! In this example the signature will only generate a base64 encoded value.
//!
//! ```rust
//! extern crate alloc;
//!
//! use alloc::sync::Arc;
//! use gearbox::net::signature::*;
//! use base64;
//!
//! let mut signing = Signature::default();
//!
//! let cal_sign = signing
//!     .config(SignCal::Base64Encode(SignCal::VarString("nonce".to_string()).into())).nonce_default();
//! let nonce = cal_sign.nonce_lock();
//!
//! let b64_nonce = base64::encode(nonce.unwrap()).into_bytes();
//!
//!
//! assert_eq!(b64_nonce, cal_sign.sign());
//! ```
//! > Note:
//! > Using nonce_lock will lock the nonce until the next signing, as soon as a signing has happened the lock will be removed!
//! > Also running the lock multiple times will force the signature generator to create new nonce values.

pub mod helpers;

use crate::time::DateTime;
use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};
use core::fmt;
use core::ops::Deref;
use hashbrown::HashMap;
use serde_derive::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Signature {
    pub(crate) config: Option<SignCal>,
    #[serde(skip)]
    pub(crate) nonce: Option<Arc<dyn Fn() -> Vec<u8>>>,
    pub(crate) variables: HashMap<String, Variable>,
    pub(crate) nonce_lock: Option<Vec<u8>>,
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Signature")
            .field("config", &self.config)
            .field("nonce", &"<Fn() -> Vec<u8>>")
            .field("variables", &self.variables)
            .field("nonce_lock", &self.nonce_lock)
            .finish()
    }
}

unsafe impl Send for Signature {}
unsafe impl Sync for Signature {}

impl Signature {
    pub fn new() -> Self {
        Signature {
            config: None,
            nonce: None,
            variables: Default::default(),
            nonce_lock: None,
        }
    }

    pub fn nonce(&mut self, o: Arc<dyn Fn() -> Vec<u8>>) -> &mut Self {
        self.nonce = Some(o);
        self
    }

    #[cfg(feature = "std")]
    pub fn nonce_default(&mut self) -> &mut Self {
        self.nonce = Some(Arc::new(move || -> Vec<u8> {
            DateTime::now().to_unix().to_string().as_bytes().to_vec()
        }));
        self
    }
    #[cfg(not(feature = "std"))]
    pub fn nonce_default(&mut self) -> &mut Self {
        self.nonce = Some(Arc::new(move || -> Vec<u8> {
            DateTime::now().to_unix().to_string().as_bytes().to_vec()
        }));
        self
    }

    pub fn nonce_lock(&mut self) -> Option<Vec<u8>> {
        let nonce_fn = self.nonce.clone();
        let nonce = nonce_fn.map(|t| t());
        self.nonce_lock = nonce.clone();
        nonce
    }

    pub fn nonce_unlock(&mut self) -> &mut Self {
        self.nonce_lock = None;
        self
    }

    pub fn config(&mut self, config: SignCal) -> &mut Self {
        self.config = Some(config);
        self
    }

    pub fn var<T: Into<Variable>>(&mut self, key: &str, value: T) -> &mut Self {
        self.variables.insert(key.to_string(), value.into());
        self
    }

    pub fn compare<T: Into<Vec<u8>>>(&mut self, signature: T, nonce: Vec<u8>) -> bool {
        let mut _self = self.clone();
        _self.nonce = Some(Arc::new(move || -> Vec<u8> { nonce.clone() }));
        signature.into() == self.sign()
    }

    pub fn sign(&mut self) -> Vec<u8> {
        let nonce_fn = &self.nonce;
        let mut variables = self.variables.clone();
        variables.insert(
            "nonce".to_string(),
            Variable::Data(if let Some(nonce) = self.nonce_lock.clone() {
                self.nonce_lock = None;
                nonce.clone()
            } else {
                if let Some(nonce_fn) = nonce_fn {
                    nonce_fn()
                } else {
                    Vec::new()
                }
            }),
        );
        sign_calc(
            self.config.as_ref().unwrap_or(&SignCal::default()),
            &variables,
        )
    }
}

impl Default for Signature {
    fn default() -> Self {
        Signature::new()
    }
}

fn sign_calc(config: &SignCal, variables: &HashMap<String, Variable>) -> Vec<u8> {
    match config {
        SignCal::HmacSha512(k, c) => {
            helpers::hmac_sha512(&sign_calc(k, variables), &sign_calc(c.deref(), variables))
        }
        SignCal::HmacSha256(k, c) => {
            helpers::hmac_sha256(&sign_calc(k, variables), &sign_calc(c.deref(), variables))
        }
        SignCal::Sha256(c) => helpers::sha256(&sign_calc(c, variables)),
        SignCal::Base64Encode(c) => helpers::base64encode(&sign_calc(c, variables)),
        SignCal::Base64Decode(c) => helpers::base64decode(&sign_calc(c, variables)),
        SignCal::Base58Encode(c) => helpers::base58encode(&sign_calc(c, variables)),
        SignCal::Base58Decode(c) => helpers::base58decode(&sign_calc(c, variables)),
        SignCal::Sha512(c) => helpers::sha512(&sign_calc(c, variables)),
        SignCal::Append(a) => a
            .iter()
            .flat_map(|t| sign_calc(t, variables))
            .collect::<Vec<u8>>(),
        SignCal::JoinAsString(a) => a
            .iter()
            .flat_map(|t| String::from_utf8(sign_calc(t, variables)))
            .collect::<Vec<String>>()
            .join("")
            .into_bytes(),
        SignCal::VarData(k) => variables
            .get(k)
            .unwrap_or(&Variable::Data(Vec::new()))
            .into(),
        SignCal::VarString(k) => variables
            .get(k)
            .unwrap_or(&Variable::Data(Vec::new()))
            .into(),
        SignCal::VarInteger(k) => variables
            .get(k)
            .unwrap_or(&Variable::Data(Vec::new()))
            .into(),
        SignCal::Raw(v) => v.clone(),
        SignCal::String(s) => s.clone().into_bytes(),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SignCal {
    HmacSha256(Box<SignCal>, Box<SignCal>),
    HmacSha512(Box<SignCal>, Box<SignCal>),
    Sha256(Box<SignCal>),
    Sha512(Box<SignCal>),
    Base64Encode(Box<SignCal>),
    Base64Decode(Box<SignCal>),
    Base58Encode(Box<SignCal>),
    Base58Decode(Box<SignCal>),
    Append(Vec<SignCal>),
    JoinAsString(Vec<SignCal>),
    VarData(String),
    VarString(String),
    VarInteger(String),
    Raw(Vec<u8>),
    String(String),
}

impl Default for SignCal {
    fn default() -> Self {
        use SignCal::*;
        HmacSha512(
            Base64Decode(VarString("secret_key".to_string()).into()).into(),
            Append(vec![
                VarString("url".to_string()),
                Sha256(
                    Append(vec![
                        VarInteger("nonce".to_string()),
                        JoinAsString(vec![
                            Raw("nonce=".to_string().into_bytes()),
                            VarInteger("nonce".to_string()),
                            Raw("&".to_string().into_bytes()),
                            VarString("payload".to_string()),
                        ]),
                    ])
                    .into(),
                ),
            ])
            .into(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Variable {
    Data(Vec<u8>),
    String(String),
    Integer(usize),
}

impl From<Variable> for Vec<u8> {
    fn from(v: Variable) -> Self {
        match v {
            Variable::Integer(i) => i.to_string().into_bytes(),
            Variable::Data(d) => d,
            Variable::String(s) => s.into_bytes(),
        }
    }
}

impl From<&Variable> for Vec<u8> {
    fn from(v: &Variable) -> Self {
        match v {
            Variable::Integer(i) => i.to_string().into_bytes(),
            Variable::Data(d) => d.clone(),
            Variable::String(s) => s.clone().into_bytes(),
        }
    }
}

impl From<String> for Variable {
    fn from(s: String) -> Self {
        Variable::String(s)
    }
}
impl From<&str> for Variable {
    fn from(s: &str) -> Self {
        Variable::String(s.to_string())
    }
}
impl From<usize> for Variable {
    fn from(i: usize) -> Self {
        Variable::Integer(i)
    }
}
impl From<u8> for Variable {
    fn from(i: u8) -> Self {
        Variable::Integer(i as usize)
    }
}
impl From<u32> for Variable {
    fn from(i: u32) -> Self {
        Variable::Integer(i as usize)
    }
}
impl From<u64> for Variable {
    fn from(i: u64) -> Self {
        Variable::Integer(i as usize)
    }
}
impl From<u128> for Variable {
    fn from(i: u128) -> Self {
        Variable::Integer(i as usize)
    }
}
impl From<i8> for Variable {
    fn from(i: i8) -> Self {
        Variable::Integer(i as usize)
    }
}
impl From<i32> for Variable {
    fn from(i: i32) -> Self {
        Variable::Integer(i as usize)
    }
}
impl From<i64> for Variable {
    fn from(i: i64) -> Self {
        Variable::Integer(i as usize)
    }
}
impl From<i128> for Variable {
    fn from(i: i128) -> Self {
        Variable::Integer(i as usize)
    }
}
impl From<Vec<u8>> for Variable {
    fn from(v: Vec<u8>) -> Self {
        Variable::Data(v)
    }
}
impl From<&[u8]> for Variable {
    fn from(v: &[u8]) -> Self {
        Variable::Data(v.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::{SignCal, Signature};
    use alloc::sync::Arc;
    use hex;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_base58() {
        use SignCal::*;
        assert_eq!(
            b"3mJr7AoUCHxNqd".to_vec(),
            Signature::default()
                .config(Base58Encode(Raw(b"1234567890".to_vec()).into()))
                .sign()
        );
    }

    #[test]
    fn test_base58_decode() {
        use SignCal::*;
        assert_eq!(
            b"1234567890".to_vec(),
            Signature::default()
                .config(Base58Decode(Raw(b"3mJr7AoUCHxNqd".to_vec()).into()))
                .sign()
        );
    }

    #[test]
    fn test_base64() {
        use SignCal::*;
        assert_eq!(
            b"MTIzNDU2Nzg5MA==".to_vec(),
            Signature::default()
                .config(Base64Encode(Raw(b"1234567890".to_vec()).into()))
                .sign()
        );
    }

    #[test]
    fn test_base64_decode() {
        use SignCal::*;
        assert_eq!(
            b"1234567890".to_vec(),
            Signature::default()
                .config(Base64Decode(Raw(b"MTIzNDU2Nzg5MA==".to_vec()).into()))
                .sign()
        );
    }

    #[test]
    fn test_sha256() {
        use SignCal::*;

        let signature = Signature::default()
            .config(Sha256(
                Raw("1234567890".to_string().as_bytes().to_vec()).into(),
            ))
            .sign();

        assert_eq!(
            hex::decode("c775e7b757ede630cd0aa1113bd102661ab38829ca52a6422ab782862f268646")
                .unwrap(),
            signature
        );
    }

    #[test]
    fn test_sha512() {
        use SignCal::*;
        assert_eq!(
            hex::decode("12b03226a6d8be9c6e8cd5e55dc6c7920caaa39df14aab92d5e3ea9340d1c8a4d3d0b8e4314f1f6ef131ba4bf1ceb9186ab87c801af0d5c95b1befb8cedae2b9").unwrap(),
            Signature::default()
                .config(Sha512(Raw(b"1234567890".to_vec()).into()))
                .sign()
        );
    }

    #[test]
    fn test_sign_cal_with_control_signature() {
        use SignCal::*;

        let nonce = 1616492376594usize;
        let mut signature = Signature::default();
        signature.var("payload", format!("ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25"))
            .var("secret_key", "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==")
            .var("url", "/0/private/AddOrder")
            .nonce(Arc::new(move || -> Vec<u8> {nonce.to_string().as_bytes().to_vec()}))
            .config(Base64Encode(
                HmacSha512(
                    Base64Decode(VarString("secret_key".to_string()).into()).into(),
                    Append(vec![
                        VarString("url".to_string()),
                        Sha256(
                            Append(vec![
                                VarInteger("nonce".to_string()),
                                JoinAsString(vec![
                                    Raw("nonce=".to_string().into_bytes()),
                                    VarInteger("nonce".to_string()),
                                    Raw("&".to_string().into_bytes()),
                                    VarString("payload".to_string()),
                                ])
                            ])
                                .into(),
                        ),
                    ])
                        .into(),
                )
                    .into(),
            ));

        let api_sign = b"4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==".to_vec();

        assert_eq!(api_sign, signature.sign());
    }

    #[test]
    fn test_sign_cal_compare_with_control_signature() {
        let nonce = 1616492376594usize;
        let mut signature = Signature::default();
        signature.var("payload", format!("ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25"))
            .var("secret_key", "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==")
            .var("url", "/0/private/AddOrder")
            .nonce(Arc::new(move || -> Vec<u8> {nonce.to_string().as_bytes().to_vec()}));

        let api_sign = base64::decode("4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==").unwrap().to_vec();

        assert!(signature.compare(api_sign, nonce.to_string().as_bytes().to_vec()));
    }

    #[test]
    fn test_nonce_lock() {
        let mut signing = Signature::default();
        let cal_sign = signing
            .config(SignCal::Base64Encode(
                SignCal::VarString("nonce".to_string()).into(),
            ))
            .nonce_default();
        let nonce = cal_sign.nonce_lock().unwrap();
        let b64_nonce = base64::encode(nonce).into_bytes();

        assert_eq!(b64_nonce, cal_sign.sign());
    }

    #[test]
    fn test_confirm_unlock() {
        let mut signing = Signature::default();
        let cal_sign = signing
            .config(SignCal::Base64Encode(
                SignCal::VarString("nonce".to_string()).into(),
            ))
            .nonce_default();
        let _nonce = cal_sign.nonce_lock();
        assert!(cal_sign.nonce_lock.is_some());
        cal_sign.nonce_unlock();
        assert!(cal_sign.nonce_lock.is_none());
    }
}
