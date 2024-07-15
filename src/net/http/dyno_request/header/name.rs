use alloc::string::{String, ToString};
use core::ops::Deref;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Name<T = String>(pub T);

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Name(s.to_string())
    }
}

impl From<String> for Name {
    fn from(s: String) -> Self {
        Name(s.to_string())
    }
}

impl From<Name> for reqwest::header::HeaderName {
    fn from(name: Name) -> Self {
        (&name).into()
    }
}

impl From<&Name> for reqwest::header::HeaderName {
    fn from(name: &Name) -> Self {
        reqwest::header::HeaderName::from_bytes(name.0.as_bytes()).unwrap()
    }
}

impl From<reqwest::header::HeaderName> for Name {
    fn from(name: reqwest::header::HeaderName) -> Self {
        Self::from(&name)
    }
}

impl From<&reqwest::header::HeaderName> for Name {
    fn from(name: &reqwest::header::HeaderName) -> Self {
        Name(name.to_string())
    }
}
