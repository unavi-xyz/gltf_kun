use std::marker::PhantomData;

use super::{iter::ElementIter, normalize::Normalize};

/// Weights.
#[derive(Clone, Debug)]
pub enum ReadWeights<'a> {
    /// Weights of type `[u8; 4]`.
    U8(ElementIter<'a, [u8; 4]>),
    /// Weights of type `[u16; 4]`.
    U16(ElementIter<'a, [u16; 4]>),
    /// Weights of type `[f32; 4]`.
    F32(ElementIter<'a, [f32; 4]>),
}

impl<'a> ReadWeights<'a> {
    /// Reinterpret weights as u8.  Lossy if the underlying iterator yields u16
    /// or f32.
    pub fn into_u8(self) -> CastingIter<'a, U8> {
        CastingIter::new(self)
    }

    /// Reinterpret weights as u16.  Lossy if the underlying iterator yields
    /// f32.
    pub fn into_u16(self) -> CastingIter<'a, U16> {
        CastingIter::new(self)
    }

    /// Reinterpret weights as f32.  Lossy if the underlying iterator yields
    /// u16.
    pub fn into_f32(self) -> CastingIter<'a, F32> {
        CastingIter::new(self)
    }
}

/// Casting iterator for `Weights`.
#[derive(Clone, Debug)]
pub struct CastingIter<'a, T>(ReadWeights<'a>, PhantomData<T>);

/// Type which describes how to cast any weight into u8.
#[derive(Clone, Debug)]
pub struct U8;

/// Type which describes how to cast any weight into u16.
#[derive(Clone, Debug)]
pub struct U16;

/// Type which describes how to cast any weight into f32.
#[derive(Clone, Debug)]
pub struct F32;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Output;

    /// Cast from u8.
    fn cast_u8(x: [u8; 4]) -> Self::Output;

    /// Cast from u16.
    fn cast_u16(x: [u16; 4]) -> Self::Output;

    /// Cast from f32.
    fn cast_f32(x: [f32; 4]) -> Self::Output;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: ReadWeights<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `Weights` object.
    pub fn unwrap(self) -> ReadWeights<'a> {
        self.0
    }
}

impl<A: Cast> ExactSizeIterator for CastingIter<'_, A> {}
impl<A: Cast> Iterator for CastingIter<'_, A> {
    type Item = A::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            ReadWeights::U8(ref mut i) => i.next().map(A::cast_u8),
            ReadWeights::U16(ref mut i) => i.next().map(A::cast_u16),
            ReadWeights::F32(ref mut i) => i.next().map(A::cast_f32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            ReadWeights::U8(ref mut i) => i.nth(x).map(A::cast_u8),
            ReadWeights::U16(ref mut i) => i.nth(x).map(A::cast_u16),
            ReadWeights::F32(ref mut i) => i.nth(x).map(A::cast_f32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            ReadWeights::U8(i) => i.last().map(A::cast_u8),
            ReadWeights::U16(i) => i.last().map(A::cast_u16),
            ReadWeights::F32(i) => i.last().map(A::cast_f32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            ReadWeights::U8(ref i) => i.size_hint(),
            ReadWeights::U16(ref i) => i.size_hint(),
            ReadWeights::F32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for U8 {
    type Output = [u8; 4];

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 4]) -> Self::Output {
        x.normalize()
    }
}

impl Cast for U16 {
    type Output = [u16; 4];

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 4]) -> Self::Output {
        x.normalize()
    }
}

impl Cast for F32 {
    type Output = [f32; 4];

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 4]) -> Self::Output {
        x.normalize()
    }
}
