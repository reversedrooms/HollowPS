use std::{
    collections::{HashMap, HashSet},
    io::Result,
};

use crate::OctData;

pub type DoubleKeyHashMap<K1, K2, V> = HashMap<K1, HashMap<K2, V>>;

#[macro_export]
macro_rules! phashmap {
    ($(($key:expr, $value:expr)),*) => {
        PropertyHashMap::Base(
            {
                let mut map = std::collections::HashMap::new();
                $(
                    map.insert($key, $value);
                )*
                map
            }
        )
    };
}

#[macro_export]
macro_rules! phashset {
    ($($value:expr),*) => {
        PropertyHashSet::Base(
            {
                let mut set = std::collections::HashSet::new();
                $(
                    set.insert($value);
                )*
                set
            }
        )
    };
}

#[macro_export]
macro_rules! pdkhashmap {
    ($(($key1:expr, $key2:expr, $value:expr)),*) => {
        PropertyDoubleKeyHashMap::Base(
            {
                let mut map = qwer::DoubleKeyHashMap::new();
                $(
                    map.entry($key1).or_insert(HashMap::new()).insert($key2, $value);
                )*
                map
            }
        )
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyHashMap<K, V>
where
    K: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    Base(HashMap<K, V>),
    Modify {
        to_add: Vec<(K, V)>,
        to_remove: Vec<K>,
    },
}

impl<K, V> PropertyHashMap<K, V>
where
    K: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    pub fn insert(&mut self, key: K, value: V) {
        match self {
            Self::Base(base) => {
                base.insert(key, value);
            }
            Self::Modify { to_add, .. } => {
                to_add.push((key, value));
            }
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        match self {
            Self::Base(base) => base.remove(&key),
            Self::Modify { to_remove, .. } => {
                to_remove.push(key);
                None
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self {
            Self::Base(base) => base.get(key),
            Self::Modify { .. } => None,
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self {
            Self::Base(base) => base.get_mut(key),
            Self::Modify { .. } => None,
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Base(base) => base.len(),
            Self::Modify { to_add, to_remove } => to_add.len() + to_remove.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Base(base) => base.is_empty(),
            Self::Modify { to_add, to_remove } => to_add.is_empty() && to_remove.is_empty(),
        }
    }

    #[must_use]
    pub fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
        match self {
            Self::Base(base) => base.iter(),
            Self::Modify { .. } => unreachable!("PropertyHashMap::Modify::iter()"),
        }
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<K, V> {
        match self {
            Self::Base(base) => base.iter_mut(),
            Self::Modify { .. } => unreachable!("PropertyHashMap::Modify::iter_mut()"),
        }
    }
}

impl<K, V> IntoIterator for PropertyHashMap<K, V>
where
    K: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;
    type Item = (K, V);
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Base(base) => base.into_iter(),
            Self::Modify { .. } => unreachable!("PropertyHashMap::Modify::into_iter()"),
        }
    }
}

impl<'a, K, V> IntoIterator for &'a PropertyHashMap<K, V>
where
    K: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    type IntoIter = std::collections::hash_map::Iter<'a, K, V>;
    type Item = (&'a K, &'a V);
    fn into_iter(self) -> Self::IntoIter {
        match self {
            PropertyHashMap::Base(base) => base.iter(),
            PropertyHashMap::Modify { .. } => unreachable!("PropertyHashMap::Modify::into_iter()"),
        }
    }
}

impl<'a, K, V> IntoIterator for &'a mut PropertyHashMap<K, V>
where
    K: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    type IntoIter = std::collections::hash_map::IterMut<'a, K, V>;
    type Item = (&'a K, &'a mut V);
    fn into_iter(self) -> Self::IntoIter {
        match self {
            PropertyHashMap::Base(base) => base.iter_mut(),
            PropertyHashMap::Modify { .. } => unreachable!("PropertyHashMap::Modify::into_iter()"),
        }
    }
}

#[test]
fn test_hashmap_iter() {
    let mut map = phashmap![(1, 2), (3, 4)];
    let mut expecting = vec![(1, 2), (3, 4)];
    let iter = map.iter_mut();
    for (key, value) in iter {
        assert!(expecting.contains(&(*key, *value)));
        expecting.retain(|x| x != &(*key, *value));
        *value += 1;
    }
}

