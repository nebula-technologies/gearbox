use core::borrow::Borrow;
use core::fmt;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::Index;
use crate_serde::ser::SerializeMap;
use crate_serde::{Deserialize, Deserializer, Serialize, Serializer};
use hashbrown::hash_map;
use hashbrown::HashMap as HBHashMap;
use hashbrown::TryReserveError;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct HashMap<K, V>(HBHashMap<K, V>);

impl<K, V> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        Self(HBHashMap::default())
    }
}

impl<K, V> HashMap<K, V> {
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn keys(&self) -> hash_map::Keys<'_, K, V> {
        self.0.keys()
    }

    pub fn into_keys(self) -> hash_map::IntoKeys<K, V> {
        self.0.into_keys()
    }

    pub fn values(&self) -> hash_map::Values<'_, K, V> {
        self.0.values()
    }

    pub fn values_mut(&mut self) -> hash_map::ValuesMut<'_, K, V> {
        self.0.values_mut()
    }

    pub fn into_values(self) -> hash_map::IntoValues<K, V> {
        self.0.into_values()
    }

    pub fn iter(&self) -> hash_map::Iter<'_, K, V> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, K, V> {
        self.0.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn drain(&mut self) -> hash_map::Drain<'_, K, V> {
        self.0.drain()
    }

    pub fn extract_if<F>(&mut self, pred: F) -> hash_map::ExtractIf<'_, K, V, F>
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.0.extract_if(pred)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.0.retain(f)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.0.try_reserve(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.0.shrink_to(min_capacity);
    }

    pub fn entry(&mut self, key: K) -> hash_map::Entry<'_, K, V, hash_map::DefaultHashBuilder> {
        self.0.entry(key)
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get(k)
    }

    pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get_key_value(k)
    }

    pub fn get_many_mut<Q: ?Sized, const N: usize>(&mut self, ks: [&Q; N]) -> Option<[&'_ mut V; N]>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get_many_mut(ks)
    }

    pub unsafe fn get_many_unchecked_mut<Q: ?Sized, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> Option<[&'_ mut V; N]>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get_many_unchecked_mut(ks)
    }

    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.contains_key(k)
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get_mut(k)
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.0.insert(k, v)
    }

    pub fn try_insert(
        &mut self,
        key: K,
        value: V,
    ) -> Result<&mut V, hash_map::OccupiedError<'_, K, V, hash_map::DefaultHashBuilder>> {
        self.0.try_insert(key, value)
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.remove(k)
    }

    pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.remove_entry(k)
    }
}

impl<K, V> HashMap<K, V> {
    pub fn raw_entry_mut(
        &mut self,
    ) -> hash_map::RawEntryBuilderMut<'_, K, V, hash_map::DefaultHashBuilder> {
        self.0.raw_entry_mut()
    }

    pub fn raw_entry(&self) -> hash_map::RawEntryBuilder<'_, K, V, hash_map::DefaultHashBuilder> {
        self.0.raw_entry()
    }
}

impl<K, V> Clone for HashMap<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, other: &Self) {
        self.0.clone_from(&other.0);
    }
}

impl<K, V> PartialEq for HashMap<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
    fn eq(&self, other: &HashMap<K, V>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V> Eq for HashMap<K, V>
where
    K: Eq + Hash,
    V: Eq,
{
}

impl<K, V> Debug for HashMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, Q: ?Sized, V> Index<&Q> for HashMap<K, V>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash,
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

// Implement Serialize
impl<K, V> Serialize for HashMap<K, V>
where
    K: Serialize + Eq + Hash,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in &self.0 {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

// Implement Deserialize
impl<'de, K, V> Deserialize<'de> for HashMap<K, V>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hb_hash_map = HBHashMap::deserialize(deserializer)?;
        Ok(HashMap(hb_hash_map))
    }
}

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// Implement FromIterator
impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut hb_hash_map = HBHashMap::new();
        for (k, v) in iter {
            hb_hash_map.insert(k, v);
        }
        HashMap(hb_hash_map)
    }
}

// Implement IntoIterator for HashMap
impl<K, V> IntoIterator for HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// Implement IntoIterator for &HashMap
impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = hash_map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// Implement IntoIterator for &mut HashMap
impl<'a, K, V> IntoIterator for &'a mut HashMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = hash_map::IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
