pub mod hash;

/**
 * The Trie trait represents a mapping from
 * a sequence of key elements to a single value.
 * 
 * This mapping can be used similarly to HashMap
 * and provides get(&self) and insert(&mut self, T, V) support.
 * 
 * No key element sequence can be the prefix of any other
 * This must be enforced by implementors to disambiguate parsing
 */
pub trait Trie<'a, K, V>
    where
        K: 'a,
        V: 'a {

    type ViewMut: TrieViewMut<'a, K, V> + 'a;
    type View: TrieView<'a, K, V> + 'a;

    fn new() -> Self;

    fn as_view(&'a self) -> Self::View;
    
    fn as_view_mut(&'a mut self) -> Self::ViewMut;

    fn insert<T>(&'a mut self, path: T, new_val: V) -> bool
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
    
    fn get<T, Q>(&'a self, path: T) -> Option<&V>
        where
            T: IntoIterator<Item=&'a K> {

        let mut view = self.as_view();

        for key in path {
            let maybe_next = view.descend(key);

            if let Some(next_view) = maybe_next {
                view = next_view;
            } else {
                return None;
            }
        }

        view.value()
    }
}

/**
 * 
 */
pub trait TrieViewMut<'a, K, V>: Sized {
    fn value(&'a mut self) -> Option<&'a mut V>;

    fn descend(self, key: &K) -> Option<Self>;
    
    fn descend_or_add(self, key: K) -> Option<Self>;
}

/**
 * The TrieView trait represents a read only view
 * of a Trie node. 
 */
pub trait TrieView<'a, K, V>: Sized {
    fn value(&'a self) -> Option<&'a V>;

    fn descend(&'a self, key: &K) -> Option<Self>;
}
