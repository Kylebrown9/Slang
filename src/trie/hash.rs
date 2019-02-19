use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::Borrow;

use super::{ Trie, TrieView, TrieViewable };

pub enum HashTrie<K, V> {
    Empty,
    
    Trivial {
        value: V
    },

    Standard {
        map: HashTrieMap<K, V>,
        next_id: u32
    }   
}

type HashTrieMap<K, V> = HashMap<(u32, K), HashTrieNode<V>>;

enum HashTrieNode<V> {
    Branch {
        id: u32
    },
    
    Leaf {
        value: V
    }
}

impl<K, V> HashTrie<K, V>  
    where K: Eq + Hash + Clone {

    pub fn new() -> Self {
        HashTrie::Empty
    }
}

impl<K, V> Trie<K, V> for HashTrie<K, V>
    where K: Hash + Eq {

    fn insert<T>(&mut self, path: T, new_val: V) -> bool 
            where T: IntoIterator<Item=K> {

        match self {
            HashTrie::Empty => {
                let path_iter = path.into_iter();

                let path_empty = ( path_iter.count() == 0 );

                if path_empty {
                    *self = HashTrie::Trivial {
                        value: new_val
                    };
                    true
                } else {
                    false
                }
            },

            HashTrie::Trivial { value } => false,

            HashTrie::Standard { map, next_id } => 
                insert_into_map(map, next_id, path, new_val)
        }
    }

    fn get<T>(&self, path: T) -> Option<&V>
        where
            T: IntoIterator, 
            K: Borrow<T::Item> {

        match self {
            HashTrie::Empty => None,

            HashTrie::Trivial { value } => {
                let path_iter = path.into_iter();

                let path_empty = ( path_iter.count() == 0 );

                if path_empty {
                    Some(&value)
                } else {
                    None
                }
            },

            HashTrie::Standard { map, next_id } => 
                get_from_map(map, path)
        }
    }
}

fn insert_into_map<'a, K, V, T>(
            map: &'a mut HashMap<(u32, K), HashTrieNode<V>>, 
            next_id: &'a mut u32,
            path: T,
            new_val: V
        ) -> bool    
        where 
            K: Hash + Eq,
            T: IntoIterator<Item=K> {

    let last_index = path.len()-1;
    let body = &path[ .. last_index ];
    let tail = path[last_index];

    let mut current = 0;

    for k in body {
        match map.get(&(current, *k)) {
            Some( HashTrieNode::Branch { id } ) => {
                current = *id;
            },
            Some( HashTrieNode::Leaf { value} ) => {
                return false;
            },
            None => {
                //Order of all 3 statements is important
                map.insert( 
                    (current, *k),
                    HashTrieNode::Branch {
                        id: *next_id
                    }
                );
                
                current = *next_id; 

                *next_id += 1;

                break;
            }
        }
    }

    if !map.contains_key(&(current, tail)) {
        map.insert(
            (current, tail), 
            HashTrieNode::Leaf { value: new_val }
        );
        true
    } else {
        false
    }
}

fn get_from_map<'a, K, V, T>(
            map: &'a HashTrieMap<K, V>, 
            path: T) -> Option<&'a V> 
        where 
            K: Hash + Eq + Borrow<T::Item>,
            T: IntoIterator {

    if path.is_empty() {
        return None;
    }

    let last_index = path.len()-1;
    let body = &path[ .. last_index ];
    let tail = path[last_index];

    let mut current = 0;

    for k in body {
        match map.get(&(current, *k)) {
            Some( HashTrieNode::Branch { id } ) => {
                current = *id;
            },
            Some( HashTrieNode::Leaf { value} ) => {
                return None;
            },
            None => {
                return None;
            }
        }
    }

    if let Some( HashTrieNode::Leaf { value } ) = map.get(&(current, tail)) {
        Some(&value)
    } else {
        None
    }
}

pub struct HashTrieView<'a, K, V> {
    trie: &'a HashTrie<K, V>,
    node: &'a HashTrieNode<V>
}

impl<'a, K, V> TrieView<'a, K, V> for HashTrieView<'a, K, V> 
    where K: Eq + Hash {

    fn value(&self) -> Option<&'a V> {
        match self {
            HashTrieView { 
                trie, 
                node: HashTrieNode::Leaf { value } 
            } => Some(&value),
            _ => None
        }
    }

    fn get(&self, key: K) -> Option<Self> {
        match self {
            HashTrieView { 
                trie: HashTrie::Standard { map, next_id }, 
                node: HashTrieNode::Branch { id } 
            } => Some(HashTrieView {
                trie: self.trie,
                node: map.get( &(*id, key) )?
            }),
            _ => None
        }
    }
}

impl<'a, K, V> TrieViewable<'a, K, V> for HashTrie<K, V>
    where K: 'static + Eq + Hash, 
        V: 'static {

    type View = HashTrieView<'a, K, V>;

    fn as_view(&'a self) -> HashTrieView<'a, K, V> {
        HashTrieView {
            trie: self,
            node: &HashTrieNode::Branch {
                id: 0
            }
        }
    }
}

#[cfg(test)]
mod test {

}