use std::{borrow::Borrow, vec::IntoIter};

#[derive(Debug, Clone, PartialEq)]
pub struct VecMap<K, V> {
    vec: Vec<(K, V)>,
}

impl<K, V> Default for VecMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> FromIterator<(K, V)> for VecMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self {
            vec: Vec::from_iter(iter),
        }
    }
}

impl<K, V> VecMap<K, V> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn insert(&mut self, key: K, mut value: V) -> Option<V>
    where
        K: PartialEq + Eq,
    {
        if let Some((_, v)) = self.vec.iter_mut().find(|(k, _)| k == &key) {
            std::mem::swap(&mut value, v);
            Some(value)
        } else {
            self.vec.push((key, value));
            None
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq + Eq + ?Sized,
    {
        self.vec
            .iter_mut()
            .enumerate()
            .find_map(|(idx, (k, _))| ((*k).borrow() == key).then_some(idx))
            .map(|idx| self.vec.remove(idx))
            .map(|(_, v)| v)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq + Eq + ?Sized,
    {
        self.vec
            .iter()
            .find(|(k, _)| (*k).borrow() == key)
            .map(|(_, v)| v)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: PartialEq + Eq + ?Sized,
    {
        self.vec
            .iter_mut()
            .find(|(k, _)| (*k).borrow() == key)
            .map(|(_, v)| v)
    }

    pub fn get_by_index(&self, idx: usize) -> Option<(&K, &V)> {
        self.vec.get(idx).map(|(k, v)| (k, v))
    }

    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: PartialEq + Eq + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.vec.iter().map(|(k, v)| (k, v))
    }
}

impl<K, V> IntoIterator for VecMap<K, V> {
    type Item = (K, V);

    type IntoIter = IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_map() {
        let map: VecMap<i32, &str> = VecMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_default_creates_empty_map() {
        let map: VecMap<i32, &str> = Default::default();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_from_iterator() {
        let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
        let map: VecMap<i32, &str> = pairs.into_iter().collect();
        assert_eq!(map.len(), 3);
        assert!(!map.is_empty());
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));
    }

    #[test]
    fn test_len_with_items() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_is_empty_with_items() {
        let mut map = VecMap::new();
        assert!(map.is_empty());
        map.insert(1, "one");
        assert!(!map.is_empty());
    }

    #[test]
    fn test_insert_new_key() {
        let mut map = VecMap::new();
        let result = map.insert(1, "one");
        assert_eq!(result, None);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), Some(&"one"));
    }

    #[test]
    fn test_insert_existing_key() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        let result = map.insert(1, "uno");
        assert_eq!(result, Some("one"));
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), Some(&"uno"));
    }

    #[test]
    fn test_insert_multiple_keys() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));
    }

    #[test]
    fn test_remove_existing_key() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        let result = map.remove(&1);
        assert_eq!(result, Some("one"));
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), None);
        assert_eq!(map.get(&2), Some(&"two"));
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut map: VecMap<i32, &str> = VecMap::new();
        map.insert(1, "one");
        let result = map.remove(&2);
        assert_eq!(result, None);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_remove_last_key() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        let result = map.remove(&1);
        assert_eq!(result, Some("one"));
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_get_existing_key() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let map: VecMap<i32, &str> = VecMap::new();
        assert_eq!(map.get(&1), None);
    }

    #[test]
    fn test_get_by_index_valid() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        assert_eq!(map.get_by_index(0), Some((&1, &"one")));
        assert_eq!(map.get_by_index(1), Some((&2, &"two")));
    }

    #[test]
    fn test_get_by_index_invalid() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        assert_eq!(map.get_by_index(1), None);
    }

    #[test]
    fn test_contains_existing_key() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        assert!(map.contains(&1));
    }

    #[test]
    fn test_contains_nonexistent_key() {
        let map: VecMap<i32, &str> = VecMap::new();
        assert!(!map.contains(&1));
    }

    #[test]
    fn test_iter_empty() {
        let map: VecMap<i32, &str> = VecMap::new();
        let items: Vec<_> = map.iter().collect();
        assert!(items.is_empty());
    }

    #[test]
    fn test_iter_with_items() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        map.insert(2, "two");

        let mut items: Vec<_> = map.iter().collect();
        items.sort_by_key(|&(k, _)| *k);

        assert_eq!(items, vec![(&1, &"one"), (&2, &"two")]);
    }

    #[test]
    fn test_string_keys() {
        let mut map = VecMap::new();
        map.insert("one".to_string(), 1);
        map.insert("two".to_string(), 2);

        assert_eq!(map.get("one"), Some(&1));
        assert_eq!(map.get("two"), Some(&2));
    }

    #[test]
    fn test_remove_with_string_slice() {
        let mut map = VecMap::new();
        map.insert("one".to_string(), 1);

        assert_eq!(map.remove("one"), Some(1));
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_complex_types() {
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct ComplexKey(i32, String);

        #[derive(Debug, PartialEq, Eq, Clone)]
        struct ComplexValue {
            field1: i32,
            field2: String,
        }

        let mut map = VecMap::new();
        let key = ComplexKey(1, "test".to_string());
        let value = ComplexValue {
            field1: 42,
            field2: "value".to_string(),
        };

        map.insert(key.clone(), value.clone());

        assert_eq!(map.get(&key), Some(&value));
    }

    #[test]
    fn test_borrow_behavior() {
        let mut map = VecMap::new();
        map.insert("one".to_string(), 1);

        // Test with a borrowed value
        assert_eq!(map.get("one"), Some(&1));
        assert!(map.contains("one"));
    }

    #[test]
    fn test_insert_returns_old_value() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        let old_value = map.insert(1, "first");

        assert_eq!(old_value, Some("one"));
        assert_eq!(map.get(&1), Some(&"first"));
    }

    #[test]
    fn test_insert_many_items() {
        let mut map = VecMap::new();
        for i in 0..100 {
            map.insert(i, i.to_string());
        }

        assert_eq!(map.len(), 100);

        for i in 0..100 {
            assert_eq!(map.get(&i), Some(&i.to_string()));
        }
    }

    #[test]
    fn test_remove_many_items() {
        let mut map = VecMap::new();
        for i in 0..100 {
            map.insert(i, i.to_string());
        }

        for i in 0..100 {
            assert_eq!(map.remove(&i), Some(i.to_string()));
        }

        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_iter_preserves_insertion_order() {
        let mut map = VecMap::new();
        map.insert(3, "three");
        map.insert(1, "one");
        map.insert(2, "two");

        let items: Vec<_> = map.iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(items, vec![(3, "three"), (1, "one"), (2, "two")]);
    }

    #[test]
    fn test_update_via_insert() {
        let mut map = VecMap::new();
        map.insert(1, "one");
        map.insert(1, "ONE");
        map.insert(1, "One");

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), Some(&"One"));
    }

    #[test]
    fn test_clone() {
        let mut original = VecMap::new();
        original.insert(1, "one");
        original.insert(2, "two");

        let cloned = original.clone();

        assert_eq!(original.len(), cloned.len());
        assert_eq!(original.get(&1), cloned.get(&1));
        assert_eq!(original.get(&2), cloned.get(&2));
    }

    #[test]
    fn test_debug_formatting() {
        let mut map = VecMap::new();
        map.insert(1, "one");

        let debug_str = format!("{map:?}");
        assert!(debug_str.contains("VecMap"));
    }

    #[test]
    fn test_overwrite_value() {
        let mut map = VecMap::new();
        let mut value = String::from("original");
        map.insert(1, value);

        // Create a new String with the same content
        value = String::from("new");
        map.insert(1, value);

        assert_eq!(map.get(&1), Some(&String::from("new")));
    }

    #[test]
    fn test_from_iterator_with_duplicates() {
        let pairs = vec![(1, "one"), (2, "two"), (1, "ONE")];
        let map: VecMap<i32, &str> = pairs.into_iter().collect();

        assert_eq!(map.len(), 3); // Note: VecMap doesn't deduplicate during collect
        assert_eq!(map.get(&1), Some(&"one")); // Gets the first occurrence
    }

    #[test]
    fn test_option_value() {
        let mut map = VecMap::new();
        map.insert(1, Some("one"));
        map.insert(2, None);

        assert_eq!(map.get(&1), Some(&Some("one")));
        assert_eq!(map.get(&2), Some(&None));
    }

    #[test]
    fn test_remove_and_reinsert() {
        let mut map = VecMap::new();
        map.insert(1, "one");

        let removed = map.remove(&1);
        assert_eq!(removed, Some("one"));

        map.insert(1, "ONE");
        assert_eq!(map.get(&1), Some(&"ONE"));
    }

    #[test]
    fn test_map_ordering_stability() {
        let mut map = VecMap::new();

        for i in 0..10 {
            map.insert(i, i.to_string());
        }

        let original_order: Vec<_> = map.iter().map(|(k, _)| *k).collect();

        // Update some values
        for i in 0..10 {
            if i % 2 == 0 {
                map.insert(i, format!("even_{i}"));
            }
        }

        let new_order: Vec<_> = map.iter().map(|(k, _)| *k).collect();

        // The order should be preserved
        assert_eq!(original_order, new_order);
    }

    #[test]
    fn test_string_key_with_str_get() {
        let mut map = VecMap::new();

        // Insert with String keys
        map.insert(String::from("hello"), 1);
        map.insert(String::from("world"), 2);

        // Get values using &str references
        assert_eq!(map.get("hello"), Some(&1));
        assert_eq!(map.get("world"), Some(&2));

        // Get a non-existent key
        assert_eq!(map.get("not_found"), None);

        // Test contains with &str
        assert!(map.contains("hello"));
        assert!(!map.contains("other"));

        // Test remove with &str
        assert_eq!(map.remove("hello"), Some(1));
        assert_eq!(map.get("hello"), None);
    }
}
