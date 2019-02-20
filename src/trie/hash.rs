use std::collections::HashMap;
use std::hash::Hash;

use super::{ Trie, TrieView, TrieViewMut };

pub enum HashTrie<K, V> {
    Trivial {
        value: V
    },

    Standard {
        map: HashTrieMap<K, V>,
        next_id: u32
    }   
}

pub type HashTrieMap<K, V> = HashMap<(u32, K), HashTrieNode<V>>;

pub enum HashTrieNode<V> {
    Branch {
        id: u32
    },
    
    Leaf {
        value: V
    }
}

impl<'a, K, V> Trie<'a, K, V> for HashTrie<K, V>
    where 
        K: 'a + Hash + Eq + Clone,
        V: 'a {

    type ViewMut = HashTrieViewMut<'a, K, V>;
    type View = HashTrieView<'a, K, V>;

    fn new() -> Self {
        HashTrie::Standard {
            map: HashMap::new(),
            next_id: 1
        }
    }

    fn as_view(&self) -> HashTrieView<'_, K, V> {
        HashTrieView {
            trie: self,
            node: None
        }
    }

    fn as_view_mut(&mut self) -> HashTrieViewMut<'_, K, V> {
        HashTrieViewMut {
            trie: self,
            node: None
        }
    }
}

pub struct HashTrieView<'a, K, V> {
    trie: &'a HashTrie<K, V>,
    node: Option<(u32, K)>
}

impl<'a, K, V> TrieView<'a, K, V> for HashTrieView<'a, K, V> 
    where K: Eq + Hash + Clone {

    fn value(&'a self) -> Option<&'a V> {
        match self {
            HashTrieView {
                trie: HashTrie::Trivial {
                    value
                },
                node: None
            } => {
                Some(&value)
            },

            HashTrieView {
                trie: HashTrie::Standard { map, next_id: _ },
                node: Some(node_value)
            } => {
                if let Some(HashTrieNode::Leaf { value }) = map.get(node_value) {
                    Some(&value)
                } else {
                    None
                }
            },

            _ => None
        }
    }

    fn descend(&'a self, key: K) -> Option<Self> {
        match self {
            HashTrieView { 
                trie: HashTrie::Standard { map, next_id: _ }, 
                node: Some(node_value)
            } => {
                if let Some(HashTrieNode::Branch { id }) = map.get(node_value) {
                    Some(HashTrieView { 
                        trie: self.trie, 
                        node: Some((*id, key.clone()))
                    })
                } else {
                    None
                }
            },

            _ => None
        }
    }
}

pub struct HashTrieViewMut<'a, K, V> {
    trie: &'a mut HashTrie<K, V>,
    node: Option<(u32, K)>
}

impl<'a, K, V> TrieViewMut<'a, K, V> for HashTrieViewMut<'a, K, V> 
    where K: Eq + Hash + Clone {

    fn value(&'a mut self) -> Option<&'a mut V> {
        match self {
            HashTrieViewMut {
                trie: HashTrie::Trivial {
                    value
                },
                node: None
            } => {
                Some(value)
            },

            HashTrieViewMut {
                trie: HashTrie::Standard { map, next_id: _ },
                node: Some(node_value)
            } => {
                if let Some(HashTrieNode::Leaf { value }) = map.get_mut(node_value) {
                    Some(value)
                } else {
                    None
                }
            },

            _ => None
        }
    }

    fn descend(self, key: K) -> Option<Self> {
        let mut self_alias = self;
        let mut node;

        if let HashTrieViewMut { 
                trie: HashTrie::Standard { map, next_id: _ }, 
                node: Some(ref node_value)
            } = &mut self_alias {

            if let Some(HashTrieNode::Branch { id }) = map.get(&node_value) {
                node = (*id, key);
            } else {
                return None;
            }
        } else if let HashTrieViewMut { 
                trie: HashTrie::Standard { map: _, next_id: _ }, 
                node: None
            } = &mut self_alias {

            node = (0, key);
        } else {
            return None;
        }

        Some(HashTrieViewMut { 
            trie: self_alias.trie, 
            node: Some(node)
        })
    }

    fn descend_or_add(self, key: K) -> Option<Self> {
        let mut self_alias = self;
        let mut node;

        if let HashTrieViewMut { 
                trie: HashTrie::Standard { map, next_id }, 
                node: Some(ref node_value)
            } = &mut self_alias {

            if let Some(HashTrieNode::Branch { id }) = map.get(&node_value) {
                node = (*id, key);
            } else {
                node = (*next_id, key);
                map.insert(node_value.clone(), HashTrieNode::Branch { id: *next_id });
                *next_id += 1;
            }
        } else if let HashTrieViewMut { 
                trie: HashTrie::Standard { map: _, next_id: _ }, 
                node: None
            } = &mut self_alias {

            node = (0, key);
        } else {
            return None;
        }

        Some(HashTrieViewMut { 
            trie: self_alias.trie, 
            node: Some(node)
        })
    }
}

#[cfg(test)]
mod test {

}