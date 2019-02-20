pub mod hash;

/// The Trie trait represents a read-only mapping from
/// a sequence of key elements to a single value.
/// This allows for get() map style behavior.
/// All Trie implementations must be prefix free.
pub trait Trie<K, V>: Sized {

    type View: TrieView<K, V>;

    fn as_view(self) -> Self::View;
    
    fn get<T>(self, path: T) -> Self::View
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

/// The TrieView trait represents a read-only view
/// of a Trie node.
/// In order to enforce prefix-free behavior a given
/// TrieView must never have a value and children
pub trait TrieView<K, V>: Sized {
    fn value(&self) -> Option<&V>;

    fn descend(&self, key: K) -> Option<Self>;
}

/// The TrieMut trait represents a mutable mapping from
/// a sequence of key elements to a single value.
/// This allows for both get() and insert() map style behavior
/// All TrieMut implementations must be prefix free.
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

        let success = view.set_value(new_val);

        success
    }
}

/// The TrieView trait represents a mutable view
/// of a Trie node.
/// In order to enforce prefix-free behavior a given
/// TrieViewMut must never have a value and children,
/// or allow a consumer to add a value or child to a node
/// when it would violate this rule
pub trait TrieViewMut<K, V>: Sized {
    fn value(&mut self) -> Option<&mut V>;
    
    fn set_value(&mut self, new_value: V) -> bool;

    fn descend(self, key: K) -> Option<Self>;
    
    fn descend_or_add(self, key: K) -> Option<Self>;
}
