use core::borrow::Borrow;
use core::cell::UnsafeCell;
use core::fmt;
use core::fmt::Debug;
use core::hash::Hash;
use core::iter::FromIterator;
use core::ops::Index;
use core::sync::atomic::{AtomicBool, Ordering};
use hashbrown::hash_map::HashMap as GBHashMap;
use hashbrown::{hash_map, TryReserveError};
#[cfg(feature = "with_serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "with_serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct HashMap<K, V> {
    locked: AtomicBool,
    data: UnsafeCell<Option<GBHashMap<K, V>>>,
}

impl<K, V> HashMap<K, V> {
    pub const fn new() -> HashMap<K, V> {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(None),
        }
    }

    fn lock(&self) -> bool {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
        true
    }

    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }

    fn ensure_initialized(&self) {
        if self.lock() {
            unsafe {
                if (*self.data.get()).is_none() {
                    *self.data.get() = Some(GBHashMap::new());
                }
            }
            self.unlock();
        }
    }

    fn as_inner(&self) -> &GBHashMap<K, V> {
        self.ensure_initialized();
        let inner = unsafe { self.data.get().as_ref().unwrap().as_ref().unwrap() };
        inner
    }

    fn as_inner_mut(&mut self) -> &mut GBHashMap<K, V> {
        self.ensure_initialized();
        let inner = self.data.get_mut().as_mut().unwrap();
        inner
    }

    fn into_inner(self) -> GBHashMap<K, V> {
        self.ensure_initialized();
        unsafe { (*self.data.get()).take().unwrap() }
    }
}

impl<K, V> HashMap<K, V> {
    pub fn capacity(&self) -> usize {
        self.as_inner().capacity()
    }

    pub fn keys(&self) -> hash_map::Keys<'_, K, V> {
        self.as_inner().keys()
    }

    pub fn into_keys(self) -> hash_map::IntoKeys<K, V> {
        self.into_inner().into_keys()
    }

    pub fn values(&self) -> hash_map::Values<'_, K, V> {
        self.as_inner().values()
    }

    pub fn values_mut(&mut self) -> hash_map::ValuesMut<'_, K, V> {
        self.as_inner_mut().values_mut()
    }

    pub fn into_values(self) -> hash_map::IntoValues<K, V> {
        self.into_inner().into_values()
    }

    pub fn iter(&self) -> hash_map::Iter<'_, K, V> {
        self.as_inner().iter()
    }

    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, K, V> {
        self.as_inner_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_inner().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_inner().is_empty()
    }

    pub fn drain(&mut self) -> hash_map::Drain<'_, K, V> {
        self.as_inner_mut().drain()
    }

    pub fn extract_if<F>(&mut self, pred: F) -> hash_map::ExtractIf<'_, K, V, F>
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.as_inner_mut().extract_if(pred)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.as_inner_mut().retain(f)
    }

    pub fn clear(&mut self) {
        self.as_inner_mut().clear()
    }
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn reserve(&mut self, additional: usize) {
        self.as_inner_mut().reserve(additional)
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.as_inner_mut().try_reserve(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.as_inner_mut().shrink_to_fit()
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.as_inner_mut().shrink_to(min_capacity)
    }

    pub fn entry(&mut self, key: K) -> hash_map::Entry<'_, K, V, hash_map::DefaultHashBuilder> {
        self.as_inner_mut().entry(key)
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.as_inner().get(k)
    }

    pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.as_inner().get_key_value(k)
    }

    pub fn get_many_mut<Q: ?Sized, const N: usize>(&mut self, ks: [&Q; N]) -> Option<[&'_ mut V; N]>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.as_inner_mut().get_many_mut(ks)
    }

    pub unsafe fn get_many_unchecked_mut<Q: ?Sized, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> Option<[&'_ mut V; N]>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.as_inner_mut().get_many_unchecked_mut(ks)
    }

    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.as_inner().contains_key(k)
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.as_inner_mut().get_mut(k)
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.as_inner_mut().insert(k, v)
    }

    pub fn try_insert(
        &mut self,
        key: K,
        value: V,
    ) -> Result<&mut V, hash_map::OccupiedError<'_, K, V, hash_map::DefaultHashBuilder>> {
        self.as_inner_mut().try_insert(key, value)
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.as_inner_mut().remove(k)
    }

    pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.as_inner_mut().remove_entry(k)
    }
}

impl<K, V> HashMap<K, V> {
    pub fn raw_entry_mut(
        &mut self,
    ) -> hash_map::RawEntryBuilderMut<'_, K, V, hash_map::DefaultHashBuilder> {
        self.as_inner_mut().raw_entry_mut()
    }

    pub fn raw_entry(&self) -> hash_map::RawEntryBuilder<'_, K, V, hash_map::DefaultHashBuilder> {
        self.as_inner().raw_entry()
    }
}

impl<K, V> Clone for HashMap<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        self.ensure_initialized();
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(Some(self.as_inner().clone())),
        }
    }

    fn clone_from(&mut self, other: &Self) {
        self.ensure_initialized();
        unsafe {
            (*self.data.get())
                .as_mut()
                .unwrap()
                .clone_from(&other.as_inner());
        }
    }
}

impl<K, V> PartialEq for HashMap<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
    fn eq(&self, other: &HashMap<K, V>) -> bool {
        self.as_inner().eq(other.as_inner())
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
        self.as_inner().fmt(f)
    }
}

impl<K, Q: ?Sized, V> Index<&Q> for HashMap<K, V>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash,
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.as_inner().get(key).expect("no entry found for key")
    }
}

#[cfg(feature = "with_serde")]
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
        for (k, v) in self.as_inner() {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[cfg(feature = "with_serde")]
impl<'de, K, V> Deserialize<'de> for HashMap<K, V>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hb_hash_map = GBHashMap::deserialize(deserializer)?;
        Ok(HashMap {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(Some(hb_hash_map)),
        })
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
        let mut hb_hash_map = Self::new();
        for (k, v) in iter {
            hb_hash_map.insert(k, v);
        }
        hb_hash_map
    }
}

// Implement IntoIterator for HashMap
impl<K, V> IntoIterator for HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inner().into_iter()
    }
}

// Implement IntoIterator for &HashMap

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = hash_map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_inner().iter()
    }
}

// Implement IntoIterator for &mut HashMap
impl<'a, K, V> IntoIterator for &'a mut HashMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = hash_map::IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_inner_mut().iter_mut()
    }
}
