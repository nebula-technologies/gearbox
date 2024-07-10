use super::Builder;
use super::{Method, Url};
#[cfg(feature = "net-signature")]
use crate::net::signature::Signature;
use alloc::string::ToString;
use spin::RwLock;

#[cfg(feature = "net-signature")]
pub(crate) static GLOBAL_SIGNING: RwLock<Option<Signature>> = RwLock::new(None);

pub struct Client {
    #[allow(unused)]
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    #[cfg(feature = "net-signature")]
    pub fn set_global_signing(sign: Signature) {
        let mut global_signing = GLOBAL_SIGNING.write();
        *global_signing = Some(sign);
    }
}

macro_rules! client_methods {
    (
        $(
            $(#[$docs:meta])*
            ($func:ident, $upcase:ident);
        )+
    ) => {
        impl Client {
            $(
                $(#[$docs])*
                pub fn $func(self, url: &str) -> Builder {
                    Builder::new_with_client(Some(self), Method::$upcase, Url::Simple(url.to_string()))
                }
            )+
        }
    }
}

client_methods! {
    /// Fetches a representation of the specified resource.
    (get, Get);
    /// Submits an entity to the specified resource, often causing a change in state or side effects on the server.
    (post, Post);
    /// Replaces all current representations of the target resource with the request payload.
    (put, Put);
    /// Deletes the specified resource.
    (delete, Delete);
    /// Applies partial modifications to a resource.
    (patch, Patch);
    /// Asks for a response identical to a GET request, but without the response body.
    (head, Head);
    /// Describes the communication options for the target resource.
    (options, Options);
    /// Establishes a tunnel to the server identified by the target resource.
    (connect, Connect);
    /// Performs a message loop-back test along the path to the target resource.
    (trace, Trace);
}
