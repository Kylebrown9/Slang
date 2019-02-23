use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

/// This whole file is implemented directly based on:
/// https://stackoverflow.com/questions/45786717/how-to-implement-hashmap-with-two-keys

pub trait KeyPair<A, B> {
    /// Obtains the first element of the pair.
    fn a(&self) -> &A;

    /// Obtains the second element of the pair.
    fn b(&self) -> &B;
}

#[derive(PartialEq, Eq, Hash)]
pub struct Pair<A, B>(pub A, pub B);

impl<A, B> KeyPair<A, B> for Pair<A, B>
    where
        A: Eq + Hash,
        B: Eq + Hash {

    fn a(&self) -> &A {
        &self.0
    }
    fn b(&self) -> &B {
        &self.1
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct HalfBorrowed<'b, A, B: 'b>(pub A, pub &'b B);

impl<'a, 'b, A, B> KeyPair<A, B> for HalfBorrowed<'b, A, B>
    where
        A: Eq + Hash + 'a,
        B: Eq + Hash + 'b {

    fn a(&self) -> &A {
        &self.0
    }
    fn b(&self) -> &B {
        self.1
    }
}

impl<'a, A, B> Borrow<KeyPair<A, B> + 'a> for Pair<A, B>
where
    A: Eq + Hash + 'a,
    B: Eq + Hash + 'a,
{
    fn borrow(&self) -> &(KeyPair<A, B> + 'a) {
        self
    }
}

impl<'a, A: Hash, B: Hash> Hash for (KeyPair<A, B> + 'a) {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a().hash(state);
        self.b().hash(state);
    }
}

impl<'a, A: Eq, B: Eq> PartialEq for (KeyPair<A, B> + 'a) {
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() && self.b() == other.b()
    }
}

impl<'a, A: Eq, B: Eq> Eq for (KeyPair<A, B> + 'a) {}