impl<T> PropertyHashSet<T>
where
    T: OctData + Eq + std::hash::Hash,
{
    pub fn insert(&mut self, value: T) {
        match self {
            Self::Base(base) => {
                base.insert(value);
            }
            Self::Modify { to_add, .. } => {
                to_add.push(value);
            }
        }
    }

    pub fn remove(&mut self, value: T) {
        match self {
            Self::Base(base) => {
                base.remove(&value);
            }
            Self::Modify { to_remove, .. } => {
                to_remove.push(value);
            }
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Base(base) => base.len(),
            Self::Modify { to_add, to_remove } => to_add.len() + to_remove.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Base(base) => base.is_empty(),
            Self::Modify { to_add, to_remove } => to_add.is_empty() && to_remove.is_empty(),
        }
    }

    #[must_use]
    pub fn iter(&self) -> std::collections::hash_set::Iter<T> {
        match self {
            Self::Base(base) => base.iter(),
            Self::Modify { .. } => unreachable!("PropertyHashSet::Modify::iter()"),
        }
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> std::collections::hash_set::Iter<T> {
        self.into_iter()
    }
}

impl<T> IntoIterator for PropertyHashSet<T>
where
    T: OctData + Eq + std::hash::Hash,
{
    type IntoIter = std::collections::hash_set::IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Base(base) => base.into_iter(),
            Self::Modify { .. } => unreachable!("PropertyHashSet::Modify::into_iter()"),
        }
    }
}

impl<'a, T> IntoIterator for &'a PropertyHashSet<T>
where
    T: OctData + Eq + std::hash::Hash,
{
    type IntoIter = std::collections::hash_set::Iter<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            PropertyHashSet::Base(base) => base.iter(),
            PropertyHashSet::Modify { .. } => unreachable!("PropertyHashSet::Modify::into_iter()"),
        }
    }
}

impl<'a, T> IntoIterator for &'a mut PropertyHashSet<T>
where
    T: OctData + Eq + std::hash::Hash,
{
    type IntoIter = std::collections::hash_set::Iter<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            PropertyHashSet::Base(base) => base.iter(),
            PropertyHashSet::Modify { .. } => unreachable!("PropertyHashSet::Modify::into_iter()"),
        }
    }
}

#[test]
fn test_phashmap_macro() {
    let map = phashmap![(1, 2), (3, 4)];
    assert_eq!(
        map,
        PropertyHashMap::Base([(1, 2), (3, 4)].into_iter().collect())
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyHashSet<T>
where
    T: OctData + Eq + std::hash::Hash,
{
    Base(HashSet<T>),
    Modify { to_add: Vec<T>, to_remove: Vec<T> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyDoubleKeyHashMap<K1, K2, V>
where
    K1: OctData + Eq + std::hash::Hash,
    K2: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    Base(DoubleKeyHashMap<K1, K2, V>),
    Modify {
        to_add: Vec<(K1, K2, V)>,
        to_remove: Vec<(K1, K2)>,
    },
}

pub struct PropertyDoubleKeyHashMapIterMut<'a, K1, K2, V> {
    outer_iter: std::collections::hash_map::IterMut<'a, K1, HashMap<K2, V>>,
    inner_iter: Option<std::collections::hash_map::IterMut<'a, K2, V>>,
    current_outer_key: Option<&'a K1>,
}

pub struct PropertyDoubleKeyHashMapIter<'a, K1, K2, V> {
    outer_iter: std::collections::hash_map::Iter<'a, K1, HashMap<K2, V>>,
    inner_iter: Option<std::collections::hash_map::Iter<'a, K2, V>>,
    current_outer_key: Option<&'a K1>,
}

pub struct PropertyDoubleKeyHashMapIntoIter<K1, K2, V> {
    outer_iter: std::collections::hash_map::IntoIter<K1, HashMap<K2, V>>,
    inner_iter: Option<std::collections::hash_map::IntoIter<K2, V>>,
    current_outer_key: Option<K1>,
}

impl<'a, K1, K2, V> Iterator for PropertyDoubleKeyHashMapIterMut<'a, K1, K2, V>
where
    K1: OctData + Eq + std::hash::Hash,
    K2: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    type Item = (&'a K1, &'a K2, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.inner_iter.is_none() {
                if let Some((key1, sub_map)) = self.outer_iter.next() {
                    self.current_outer_key = Some(key1);
                    self.inner_iter = Some(sub_map.iter_mut());
                } else {
                    return None;
                }
            }

            if let Some(inner_iter) = &mut self.inner_iter {
                if let Some((key2, value)) = inner_iter.next() {
                    return Some((self.current_outer_key.unwrap(), key2, value));
                }
                self.inner_iter = None;
            }
        }
    }
}

