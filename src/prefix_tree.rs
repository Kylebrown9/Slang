use std::collections::HashMap;
use std::hash::Hash;

trait Trie<K, V> {
    fn new() -> Self;

    fn get(&self, path: &[K]) -> Option<V>;

    fn insert(&mut self, path: &[K], new_val: V) -> Option<V>;
}

enum HashTrie<K, V> {
    Branch {
        map: HashMap<K, HashTrie<K, V>>,
        value: Option<V>
    },
    PathCompressed {
        path: Vec<K>,
        next: Box<HashTrie<K, V>>
    },
    Leaf {
        value: V
    },
    Empty
}

impl<K, V> Trie<K, V> for HashTrie<K, V> 
    where K: Eq + Hash + Clone {

    fn new() -> Self {
        HashTrie::Branch {
            map: HashMap::new(),
            value: None
        }
    }

    fn get(&self, key: &[K]) -> Option<V> {
        match self {
            HashTrie::Branch { map, value } => {
                if let Some(k) = key.get(0) {
                    if let Some(node) = map.get(k) {
                        node.get(&key[1..])
                    } else {
                        None
                    }
                } else {
                    *value
                }
            },

            HashTrie::PathCompressed { path, next } => {
                if &key[ .. path.len() ] == &path[ .. ] {
                    next.get(&key[ path.len() .. ])
                } else {
                    None
                }
            },

            HashTrie::Leaf { value } => {
                if key.is_empty() {
                    Some(*value)
                } else {
                    None
                }
            },

            HashTrie::Empty => {
                None
            }
        }
    }

    fn insert(&mut self, path: &[K], new_val: V) -> Option<V> {
        match self {
            HashTrie::Branch { map, value } => {
                //When the path has a first value
                if let Some(k) = path.get(0) {

                    //When the first path value has a Trie
                    if let Some(node) = map.get(k) {
                        node.insert(&path[1..], new_val)
                    } 
                    
                    //When the first path value is not assigned
                    else {
                        match path.len() {
                            1 => {
                                map.insert(*k, HashTrie::Leaf { value: new_val });
                            },
                            _ => {
                                map.insert(*k, HashTrie::PathCompressed {
                                    path: Vec::from(&path[ 1 .. ]),
                                    next: Box::new(HashTrie::Leaf { value: new_val })
                                });
                            }
                        }
                        None
                    }
                } 
                //When the path is empty
                else {
                    let old_val = *value;
                    *value = Some(new_val);
                    old_val
                }
            },

            HashTrie::PathCompressed { path, next } => {
                if &path[ .. path.len() ] == &path[ .. ] {
                    next.insert(&path[ path.len() .. ], new_val)
                } else {
                    None //TODO
                }
            },

            HashTrie::Leaf { value } => {
                *self = HashTrie::Branch {
                    map: HashMap::new(),
                    value: Some(*value)
                };
                
                self.insert(path, new_val)
            },

            HashTrie::Empty => {
                *self = HashTrie::Leaf {
                    value: new_val
                };
                None
            }
        }
    }
}