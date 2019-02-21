pub mod hash;

/// The Trie trait represents a read-only mapping from
/// a sequence of key elements to a single value.
/// This allows for get() map style behavior.
/// All Trie implementations must be prefix free.
pub trait Trie<K, V>: Sized {

    /// The read-only View type for this Trie
    type View: TrieView<K, V>;

    /// Creates a View of this Trie
    fn as_view(self) -> Self::View;
    
    /// Gets the View for the specified node if it exists
    fn get_view<T>(self, path: T) -> Option<Self::View>
        where
            T: IntoIterator<Item=K> {

        let mut view = self.as_view();

        for key in path {
            view = match view.descend(key) {
                Some(view) => view,
                None => { 
                    return None;
                }
            };
        }

        Some(view)
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

    /// If this view is of a Branch with a correspond child,
    /// return a view of the child. 
    /// Otherwise returns None.
    fn descend(&self, key: K) -> Option<Self>;
}

/// The TrieMut trait represents a mutable mapping from
/// a sequence of key elements to a single value.
/// This allows for both get() and insert() map style behavior
/// All TrieMut implementations must be prefix free.
pub trait TrieMut<K, V>: Trie<K, V> {

    /// The mutable view type for this TrieMut
    type ViewMut: TrieViewMut<K, V>;

    /// Creates a ViewMut of this Trie
    fn as_view_mut(self) -> Self::ViewMut;

    /// Returns true if the insert succeeded
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

    /// If this view is of a Leaf, returns a mutable reference to its value
    /// Otherwise returns None
    fn value(&mut self) -> Option<&mut V>;
    
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
