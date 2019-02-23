mod key_pair;

pub mod hash;

use std::borrow::Borrow;

pub trait HasView<'a, K, V>: Trie<K, V> {
    type View: 'a + TrieView<K, V>;

    fn as_view(&'a self) -> Self::View;
}

/// The Trie trait represents a read-only mapping from
/// a sequence of key elements to a single value.
/// This allows for get() map style behavior.
/// All Trie implementations must be prefix free.
pub trait Trie<K, V>: Sized {
    /// Gets the View for the specified node if it exists
    fn get_view<'a, I>(&'a self, path: I) -> Option<Self::View>
        where
            I: IntoIterator,
            I::Item: Borrow<K>,
            Self: HasView<'a, K, V> {

        let mut view = self.as_view();

        for key in path {
            view = match view.descend(key.borrow()) {
                Some(view) => view,
                None => { 
                    return None;
                }
            };
        }

        Some(view)
    }

    /// Gets the value for the specified node if it exists
    fn get<'a, I>(&'a self, path: I) -> Option<&'a V>
        where
            I: IntoIterator<Item = &'a K>,
            K: 'a,
            Self: HasView<'a, K, V> {
                
        let mut view = self.as_view();

        for key in path {
            view = match view.descend(key) {
                Some(view) => view,
                None => { 
                    return None;
                }
            };
        }

        view.into_value()
    }
}

/// The TrieView trait represents a read-only view
/// of a Trie node.
/// In order to enforce prefix-free behavior a given
/// TrieView must never have a value and children
pub trait TrieView<K, V>: Sized {

    /// If this view is of a Leaf, returns a reference to its value
    /// Otherwise returns None
    fn value(&self) -> Option<&V>;

    /// If this view is of a Leaf, returns a reference to its value
    /// Otherwise returns None
    fn into_value<'a>(self) -> Option<&'a V> where Self: 'a;

    /// If this view is of a Branch with a correspond child,
    /// return a view of the child. 
    /// Otherwise returns None.
    fn descend(&self, key: &K) -> Option<Self>;
}

pub trait HasViewMut<'a, K, V>: TrieMut<K, V> {
    type ViewMut: 'a + TrieViewMut<K, V>;

    /// Creates a ViewMut of this Trie
    fn as_view_mut(&'a mut self) -> Self::ViewMut;
}

/// The TrieMut trait represents a mutable mapping from
/// a sequence of key elements to a single value.
/// This allows for both get() and insert() map style behavior
/// All TrieMut implementations must be prefix free.
pub trait TrieMut<K, V>: Trie<K, V> {

    /// Returns true if the insert succeeded
    fn insert<'a, T>(&'a mut self, path: T, new_val: V) -> bool
        where 
            T: IntoIterator<Item=K>,
            Self: HasViewMut<'a, K, V> {

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

    /// If this view is of a Leaf, returns a mutable reference to its value
    /// Otherwise returns None
    fn value_mut(&mut self) -> Option<&mut V>;

    /// If this view is of a Leaf, returns a mutable reference to its value
    /// Otherwise returns None
    fn into_value_mut<'a>(self) -> Option<&'a mut V> where Self: 'a;
    
    /// If this view is of a Leaf, 
    /// overwrite its value with the new_value. (returns true)
    /// 
    /// If this view is a nonexistent child of a branch, 
    /// create it as a leaf with the new_value. (returns true)
    /// 
    /// Otherwise (returns false)
    fn set_value(&mut self, new_value: V) -> bool;

    /// If this view is of a Branch with a correspond child,
    /// return a view of the child. 
    /// Otherwise returns None.
    fn descend(self, key: K) -> Option<Self>;
    
    /// If this view is of a Branch with a correspond child,
    /// return a view of the child. 
    /// If this view is of a Branch without a corresponding child,
    /// create and return a view of it.
    /// Otherwise returns None.
    fn descend_or_add(self, key: K) -> Option<Self>;
}
