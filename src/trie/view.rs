pub trait TrieViewable<'a, K, V> {
    type View: TrieView<'a, K, V>;

    fn as_view(&self) -> Self::View;
}

pub trait TrieView<'a, K, V>: Sized {
    fn value(&self) -> Option<&V>;

    fn get(&self, key: K) -> Option<Self>;
}