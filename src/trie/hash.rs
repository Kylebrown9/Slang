use std::collections::HashMap;
use std::hash::Hash;

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

/// Represents a mapping from Edges to Nodes
pub type HashTrieMap<K, V> = HashMap<HashTrieEdge<K>, HashTrieNode<V>>;

/// An edge uniquely identifies the node it points to,
/// by indicating the node it is coming from and the key
/// that is followed to get there
pub type HashTrieEdge<K> = Pair<u32, K>;

/// An edge uniquely identifies the node it points to,
/// by indicating the node it is coming from and the key
/// that is followed to get there
pub type HashTrieEdgeView<'b, K> = HalfBorrowed<'b, u32, K>;

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

impl<K, V> HashTrie<K, V>
    where 
        K: Hash + Eq + Clone {

    pub fn new() -> Self {
        HashTrie::Standard {
            map: HashMap::new(),
            //1 is the next_id of empty HashTries, because 0 is reserved for the root
            next_id: 1  
        }
    }

    pub fn as_view(&self) -> HashTrieView<'_, '_, K, V> {
        HashTrieView::new(self)
    }

    /// Gets the View for the specified node if it exists
    pub fn get_view<'c, I>(&self, path: I) -> Option<HashTrieView<'_, 'c, K, V>>
        where
            I: IntoIterator<Item = &'c K>,
            K: 'c {

        let mut iter = path.into_iter();

        let base_view: HashTrieView<'_, '_, K, V> = self.as_view();
        let mut view:  HashTrieView<'_, 'c, K, V>;

        if let Some(key) = iter.next() {
            if let Some(next_view) = base_view.descend(key) {
                view = next_view;
            } else {
                return None;
            }
        } else {
            return None;
        }

        while let Some(key) = iter.next() {
            if let Some(next_view) = view.descend(key) {
                view = next_view;
            } else {
                return None;
            }
        }

        Some(view)
    }

    /// Gets the value for the specified node if it exists
    pub fn get<'c, I>(&self, path: I) -> Option<&'_ V>
        where
            I: IntoIterator<Item = &'c K>,
            K: 'c {

        match self.get_view(path) {
            Some(view) => view.value(),
            None => None
        }
    }

    pub fn as_view_mut(&mut self) -> HashTrieViewMut<'_, K, V> {
        HashTrieViewMut::new(self)
    }

    /// Returns true if the insert succeeded
    pub fn insert<'a, T>(&mut self, path: T, new_val: V) -> bool
        where 
            T: IntoIterator<Item=K> {

        let mut view = self.as_view_mut();

        for key in path {
            let maybe_next = view.descend_or_add(key);

            if let Some(next_view) = maybe_next {
                view = next_view;
            } else {
                return false;
            }
        }

        let success = view.set_value(new_val);

        success
    }
}

/// A read-only view of a HashTrie
pub struct HashTrieView<'a, 'b, K, V>
    where
        K: Hash + Eq {

    /// The HashTrie being Viewed
    trie: &'a HashTrie<K, V>,

    /// The Some edge leading to the current node,
    /// or None if the current node is the root
    edge: Option<HashTrieEdgeView<'b, K>>
}

impl<'a, K, V> HashTrieView<'a, 'a, K, V>
    where
        K: Hash + Eq {

    fn new(hash_trie: &'a HashTrie<K,V>) -> Self {
        HashTrieView {
            trie: hash_trie,
            edge: None  //Indicates that the current node is the root
        }
    }
}

/// The 'a lifetime represents the HashTrie this is a view of
/// The 'b lifetime represents the lifetime of the key reference
///  used to create this view
/// The K and V parameters correspond to the signature of the HashTrie
impl<'a, 'b, K, V> HashTrieView<'a, 'b, K, V>
    where
        K: Hash + Eq {

    fn value(&self) -> Option<&'a V> {
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

    fn descend<'c>(&self, key: &'c K) -> Option<HashTrieView<'a, 'c, K, V>> {
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

    fn descend(mut self, key: K) -> Option<HashTrieViewMut<'a, K, V>> {
        let last_node;

        match &mut self {
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
            trie: self.trie, 
            edge: Some(Pair(last_node, key))
        })
    }

    fn descend_or_add(mut self, key: K) -> Option<Self> {
        let last_node;
        let trie_ref;

        match self {
            HashTrieViewMut { 
                trie,
                edge: None
            } => {
                last_node = 0; //Indicating root node
                trie_ref = trie;
            },

            HashTrieViewMut { 
                trie, 
                edge: Some(last_edge)
            } => {
                if let HashTrie::Standard { map, next_id } = trie {
                    if let Some(HashTrieNode::Branch { id }) = map.get(&last_edge as &KeyPair<u32, K>) {
                        last_node = *id;
                    } else {
                        last_node = *next_id;
                        
                        let edge_clone = Pair(last_edge.0, last_edge.1);
                        
                        map.insert(edge_clone, HashTrieNode::Branch { id: *next_id });

                        *next_id += 1;  //Will currently panic when overflow occurs
                    }
                } else {
                    return None;
                }

                trie_ref = trie;
            },

            _ => {
                return None;
            }
        };

        Some(HashTrieViewMut { 
            trie: trie_ref, 
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

        let keys_a_insert = vec![
            "A".to_string(), 
            "A".to_string(), 
            "A".to_string()
        ];

        let keys_a_get: Vec<&String> = keys_a_insert.iter().collect();

        hash_trie.insert(keys_a_insert.clone(), "A".to_string());

        assert_eq!(hash_trie.get(keys_a_get), Some(&"A".to_string()));
    }

    #[test]
    fn insert_insert_get() {
        let mut hash_trie = HashTrie::new();

        let keys_a_insert = vec![
            "A".to_string(), 
            "A".to_string(), 
            "A".to_string()
        ];

        let keys_a_get: Vec<&String> = keys_a_insert.iter().collect();

        hash_trie.insert(keys_a_insert.clone(), "A".to_string());

        let keys_b_insert = vec![
            "B".to_string(), 
            "B".to_string(), 
            "B".to_string()
        ];

        let keys_b_get: Vec<&String> = keys_b_insert.iter().collect();

        hash_trie.insert(keys_b_insert.clone(), "B".to_string());

        assert_eq!(hash_trie.get(keys_a_get), Some(&"A".to_string()));

        assert_eq!(hash_trie.get(keys_b_get), Some(&"B".to_string()));
    }
}