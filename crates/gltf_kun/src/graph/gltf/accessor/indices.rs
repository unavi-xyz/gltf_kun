use std::marker::PhantomData;

use super::iter::ElementIter;

/// Index data.
#[derive(Clone, Debug)]
pub enum ReadIndices<'a> {
    /// Index data of type U8
    U8(ElementIter<'a, u8>),
    /// Index data of type U16
    U16(ElementIter<'a, u16>),
    /// Index data of type U32
    U32(ElementIter<'a, u32>),
}

impl<'a> ReadIndices<'a> {
    /// Reinterpret indices as u32, which can fit any possible index.
    pub fn into_u32(self) -> CastingIter<'a, U32> {
        CastingIter::new(self)
    }
}

/// Casting iterator for `Indices`.
#[derive(Clone, Debug)]
pub struct CastingIter<'a, T>(ReadIndices<'a>, PhantomData<T>);

/// Type which describes how to cast any index into u32.
#[derive(Clone, Debug)]
pub struct U32;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Output;

    /// Cast from u8.
    fn cast_u8(x: u8) -> Self::Output;

    /// Cast from u16.
    fn cast_u16(x: u16) -> Self::Output;

    /// Cast from u32.
    fn cast_u32(x: u32) -> Self::Output;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: ReadIndices<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `Indices` object.
    pub fn unwrap(self) -> ReadIndices<'a> {
        self.0
    }
}

impl<A: Cast> ExactSizeIterator for CastingIter<'_, A> {}
impl<A: Cast> Iterator for CastingIter<'_, A> {
    type Item = A::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            ReadIndices::U8(ref mut i) => i.next().map(A::cast_u8),
            ReadIndices::U16(ref mut i) => i.next().map(A::cast_u16),
            ReadIndices::U32(ref mut i) => i.next().map(A::cast_u32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            ReadIndices::U8(ref mut i) => i.nth(x).map(A::cast_u8),
            ReadIndices::U16(ref mut i) => i.nth(x).map(A::cast_u16),
            ReadIndices::U32(ref mut i) => i.nth(x).map(A::cast_u32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            ReadIndices::U8(i) => i.last().map(A::cast_u8),
            ReadIndices::U16(i) => i.last().map(A::cast_u16),
            ReadIndices::U32(i) => i.last().map(A::cast_u32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            ReadIndices::U8(ref i) => i.size_hint(),
            ReadIndices::U16(ref i) => i.size_hint(),
            ReadIndices::U32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for U32 {
    type Output = u32;

    fn cast_u8(x: u8) -> Self::Output {
        x as Self::Output
    }
    fn cast_u16(x: u16) -> Self::Output {
        x as Self::Output
    }
    fn cast_u32(x: u32) -> Self::Output {
        x
    }
}
