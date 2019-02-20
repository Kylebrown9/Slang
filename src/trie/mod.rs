pub mod hash;

/// The Trie trait represents a mapping from
/// a sequence of key elements to a single value.
/// No key element sequence can be the prefix of any other
/// This must be enforced by implementors to disambiguate parsing
/// 
/// This mapping can be used similarly to HashMap
/// and provides get(&self) and insert(&mut self, T, V) support.
/// 
/// These map-like functionalities are implemented using
/// TrieView and TrieViewMut respectively.
/// An implementor of Trie must provide a method for constructing
/// these views from the Trie.
pub trait Trie<K, V>: Sized {

    type View: TrieView<K, V>;

    fn as_view(self) -> Self::View;
    
    fn get<T, Q>(self, path: T) -> Self::View
        where
            T: IntoIterator<Item=K> {

        let mut view = self.as_view();

        for key in path {
            view = match view.descend(key) {
                Some(view) => view,
                None => break
            };
        }

        view
    }
}

pub trait TrieMut<K, V>: Trie<K, V> {

    type ViewMut: TrieViewMut<K, V>;

    fn as_view_mut(self) -> Self::ViewMut;

    fn insert<T>(self, path: T, new_val: V) -> bool
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

        if let Some(val_ref) = view.value() {
            *val_ref = new_val;
            true
        } else {
            false
        }
    }
}

///
/// 
pub trait TrieViewMut<K, V>: Sized {
    fn value(&mut self) -> Option<&mut V>;

    fn descend(self, key: K) -> Option<Self>;
    
    fn descend_or_add(self, key: K) -> Option<Self>;
}

/// The TrieView trait represents a read only view
/// of a Trie node.
pub trait TrieView<K, V>: Sized {
    fn value(&self) -> Option<&V>;

    fn descend(&self, key: K) -> Option<Self>;
}
