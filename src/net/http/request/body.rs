use alloc::sync::Arc;

pub trait BodyTrait {}

pub enum Body {
    Bytes(Vec<u8>),
    Reference(Arc<reqwest::Response>),
}

impl BodyTrait for Body {}

impl From<reqwest::Response> for Body {
    fn from(r: reqwest::Response) -> Self {
        Body::Reference(Arc::new(r))
    }
}
impl From<Arc<reqwest::Response>> for Body {
    fn from(r: Arc<reqwest::Response>) -> Self {
        Body::Reference(r)
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

pub struct BodyOwned {
    body: Box<dyn BodyTrait>,
}

impl BodyOwned {}

impl From<Body> for BodyOwned {
    fn from(b: Body) -> Self {
        BodyOwned { body: Box::new(b) }
    }
}

impl From<reqwest::Response> for BodyOwned {
    fn from(r: reqwest::Response) -> Self {
        BodyOwned {
            body: Box::new(Body::from(r)),
        }
    }
}
impl From<Arc<reqwest::Response>> for BodyOwned {
    fn from(r: Arc<reqwest::Response>) -> Self {
        BodyOwned {
            body: Box::new(Body::from(r)),
        }
    }
}
