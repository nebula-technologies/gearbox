use crate::common::BoxedFuture;
use crate::error::tracer::TracerError;
use crate::error::DynTracerError;
use crate::rails::ext::fut::{FutureResult, IntoFutureResult};
use crate::rails::ext::syn::RailsMapErrTracer;
use crate::{error_info, tracer_dyn_err, tracer_err};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};
use bytes::Bytes;
use core::future::Future;
use serde::{de::DeserializeOwned, Serialize};
use serde_derive::{Deserialize as DeriveDeserialize, Serialize as DeriveSerialize};
use spin::mutex::Mutex;

pub trait BodyTrait {
    fn into_bytes(&mut self) -> BoxedFuture<Result<Bytes, DynTracerError>>;
    fn into_string(&mut self) -> BoxedFuture<Result<String, DynTracerError>>;
    fn into_unchecked_string(&mut self) -> BoxedFuture<Result<String, DynTracerError>>;
    fn try_sync_into_string(&mut self) -> Result<String, DynTracerError>;
}

#[derive(Debug, Clone, DeriveSerialize, DeriveDeserialize)]
pub enum Body {
    Bytes(Vec<u8>),
    #[serde(skip)]
    Reference(Arc<Mutex<Option<reqwest::Response>>>),
    Empty,
}

impl Body {
    pub fn empty() -> Self {
        Body::Empty
    }
}

impl BodyTrait for Body {
    fn into_bytes(&mut self) -> BoxedFuture<Result<Bytes, DynTracerError>> {
        Box::pin(async move {
            match self {
                Body::Bytes(ref b) => Ok(Bytes::from(b.clone())),
                Body::Reference(r) => {
                    r.lock()
                        .take()
                        .ok_or(tracer_dyn_err!("Reference is empty"))
                        .into_future()
                        .and_then(|t| async { t.bytes().await.map_dyn_tracer_err(error_info!()) })
                        .await
                }
                Body::Empty => Ok(Bytes::new()),
            }
            .map_err(|e| tracer_dyn_err!(e))
            .map(|t| {
                *self = Body::Bytes(t.clone().to_vec());
                t
            })
        })
    }
    fn into_string(&mut self) -> BoxedFuture<Result<String, DynTracerError>> {
        Box::pin(async move {
            self.into_bytes()
                .await
                .and_then(|t| String::from_utf8(t.to_vec()).map_dyn_tracer_err(error_info!()))
        })
    }

    fn into_unchecked_string(&mut self) -> BoxedFuture<Result<String, DynTracerError>> {
        Box::pin(async move {
            self.into_bytes()
                .await
                .map(|t| unsafe { String::from_utf8_unchecked(t.to_vec()) })
        })
    }

    fn try_sync_into_string(&mut self) -> Result<String, DynTracerError> {
        match self {
            Body::Bytes(ref b) => String::from_utf8(b.clone()).map_dyn_tracer_err(error_info!()),
            Body::Reference(r) => Err(tracer_dyn_err!()),
            Body::Empty => Ok(String::new()),
        }
    }
}

impl From<reqwest::Response> for Body {
    fn from(r: reqwest::Response) -> Self {
        Body::Reference(Arc::new(Mutex::new(Some(r))))
    }
}
impl From<Box<reqwest::Response>> for Body {
    fn from(r: Box<reqwest::Response>) -> Self {
        Body::Reference(Arc::new(Mutex::new(Some(*r))))
    }
}

impl From<String> for Body {
    fn from(s: String) -> Self {
        Body::Bytes(s.into_bytes())
    }
}

impl From<&str> for Body {
    fn from(s: &str) -> Self {
        Body::Bytes(s.to_string().into_bytes())
    }
}

impl From<u8> for Body {
    fn from(s: u8) -> Self {
        Body::Bytes(vec![s])
    }
}

impl From<&[u8]> for Body {
    fn from(s: &[u8]) -> Self {
        Body::Bytes(s.to_vec())
    }
}
impl From<Vec<u8>> for Body {
    fn from(s: Vec<u8>) -> Self {
        Body::Bytes(s)
    }
}

#[derive(Debug, DeriveSerialize, DeriveDeserialize)]
pub struct BodyOwned<T = Body>
where
    T: BodyTrait + Serialize + DeserializeOwned,
{
    #[allow(unused)]
    #[serde(with = "body_owned_serde")]
    pub(crate) body: Mutex<Box<T>>,
}

impl BodyOwned {
    pub fn into_bytes(&self) -> BoxedFuture<Result<Bytes, DynTracerError>> {
        Box::pin(async move { self.body.lock().into_bytes().await })
    }
    pub fn into_str(&self) -> BoxedFuture<Result<String, DynTracerError>> {
        Box::pin(async move { self.body.lock().into_string().await })
    }
    pub async fn update_body<
        P: FnOnce(Box<Body>) -> O,
        O: Future<Output = Result<Box<Body>, DynTracerError>>,
    >(
        &mut self,
        o: P,
    ) -> Result<(), DynTracerError> {
        let mut body = self.body.lock();
        o(body.clone()).await.map(|t| {
            *body = t;
        })
    }
    pub fn try_sync_into_string(&self) -> Result<String, DynTracerError> {
        self.body.lock().try_sync_into_string()
    }
}

impl Default for BodyOwned {
    fn default() -> Self {
        BodyOwned {
            body: Mutex::new(Box::new(Body::Empty)),
        }
    }
}

impl Clone for BodyOwned {
    fn clone(&self) -> Self {
        BodyOwned {
            body: Mutex::new(self.body.lock().clone()),
        }
    }
}

mod body_owned_serde {
    use crate_serde::Serialize;
    use serde::{Deserialize, Deserializer, Serializer};
    use spin::Mutex;

    pub fn serialize<S, T>(mutex: &Mutex<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        let lock = mutex.lock();
        lock.serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Mutex<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Mutex::new(value))
    }
}

impl From<Body> for BodyOwned {
    fn from(b: Body) -> Self {
        BodyOwned {
            body: Mutex::new(Box::new(b)),
        }
    }
}

impl From<reqwest::Response> for BodyOwned {
    fn from(r: reqwest::Response) -> Self {
        BodyOwned {
            body: Mutex::new(Box::new(Body::from(r))),
        }
    }
}
impl From<Box<reqwest::Response>> for BodyOwned {
    fn from(r: Box<reqwest::Response>) -> Self {
        BodyOwned {
            body: Mutex::new(Box::new(Body::from(r))),
        }
    }
}

impl From<String> for BodyOwned {
    fn from(s: String) -> Self {
        BodyOwned {
            body: Mutex::new(Box::new(Body::from(s))),
        }
    }
}

impl From<&str> for BodyOwned {
    fn from(s: &str) -> Self {
        BodyOwned {
            body: Mutex::new(Box::new(Body::from(s))),
        }
    }
}

impl From<u8> for BodyOwned {
    fn from(s: u8) -> Self {
        BodyOwned {
            body: Mutex::new(Box::new(Body::from(s))),
        }
    }
}

impl From<&[u8]> for BodyOwned {
    fn from(s: &[u8]) -> Self {
        BodyOwned {
            body: Mutex::new(Box::new(Body::from(s))),
        }
    }
}

impl From<Vec<u8>> for BodyOwned {
    fn from(s: Vec<u8>) -> Self {
        BodyOwned {
            body: Mutex::new(Box::new(Body::from(s))),
        }
    }
}
