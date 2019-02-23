use std::collections::HashMap;
use std::hash::Hash;

use super::{ Trie, TrieMut, TrieView, TrieViewMut, HasView, HasViewMut };
use super::key_pair::{ KeyPair, Pair, HalfBorrowed };

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
        K: Hash + Eq {
    
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
pub type HashTrieEdge<K> = Pair<u32, K>;

/// An edge uniquely identifies the node it points to,
/// by indicating the node it is coming from and the key
/// that is followed to get there
pub type HashTrieEdgeView<'a, K> = HalfBorrowed<'a, u32, K>;

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

impl<'a, K: 'a, V: 'a> HasView<'a, K, V> for HashTrie<K, V>
    where 
        K: Hash + Eq {
    type View = HashTrieView<'a, K, V>;

    fn as_view(&'a self) -> Self::View {
        HashTrieView::new(self)
    }
}

impl<K, V> Trie<K, V> for HashTrie<K, V>
    where 
        K: Hash + Eq {
    
}

impl<'a, K: 'a, V: 'a> HasViewMut<'a, K, V> for HashTrie<K, V>
    where 
        K: Hash + Eq + Clone {
    type ViewMut = HashTrieViewMut<'a, K, V>;

    fn as_view_mut(&'a mut self) -> Self::ViewMut {
        HashTrieViewMut::new(self)
    }
}

impl<K, V> TrieMut<K, V> for HashTrie<K, V>
    where 
        K: Hash + Eq + Clone {

}

/// A read-only view of a HashTrie
pub struct HashTrieView<'a, K, V>
    where
        K: Hash + Eq {

    /// The HashTrie being Viewed
    trie: &'a HashTrie<K, V>,

    /// The Some edge leading to the current node,
    /// or None if the current node is the root
    edge: Option<HashTrieEdgeView<'a, K>>
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
    where K: Eq + Hash {

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
                if let Some(HashTrieNode::Leaf { value }) = map.get(last_edge as &KeyPair<u32, K>) {
                    Some(&value)
                } else {
                    None
                }
            },

            _ => None
        }
    }

    fn into_value<'b>(self) -> Option<&'b V> where Self: 'b {
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
                edge: Some(ref last_edge)
            } => {
                if let Some(HashTrieNode::Leaf { value }) = map.get(last_edge as &KeyPair<u32, K>) {
                    Some(&value)
                } else {
                    None
                }
            },

            _ => None
        }
    }

    fn descend(&self, key: &K) -> Option<Self> {
        match self {
            HashTrieView { 
                trie: HashTrie::Standard { map, .. }, 
                edge: None  //Indicates current node is root
            } => {
                let next_edge = HalfBorrowed(0, key); //Set previous node to 0

                Some(HashTrieView { 
                    trie: self.trie, 
                    edge: Some(next_edge)
                })
            },

            HashTrieView { 
                trie: HashTrie::Standard { map, .. }, 
                edge: Some(last_edge)
            } => {
                if let Some(HashTrieNode::Branch { id }) = map.get(last_edge as &KeyPair<u32, K>) {
                    let next_edge = HalfBorrowed(*id, key);

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

    fn value_mut(&mut self) -> Option<&mut V> {
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

    fn into_value_mut<'b>(self) -> Option<&'b mut V> where Self: 'b {
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
                edge: Some(ref last_edge)
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
    
    fn set_value(&mut self, new_value: V) -> bool
        where
            K: Clone {

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
                        let edge_clone = Pair(last_edge.0, last_edge.1.clone());

                        map.insert(edge_clone, HashTrieNode::Leaf { 
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
        let last_node;

        match &mut self_alias {
            HashTrieViewMut { 
                trie: HashTrie::Standard { .. }, 
                edge: None
            } => {
                last_node = 0; //Indicating root node
            },

            HashTrieViewMut { 
                trie: HashTrie::Standard { map, next_id }, 
                edge: Some(ref last_edge)
            } => {
                if let Some(HashTrieNode::Branch { id }) = map.get(last_edge as &KeyPair<u32, K>) {
                    last_node = *id;
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
            edge: Some(Pair(last_node, key))
        })
    }

    fn descend_or_add(self, key: K) -> Option<Self> {
        let mut self_alias = self;
        let mut last_node;

        match &mut self_alias {
            HashTrieViewMut { 
                trie: HashTrie::Standard { .. }, 
                edge: None
            } => {
                last_node = 0; //Indicating root node
            },

            HashTrieViewMut { 
                trie: HashTrie::Standard { map, next_id }, 
                edge: Some(ref last_edge)
            } => {
                if let Some(HashTrieNode::Branch { id }) = map.get(last_edge as &KeyPair<u32, K>) {
                    last_node = *id;
                } else {
                    last_node = *next_id;
                    
                    let edge_clone = Pair(last_edge.0, last_edge.1.clone());
                    
                    map.insert(edge_clone, HashTrieNode::Branch { id: *next_id });

                    *next_id += 1;  //Will currently panic when overflow occurs
                }
            },

            _ => {
                return None;
            }
        };

        Some(HashTrieViewMut { 
            trie: self_alias.trie, 
            edge: Some(Pair(last_node, key))
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

        let keys_a_get = vec![ "A", "A", "A" ];

        hash_trie.insert(keys_a.clone(), "A".to_string());

        assert_eq!(hash_trie.get(keys_a_get), Some(&"A".to_string()));
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

        assert_eq!(hash_trie.get(keys_a), Some(&"A".to_string()));

        assert_eq!(hash_trie.get(keys_b), Some(&"B".to_string()));
    }
}