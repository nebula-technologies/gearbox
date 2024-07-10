use alloc::{string::String, vec::Vec};

use serde_derive::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg_attr(feature = "std", derive(uniffi::Object))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Body {
    data_body: Option<Vec<u8>>,
    text_body: Option<String>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Body {
    pub fn from_bytes(body: Vec<u8>) -> Body {
        Body {
            data_body: Some(body),
            text_body: None,
        }
    }
    pub fn from_text(body: String) -> Body {
        Body {
            data_body: None,
            text_body: Some(body),
        }
    }
}
