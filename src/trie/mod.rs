pub mod hash;

use std::borrow::Borrow;

/**
 * The Trie trait represents a mapping from
 * a sequence of key elements to a single value.
 * 
 * This mapping can be used similarly to HashMap
 * and provides get(&self) and insert(&mut self, T, V) support.
 * 
 * No key element sequence can be the prefix of any other
 * This must be enforced by implementors to disambiguate parsing
 */
pub trait Trie<K, V> {
    fn insert<T>(&mut self, path: T, new_val: V) -> bool
        where T: IntoIterator<Item=K>;
    
    fn get<T>(&self, path: T) -> Option<&V>
        where
            T: IntoIterator, 
            K: Borrow<T::Item>;
}

/**
 * The TrieView trait represents a read only view
 * of a Trie node. 
 * 
 * 
 */
pub trait TrieView<'a, K, V>: Sized {
    fn value(&self) -> Option<&'a V>;

    fn get(&self, key: K) -> Option<Self>;
}

pub trait TrieViewable<'a, K, V> {
    type View: TrieView<'a, K, V>;

    fn as_view(&'a self) -> Self::View;
}