impl<'a, K1, K2, V> Iterator for PropertyDoubleKeyHashMapIter<'a, K1, K2, V>
where
    K1: OctData + Eq + std::hash::Hash,
    K2: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    type Item = (&'a K1, &'a K2, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.inner_iter.is_none() {
                if let Some((key1, sub_map)) = self.outer_iter.next() {
                    self.current_outer_key = Some(key1);
                    self.inner_iter = Some(sub_map.iter());
                } else {
                    return None;
                }
            }

            if let Some(inner_iter) = &mut self.inner_iter {
                if let Some((key2, value)) = inner_iter.next() {
                    return Some((self.current_outer_key.unwrap(), key2, value));
                }
                self.inner_iter = None;
            }
        }
    }
}

impl<K1, K2, V> Iterator for PropertyDoubleKeyHashMapIntoIter<K1, K2, V>
where
    K1: OctData + Eq + std::hash::Hash + Copy,
    K2: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    type Item = (K1, K2, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.inner_iter.is_none() {
                if let Some((key1, sub_map)) = self.outer_iter.next() {
                    self.current_outer_key = Some(key1);
                    self.inner_iter = Some(sub_map.into_iter());
                } else {
                    return None;
                }
            }

            if let Some(inner_iter) = &mut self.inner_iter {
                if let Some((key2, value)) = inner_iter.next() {
                    return Some((self.current_outer_key.unwrap(), key2, value));
                }
                self.inner_iter = None;
            }
        }
    }
}

impl<K1, K2, V> IntoIterator for PropertyDoubleKeyHashMap<K1, K2, V>
where
    K1: OctData + Eq + std::hash::Hash + Copy,
    K2: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    type IntoIter = PropertyDoubleKeyHashMapIntoIter<K1, K2, V>;
    type Item = (K1, K2, V);

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Base(base) => PropertyDoubleKeyHashMapIntoIter {
                outer_iter: base.into_iter(),
                inner_iter: None,
                current_outer_key: None,
            },
            Self::Modify { .. } => {
                unreachable!("PropertyDoubleKeyHashMap::Modify::into_iter() is not implemented")
            }
        }
    }
}

impl<'a, K1, K2, V> IntoIterator for &'a PropertyDoubleKeyHashMap<K1, K2, V>
where
    K1: OctData + Eq + std::hash::Hash,
    K2: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    type IntoIter = PropertyDoubleKeyHashMapIter<'a, K1, K2, V>;
    type Item = (&'a K1, &'a K2, &'a V);

    fn into_iter(self) -> Self::IntoIter {
        match self {
            PropertyDoubleKeyHashMap::Base(base) => PropertyDoubleKeyHashMapIter {
                outer_iter: base.iter(),
                inner_iter: None,
                current_outer_key: None,
            },
            PropertyDoubleKeyHashMap::Modify { .. } => {
                unreachable!("PropertyDoubleKeyHashMap::Modify::into_iter() is not implemented")
            }
        }
    }
}

impl<'a, K1, K2, V> IntoIterator for &'a mut PropertyDoubleKeyHashMap<K1, K2, V>
where
    K1: OctData + Eq + std::hash::Hash,
    K2: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    type IntoIter = PropertyDoubleKeyHashMapIterMut<'a, K1, K2, V>;
    type Item = (&'a K1, &'a K2, &'a mut V);

    fn into_iter(self) -> Self::IntoIter {
        match self {
            PropertyDoubleKeyHashMap::Base(base) => PropertyDoubleKeyHashMapIterMut {
                outer_iter: base.iter_mut(),
                inner_iter: None,
                current_outer_key: None,
            },
            PropertyDoubleKeyHashMap::Modify { .. } => {
                unreachable!("PropertyDoubleKeyHashMap::Modify::into_iter() is not implemented")
            }
        }
    }
}

