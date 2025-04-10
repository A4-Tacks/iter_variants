#![no_std]

pub use iter_variants_derive::IterVariants;
use iter_variants_derive::impl_iter_variants_tuple;

pub trait IterVariants {
    type IterVariantsInput;
    fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F);
}

impl IterVariants for bool {
    type IterVariantsInput = Self;
    fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F) {
        f(false);
        f(true);
    }
}

impl<U: IterVariants> IterVariants for Option<U>
where
    <U as IterVariants>::IterVariantsInput: IterVariants,
{
    type IterVariantsInput = Option<<U as IterVariants>::IterVariantsInput>;
    fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F) {
        f(None);
        U::iter_variants(|v| f(Some(v)));
    }
}

macro_rules! impl_iter_variants_for_primitives {
    ( $t:ty ) => {
        impl IterVariants for $t {
            type IterVariantsInput = Self;
            fn iter_variants<F: Fn(Self::IterVariantsInput)>(f: F) {
                for i in <$t>::MIN..=<$t>::MAX {
                    f(i);
                }
            }
        }
    };
}

impl_iter_variants_for_primitives!(u8);
impl_iter_variants_for_primitives!(u16);
impl_iter_variants_for_primitives!(u32);
impl_iter_variants_for_primitives!(u64);
impl_iter_variants_for_primitives!(u128);
impl_iter_variants_for_primitives!(usize);

impl_iter_variants_for_primitives!(i8);
impl_iter_variants_for_primitives!(i16);
impl_iter_variants_for_primitives!(i32);
impl_iter_variants_for_primitives!(i64);
impl_iter_variants_for_primitives!(i128);
impl_iter_variants_for_primitives!(isize);

impl_iter_variants_tuple!();

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::IterVariants;

    #[derive(IterVariants, Clone, Copy)]
    enum A {
        B, C
    }

    #[derive(IterVariants)]
    struct Foo(Option<()>, u32, bool, Option<A>);

    #[derive(IterVariants)]
    enum Bar {
        A(bool),
        B(Option<bool>, usize),
        C,
        D {
            x: i32,
            y: Option<A>,
        }
    }
}
