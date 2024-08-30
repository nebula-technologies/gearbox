pub mod sld;

use crate::time::DateTime;
use alloc::{collections::btree_map::BTreeMap, string::String, vec::Vec};

pub type DocumentId = String;
pub type NameSpace = String;
pub type ElementIdentifier = String;

pub struct BaseDocument {
    pub id: DocumentId,
    pub doc_type: String,
    pub name: String,
    pub hardware_backed: bool,
    pub created_at: DateTime,
    pub requires_user_auth: DateTime,
    pub name_spaced_data: BTreeMap<NameSpace, BTreeMap<ElementIdentifier, Vec<u8>>>,
}
