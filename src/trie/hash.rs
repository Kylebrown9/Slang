use std::collections::HashMap;
use std::hash::Hash;

use super::{ Trie, TrieMut, TrieView, TrieViewMut };

/// A Trie/TrieMut implementor, that stores all nodes
/// in a single HashMap
pub enum HashTrie<K, V>
    where 
        K: Hash + Eq {

    /// A Trivial HashTrie is one whose root (representing the empty sequence)
    /// is mapped to a value. 
    /// Since the empty sequence is a prefix of all other sequences,
    /// no other data can be stored.
    Trivial {
        value: V
    },

    /// A Standard HashTrie is any non-trivial one, including the empty Trie.
    /// Nodes are represented by unsigned integers (with the root being zero),
    /// and contains a mapping from edges to nodes, as well as the next unused node id.
    Standard {
        map: HashTrieMap<K, V>,
        next_id: u32
    }   
}

impl<K, V> HashTrie<K, V>
    where 
        K: Hash + Eq + Clone {
    
    /// Constructs an empty HashTrie
    pub fn new() -> Self {
        HashTrie::Standard {
            map: HashMap::new(),
            //1 is the next_id of empty HashTries, because 0 is reserved for the root
            next_id: 1  
        }
    }
}

/// Represents a mapping from Edges to Nodes
pub type HashTrieMap<K, V> = HashMap<HashTrieEdge<K>, HashTrieNode<V>>;

/// An edge uniquely identifies the node it points to,
/// by indicating the node it is coming from and the key
/// that is followed to get there
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct HashTrieEdge<K>
    where
        K: Hash + Eq {

    prev_node: u32,
    edge_key: K
}

/// A node in the HashTrie
pub enum HashTrieNode<V> {
    /// A Branch has an id so that it can be represented as
    /// the previous_node for edges.
    /// It does not have a value because that would 
    /// violate the prefix-free property
    Branch {
        id: u32
    },
    
    /// A Leaf has a value that it holds, but has no id
    /// because Leafs are never the previous_node for an edge.
    Leaf {
        value: V
    }
}

impl<'a, K, V> Trie<K, V> for &'a HashTrie<K, V>
    where 
        K: Hash + Eq + Clone {

    type View = HashTrieView<'a, K, V>;

    fn as_view(self) -> HashTrieView<'a, K, V> {
        HashTrieView::new(self)
    }
}

impl<'a, K, V> Trie<K, V> for &'a mut HashTrie<K, V>
    where 
        K: Hash + Eq + Clone {

    type View = HashTrieView<'a, K, V>;

    fn as_view(self) -> HashTrieView<'a, K, V> {
        HashTrieView::new(self)
    }
}

impl<'a, K, V> TrieMut<K, V> for &'a mut HashTrie<K, V>
    where 
        K: Hash + Eq + Clone {

    type ViewMut = HashTrieViewMut<'a, K, V>;

    fn as_view_mut(self) -> HashTrieViewMut<'a, K, V> {
        HashTrieViewMut::new(self)
    }
}

/// A read-only view of a HashTrie
pub struct HashTrieView<'a, K, V>
    where
        K: Hash + Eq {

    /// The HashTrie being Viewed
    trie: &'a HashTrie<K, V>,

    /// The Some edge leading to the current node,
    /// or None if the current node is the root
    edge: Option<HashTrieEdge<K>>
}

impl<'a, K, V> HashTrieView<'a, K, V>
    where
        K: Hash + Eq {

    fn new(hash_trie: &'a HashTrie<K,V>) -> Self {
        HashTrieView {
            trie: hash_trie,
            edge: None  //Indicates that the current node is the root
        }
    }
}

impl<'a, K, V> TrieView<K, V> for HashTrieView<'a, K, V> 
    where K: Eq + Hash + Clone {

    fn value(&self) -> Option<&V> {
        match self {
            HashTrieView {
                trie: HashTrie::Trivial {
                    value
                },
                edge: None  //Indicates current node is root
            } => {
                Some(&value)
            },

            HashTrieView {
                trie: HashTrie::Standard { map, .. },
                edge: Some(last_edge)
            } => {
                if let Some(HashTrieNode::Leaf { value }) = map.get(last_edge) {
                    Some(&value)
                } else {
                    None
                }
            },

            _ => None
        }
    }

    fn descend(&self, key: K) -> Option<Self> {
        match self {
            HashTrieView { 
                trie: HashTrie::Standard { map, .. }, 
                edge: None  //Indicates current node is root
            } => {
                let next_edge = HashTrieEdge {
                    prev_node: 0,   //Set previous node to 0
                    edge_key: key
                };

                Some(HashTrieView { 
                    trie: self.trie, 
                    edge: Some(next_edge)
                })
            },

            HashTrieView { 
                trie: HashTrie::Standard { map, .. }, 
                edge: Some(last_edge)
            } => {
                if let Some(HashTrieNode::Branch { id }) = map.get(last_edge) {
                    let next_edge = HashTrieEdge {
                        prev_node: *id,
                        edge_key: key.clone()
                    };

                    Some(HashTrieView { 
                        trie: self.trie, 
                        edge: Some(next_edge)
                    })
                } else {
                    None
                }
            },

            _ => None
        }
    }
}