impl<K1, K2, V> PropertyDoubleKeyHashMap<K1, K2, V>
where
    K1: OctData + Eq + std::hash::Hash,
    K2: OctData + Eq + std::hash::Hash,
    V: OctData,
{
    pub fn insert(&mut self, key: K1, sub_key: K2, value: V) {
        match self {
            Self::Base(base) => {
                base.entry(key)
                    .or_insert_with(HashMap::new)
                    .insert(sub_key, value);
            }
            Self::Modify { to_add, .. } => {
                to_add.push((key, sub_key, value));
            }
        }
    }

    pub fn get(&self, key: &K1, sub_key: &K2) -> Option<&V> {
        match self {
            Self::Base(base) => base.get(key).and_then(|sub_map| sub_map.get(sub_key)),
            Self::Modify { .. } => {
                unreachable!("PropertyDoubleKeyHashMap::Modify::get() is not implemented")
            }
        }
    }

    pub fn get_mut(&mut self, key: &K1, sub_key: &K2) -> Option<&mut V> {
        match self {
            Self::Base(base) => base
                .get_mut(key)
                .and_then(|sub_map| sub_map.get_mut(sub_key)),
            Self::Modify { .. } => {
                unreachable!("PropertyDoubleKeyHashMap::Modify::get_mut() is not implemented")
            }
        }
    }

    pub fn remove(&mut self, key: K1, sub_key: K2) -> Option<V> {
        match self {
            Self::Base(base) => {
                let mut removed = None;
                base.entry(key)
                    .and_modify(|sub_map| removed = sub_map.remove(&sub_key));
                removed
            }
            Self::Modify { to_remove, .. } => {
                to_remove.push((key, sub_key));
                None
            }
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Base(base) => base.len(),
            Self::Modify { to_add, to_remove } => to_add.len() + to_remove.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Base(base) => base.is_empty(),
            Self::Modify { to_add, to_remove } => to_add.is_empty() && to_remove.is_empty(),
        }
    }

    #[must_use]
    pub fn iter(&self) -> PropertyDoubleKeyHashMapIter<K1, K2, V> {
        match self {
            Self::Base(base) => PropertyDoubleKeyHashMapIter {
                outer_iter: base.iter(),
                inner_iter: None,
                current_outer_key: None,
            },
            Self::Modify { .. } => {
                unreachable!("PropertyDoubleKeyHashMap::Modify::iter() is not implemented")
            }
        }
    }

    pub fn iter_mut(&mut self) -> PropertyDoubleKeyHashMapIterMut<K1, K2, V> {
        match self {
            Self::Base(base) => PropertyDoubleKeyHashMapIterMut {
                outer_iter: base.iter_mut(),
                inner_iter: None,
                current_outer_key: None,
            },
            Self::Modify { .. } => {
                unreachable!("PropertyDoubleKeyHashMap::Modify::iter_mut() is not implemented")
            }
        }
    }
}

#[test]
fn test_dkhashmap_iter() {
    let mut map = PropertyDoubleKeyHashMap::Base(
        std::iter::once((1, [(2, 3), (4, 5)].into_iter().collect())).collect(),
    );
    let mut expecting = vec![(1, 2, 3), (1, 4, 5)];
    let iter = map.iter_mut();
    for (key1, key2, value) in iter {
        assert!(expecting.contains(&(*key1, *key2, *value)));
        expecting.retain(|x| x != &(*key1, *key2, *value));
        *value += 1;
    }
}

impl<K, V> OctData for PropertyHashMap<K, V>
where
    K: OctData + Eq + Ord + std::hash::Hash,
    V: OctData,
{
    fn marshal_to<W: std::io::Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        match self {
            Self::Base(map) => {
                map.marshal_to(w, bt_property_tag)?;
            }
            Self::Modify { to_add, to_remove } => {
                let len = -(to_add.len() as i32 + to_remove.len() as i32);
                len.marshal_to(w, bt_property_tag)?;
                for (key, value) in to_add {
                    key.marshal_to(w, bt_property_tag)?;
                    false.marshal_to(w, bt_property_tag)?;
                    value.marshal_to(w, bt_property_tag)?;
                }
                for key in to_remove {
                    key.marshal_to(w, bt_property_tag)?;
                    true.marshal_to(w, bt_property_tag)?;
                }
            }
        }
        Ok(())
    }

    fn unmarshal_from<R: std::io::Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;

        if len >= 0 {
            let mut map = HashMap::with_capacity(len as usize);
            for _ in 0..len {
                map.insert(
                    K::unmarshal_from(r, bt_property_tag)?,
                    V::unmarshal_from(r, bt_property_tag)?,
                );
            }
            Ok(Self::Base(map))
        } else {
            let mut to_add = Vec::new();
            let mut to_remove = Vec::new();

            for _ in 0..-len {
                let key = K::unmarshal_from(r, bt_property_tag)?;
                if !bool::unmarshal_from(r, bt_property_tag)? {
                    to_add.push((key, V::unmarshal_from(r, bt_property_tag)?));
                } else {
                    to_remove.push(key);
                }
            }

            Ok(Self::Modify { to_add, to_remove })
        }
    }
}

