use std::collections::HashMap;
use std::hash::Hash;

trait Trie<K, V> {
    type View<'a>: TrieView<'a, K, V>;

    fn insert(&mut self, path: &[K], new_val: V) -> bool;
    
    fn get(&self, path: &[K]) -> Option<&V>;

    fn as_view<'a>(&'a self) -> Self::View<'a>;
}

trait TrieView<'a, K, V> 
    where Self: Sized {

    fn value(&self) -> Option<&V>;

    fn get(&self, key: K) -> Option<Self>;
}

enum HashTrie<K, V> {
    Empty,
    
    Trivial {
        value: V
    },

    Standard {
        map: HashMap<(u32, K), HashTrieNode<V>>,
        next_id: u32
    }   
}

enum HashTrieNode<V> {
    Branch {
        id: u32
    },
    Leaf {
        value: V
    }
}

struct HashTrieView<'a, K, V> {
    trie: &'a HashTrie<K, V>,
    node: &'a HashTrieNode<V>
}

impl<K, V> HashTrie<K, V>  
    where K: Eq + Hash + Clone {

    fn new() -> Self {
        HashTrie::Standard {
            map: HashMap::new(),
            next_id: 1
        }
    }
}

impl<K, V> Trie<K, V> for HashTrie<K, V>
    where K: Hash + Eq {

    // type View<'a> = HashTrieView<'a, K, V>;

    fn insert(&mut self, path: &[K], new_val: V) -> bool {
        match self {
            HashTrie::Empty => {
                *self = HashTrie::Trivial {
                    value: new_val
                };
                true
            },

            HashTrie::Trivial { value } => {
                false
            },

            HashTrie::Standard { map, next_id } => insert_into_map(map, next_id, path, new_val)
        }
    }

    fn get(&self, path: &[K]) -> Option<&V> {
        match self {
            HashTrie::Empty => {
                None
            },

            HashTrie::Trivial { value } => {
                if path.is_empty() {
                    Some(&value)
                } else {
                    None
                }
            },

            HashTrie::Standard { map, next_id } => get_from_map(map, path)
        }
    }

    // fn as_view<'a>(&'a self) -> Self::View<'a> {
    //     HashTrieView {
    //         trie: &self,
    //         id: 0
    //     }
    // }
}

fn get_from_map<'a,K,V>(map: &'a HashMap<(u32, K), HashTrieNode<V>>, path: &[K]) -> Option<&'a V> 
        where K: Hash + Eq {

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

fn insert_into_map<'a, K, V>(
            map: &'a mut HashMap<(u32, K), HashTrieNode<V>>, 
            next_id: &'a mut u32,
            path: &[K],
            new_val: V
        ) -> bool    
        where K: Hash + Eq {

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