// Regression test for <https://github.com/rust-lang/rust/issues/85451>.
// An outlives bound on a type wrapping an associated type, like
// `Option<<T as Trait>::Type>: 'a`, used to fail with E0309. The trait-impl case
// from that issue still errors and is not covered here.
//@ check-pass

#![allow(dead_code)]

trait Trait {
    type Type;
}

fn opt<'a, T, I>(_: I)
where
    T: Trait,
    I: IntoIterator<Item = &'a Option<<T as Trait>::Type>>,
    Option<<T as Trait>::Type>: 'a,
{
}

trait Container {
    type Item;
}

struct Wrapper<T>(T);

fn wrapper<'a, H, C>(_: C)
where
    H: Trait,
    C: Container<Item = &'a Wrapper<H::Type>>,
    Wrapper<H::Type>: 'a,
{
}

fn tuple<'a, H, C>(_: C)
where
    H: Trait,
    C: Container<Item = &'a (H::Type,)>,
    (H::Type,): 'a,
{
}

fn main() {}
