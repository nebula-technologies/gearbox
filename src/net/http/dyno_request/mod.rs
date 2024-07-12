pub mod header;
pub mod response;
pub mod status_code;
pub mod url;

use header::HeaderMap;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ApiLayout {
    pub group_description: Option<String>,
    pub servers: Vec<url::Url>,
    pub endpoints: HashMap<String, MethodLayout>,
}

pub enum MethodLayout {
    Get(GetLayout),
    Post(PostLayout),
    Put(PutLayout),
    Delete(DeleteLayout),
    Patch(PatchLayout),
    Options(OptionsLayout),
    Head(HeadLayout),
    Connect(ConnectLayout),
    Trace(TraceLayout),
}

pub struct GetLayout {
    pub description: Option<String>,
    pub headers: HeaderMap,
    pub request: RequestContent,
    pub responses: ExpectedResponses,
}

pub struct PostLayout {
    pub singleuse: bool,
    pub description: Option<String>,
    pub headers: HeaderMap,
    pub request: RequestContent,
    pub responses: ExpectedResponses,
}

pub struct PutLayout {
    pub description: Option<String>,
    pub headers: HeaderMap,
    pub request: RequestContent,
    pub responses: ExpectedResponses,
}

pub struct DeleteLayout {
    pub description: Option<String>,
    pub headers: HeaderMap,
    pub request: Option<RequestContent>,
    pub responses: ExpectedResponses,
}

pub struct PatchLayout {
    pub description: Option<String>,
    pub headers: HeaderMap,
    pub request: RequestContent,
    pub responses: ExpectedResponses,
}

pub struct OptionsLayout {
    pub description: Option<String>,
    pub headers: HeaderMap,
    pub responses: ExpectedResponses,
}

pub struct HeadLayout {
    pub description: Option<String>,
    pub headers: HeaderMap,
    pub responses: ExpectedResponses,
}

pub struct ConnectLayout {
    pub description: Option<String>,
    pub headers: HeaderMap,
    pub request: RequestContent,
    pub responses: ExpectedResponses,
}

pub struct TraceLayout {
    pub description: Option<String>,
    pub headers: HeaderMap,
    pub request: RequestContent,
    pub responses: ExpectedResponses,
}

pub struct RequestContent {
    pub content: HashMap<String, Schema>,
}

pub enum Schema {
    String,
    Properties(Properties),
    Array(Vec<Schema>),
    Object(HashMap<String, Schema>),
    Boolean,
    Number,
    Integer,
    Null,
}

pub struct Properties {
    pub fields: HashMap<String, PropertyType>,
}

pub struct PropertyType {
    pub r#type: String,
}

/// Matches parts of a response for variable capturing.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Matcher {
    #[serde(skip_serializing_if = "Option::is_none")]
    between: Option<(String, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regexp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xpath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<bool>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Matcher {
    /// Creates a matcher for capturing text between two strings.
    pub fn between(from: String, to: String) -> Matcher {
        Matcher {
            between: Some((from, to)),
            regexp: None,
            xpath: None,
            all: None,
        }
    }

    /// Creates a matcher for capturing text using a regular expression.
    pub fn regexp(regexp: String) -> Matcher {
        Matcher {
            between: None,
            regexp: Some(regexp),
            xpath: None,
            all: None,
        }
    }

    /// Creates a matcher for capturing all text.
    pub fn all(all: bool) -> Matcher {
        Matcher {
            between: None,
            regexp: None,
            all: Some(all),
            xpath: None,
        }
    }
}