impl<K> OctData for PropertyHashSet<K>
where
    K: OctData + Eq + std::hash::Hash,
{
    fn marshal_to<W: std::io::Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        match self {
            Self::Base(set) => {
                set.marshal_to(w, bt_property_tag)?;
            }
            Self::Modify { to_add, to_remove } => {
                let len = -(to_add.len() as i32 + to_remove.len() as i32);
                len.marshal_to(w, bt_property_tag)?;
                for value in to_add {
                    value.marshal_to(w, bt_property_tag)?;
                    false.marshal_to(w, bt_property_tag)?;
                }
                for value in to_remove {
                    value.marshal_to(w, bt_property_tag)?;
                    true.marshal_to(w, bt_property_tag)?;
                }
            }
        }
        Ok(())
    }

    fn unmarshal_from<R: std::io::Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;

        if len >= 0 {
            let mut set = HashSet::with_capacity(len as usize);
            for _ in 0..len {
                set.insert(K::unmarshal_from(r, bt_property_tag)?);
            }
            Ok(Self::Base(set))
        } else {
            let mut to_add = Vec::new();
            let mut to_remove = Vec::new();

            for _ in 0..-len {
                let value = K::unmarshal_from(r, bt_property_tag)?;
                if !bool::unmarshal_from(r, bt_property_tag)? {
                    to_add.push(value);
                } else {
                    to_remove.push(value);
                }
            }

            Ok(Self::Modify { to_add, to_remove })
        }
    }
}

impl<K1, K2, V> OctData for PropertyDoubleKeyHashMap<K1, K2, V>
where
    K1: OctData + Eq + Ord + std::hash::Hash,
    K2: OctData + Eq + Ord + std::hash::Hash,
    V: OctData,
{
    fn marshal_to<W: std::io::Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        match self {
            Self::Base(map) => {
                map.marshal_to(w, bt_property_tag)?;
            }
            Self::Modify { to_add, to_remove } => {
                let len = -(to_add.len() as i32 + to_remove.len() as i32);
                len.marshal_to(w, bt_property_tag)?;
                for (key1, key2, value) in to_add {
                    key1.marshal_to(w, bt_property_tag)?;
                    key2.marshal_to(w, bt_property_tag)?;
                    false.marshal_to(w, bt_property_tag)?;
                    value.marshal_to(w, bt_property_tag)?;
                }
                for (key1, key2) in to_remove {
                    key1.marshal_to(w, bt_property_tag)?;
                    key2.marshal_to(w, bt_property_tag)?;
                    true.marshal_to(w, bt_property_tag)?;
                }
            }
        }
        Ok(())
    }

    fn unmarshal_from<R: std::io::Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;
        if len >= 0 {
            let mut map = HashMap::new();
            for _ in 0..len {
                let key1 = K1::unmarshal_from(r, bt_property_tag)?;
                let key2 = K2::unmarshal_from(r, bt_property_tag)?;
                let value = V::unmarshal_from(r, bt_property_tag)?;
                map.entry(key1)
                    .or_insert_with(HashMap::new)
                    .insert(key2, value);
            }
            Ok(Self::Base(map))
        } else {
            let mut to_add = Vec::new();
            let mut to_remove = Vec::new();

            for _ in 0..-len {
                let key1 = K1::unmarshal_from(r, bt_property_tag)?;
                let key2 = K2::unmarshal_from(r, bt_property_tag)?;
                if !bool::unmarshal_from(r, bt_property_tag)? {
                    to_add.push((key1, key2, V::unmarshal_from(r, bt_property_tag)?));
                } else {
                    to_remove.push((key1, key2));
                }
            }

            Ok(Self::Modify { to_add, to_remove })
        }
    }
}
