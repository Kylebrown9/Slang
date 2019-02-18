pub mod view;
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