/// A mutable view of a HashTrie
pub struct HashTrieViewMut<'a, K, V>
    where
        K: Hash + Eq {
            
    /// The HashTrie being Viewed
    trie: &'a mut HashTrie<K, V>,

    /// The Some edge leading to the current node,
    /// or None if the current node is the root
    edge: Option<HashTrieEdge<K>>
}

impl<'a, K, V> HashTrieViewMut<'a, K, V>
    where
        K: Hash + Eq {

    fn new(hash_trie: &'a mut HashTrie<K,V>) -> Self {
        HashTrieViewMut {
            trie: hash_trie,
            edge: None  //Indicates that the current node is the root
        }
    }
}

impl<'a, K, V> TrieViewMut<K, V> for HashTrieViewMut<'a, K, V> 
    where K: Eq + Hash + Clone {

    fn value(&mut self) -> Option<&mut V> {
        match self {
            HashTrieViewMut {
                trie: HashTrie::Trivial {
                    value
                },
                edge: None  //Indicates current node is root
            } => {
                Some(value)
            },

            HashTrieViewMut {
                trie: HashTrie::Standard { map, .. },
                edge: Some(last_edge)
            } => {
                if let Some(HashTrieNode::Leaf { value }) = map.get_mut(last_edge) {
                    Some(value)
                } else {
                    None
                }
            },

            _ => None
        }
    }
    
    fn set_value(&mut self, new_value: V) -> bool {
        let make_trivial;

        match self {
            HashTrieViewMut {
                trie: HashTrie::Standard { map, .. },
                edge: None  //Indicates current node is root
            } => {
                make_trivial = map.is_empty();
            },

            HashTrieViewMut {
                trie: HashTrie::Standard { map, .. },
                edge: Some(last_edge)
            } => {
                match map.get_mut(last_edge) {
                    None => {
                        map.insert(last_edge.clone(), HashTrieNode::Leaf { 
                            value: new_value 
                        });
                        return true;
                    },

                    Some(HashTrieNode::Leaf { value }) => {
                        *value = new_value;
                        return true;
                    }

                    _ => {
                        return false;
                    }
                }
            },

            _ => {
                return false;
            }
        };

        if make_trivial {
            *self.trie = HashTrie::Trivial { value: new_value };
            true
        } else {
            false
        }
    }

    fn descend(self, key: K) -> Option<Self> {
        let mut self_alias = self;
        let next_edge;

        match &mut self_alias {
            HashTrieViewMut { 
                trie: HashTrie::Standard { .. }, 
                edge: None
            } => {
                next_edge = HashTrieEdge {
                    prev_node: 0,   //Indicating root node
                    edge_key: key
                };
            },

            HashTrieViewMut { 
                trie: HashTrie::Standard { map, next_id }, 
                edge: Some(ref last_edge)
            } => {
                if let Some(HashTrieNode::Branch { id }) = map.get(&last_edge) {
                    next_edge = HashTrieEdge {
                        prev_node: *id,
                        edge_key: key
                    };
                } else {
                    return None;
                }
            },

            _ => {
                return None;
            }
        };

        Some(HashTrieViewMut { 
            trie: self_alias.trie, 
            edge: Some(next_edge)
        })
    }

    fn descend_or_add(self, key: K) -> Option<Self> {
        let mut self_alias = self;
        let mut next_edge;

        match &mut self_alias {
            HashTrieViewMut { 
                trie: HashTrie::Standard { .. }, 
                edge: None
            } => {
                next_edge = HashTrieEdge {
                    prev_node: 0,   //Indicating root node
                    edge_key: key
                };
            },

            HashTrieViewMut { 
                trie: HashTrie::Standard { map, next_id }, 
                edge: Some(ref last_edge)
            } => {
                if let Some(HashTrieNode::Branch { id }) = map.get(&last_edge) {
                    next_edge = HashTrieEdge {
                        prev_node: *id,
                        edge_key: key
                    };
                } else {
                    next_edge = HashTrieEdge {
                        prev_node: *next_id,
                        edge_key: key
                    };
                    
                    map.insert(last_edge.clone(), HashTrieNode::Branch { id: *next_id });

                    *next_id += 1;  //Will currently panic when overflow occurs
                }
            },

            _ => {
                return None;
            }
        };

        Some(HashTrieViewMut { 
            trie: self_alias.trie, 
            edge: Some(next_edge)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_get() {
        let mut hash_trie = HashTrie::new();

        let keys_a = vec![
            "A".to_string(), 
            "A".to_string(), 
            "A".to_string()
        ];

        hash_trie.insert(keys_a.clone(), "A".to_string());

        assert_eq!(hash_trie.get_view(keys_a).unwrap().value(), Some(&"A".to_string()));
    }

    #[test]
    fn insert_insert_get() {
        let mut hash_trie = HashTrie::new();

        let keys_a = vec![
            "A".to_string(), 
            "A".to_string(), 
            "A".to_string()
        ];

        hash_trie.insert(keys_a.clone(), "A".to_string());

        let keys_b = vec![
            "B".to_string(), 
            "B".to_string(), 
            "B".to_string()
        ];

        hash_trie.insert(keys_b.clone(), "B".to_string());

        assert_eq!(hash_trie.get_view(keys_a).unwrap().value(), Some(&"A".to_string()));

        assert_eq!(hash_trie.get_view(keys_b).unwrap().value(), Some(&"B".to_string()));
    }
}