use crate::collections::HashMap;
use core::{
    any::{type_name, TypeId},
    ops::{Deref, DerefMut},
};
use spin::RwLock;

static TYPE_REGISTRY: RwLock<Option<TypeRegistry>> = RwLock::new(None);

pub fn get_type_id(name: &str) -> Option<TypeId> {
    TYPE_REGISTRY
        .read()
        .as_ref()
        .and_then(|t| t.get_type_id(name).map(|t| t.clone()))
}
pub fn get_type_name(id: &TypeId) -> Option<String> {
    TYPE_REGISTRY
        .read()
        .as_ref()
        .and_then(|t| t.get_type_name(id).map(|t| t.clone()))
}

pub fn register_type<T: 'static>() {
    TYPE_REGISTRY
        .write()
        .get_or_insert(TypeRegistry::default())
        .register_type::<T>();
}

#[derive(Default)]
pub struct TypeRegistry(HashMap<String, TypeId>, HashMap<TypeId, String>);

impl TypeRegistry {
    pub fn register_type<T: 'static>(&mut self) {
        let type_name = type_name::<T>().to_string();
        let type_id = TypeId::of::<T>();
        self.0.insert(type_name.clone(), type_id);
        self.1.insert(type_id, type_name);
    }

    pub fn get_type_id(&self, type_name: &str) -> Option<TypeId> {
        self.0.get(type_name).cloned()
    }
    pub fn get_type_name(&self, type_id: &TypeId) -> Option<String> {
        self.1.get(type_id).cloned()
    }
}

impl Deref for TypeRegistry {
    type Target = HashMap<String, TypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TypeRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
