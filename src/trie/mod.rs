pub mod hash;

/**
 * The Base Trie API
 * 
 * Tri 
 */
pub trait Trie<K, V> {
    fn insert(&mut self, path: &[K], new_val: V) -> bool;
    
    fn get(&self, path: &[K]) -> Option<&V>;
}

/**
 * A TrieView represents a read only view of a trie node
 * It may optionally have a value
 */
pub trait TrieView<'a, K, V>: Sized {
    fn value(&self) -> Option<&'a V>;

    fn get(&self, key: K) -> Option<Self>;
}

pub trait TrieViewable<'a, K, V> {
    type View: TrieView<'a, K, V>;

    fn as_view(&'a self) -> Self::View;
}

