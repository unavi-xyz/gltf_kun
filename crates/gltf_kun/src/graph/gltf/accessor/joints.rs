use std::marker::PhantomData;

use super::iter::ElementIter;

/// Vertex joints.
#[derive(Clone, Debug)]
pub enum ReadJoints<'a> {
    /// Joints of type `[u8; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U8(ElementIter<'a, [u8; 4]>),
    /// Joints of type `[u16; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U16(ElementIter<'a, [u16; 4]>),
}

impl<'a> ReadJoints<'a> {
    /// Reinterpret joints as u16, which can fit any possible joint.
    pub const fn into_u16(self) -> CastingIter<'a, U16> {
        CastingIter::new(self)
    }
}

/// Casting iterator for `Joints`.
#[derive(Clone, Debug)]
pub struct CastingIter<'a, T>(ReadJoints<'a>, PhantomData<T>);

/// Type which describes how to cast any joint into u16.
#[derive(Clone, Debug)]
pub struct U16;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Output;

    /// Cast from u8.
    fn cast_u8(x: [u8; 4]) -> Self::Output;

    /// Cast from u16.
    fn cast_u16(x: [u16; 4]) -> Self::Output;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) const fn new(iter: ReadJoints<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `Joints` object.
    pub const fn unwrap(self) -> ReadJoints<'a> {
        self.0
    }
}

impl<A: Cast> ExactSizeIterator for CastingIter<'_, A> {}
impl<A: Cast> Iterator for CastingIter<'_, A> {
    type Item = A::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            ReadJoints::U8(ref mut i) => i.next().map(A::cast_u8),
            ReadJoints::U16(ref mut i) => i.next().map(A::cast_u16),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            ReadJoints::U8(ref mut i) => i.nth(x).map(A::cast_u8),
            ReadJoints::U16(ref mut i) => i.nth(x).map(A::cast_u16),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            ReadJoints::U8(i) => i.last().map(A::cast_u8),
            ReadJoints::U16(i) => i.last().map(A::cast_u16),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            ReadJoints::U8(ref i) => i.size_hint(),
            ReadJoints::U16(ref i) => i.size_hint(),
        }
    }
}

impl Cast for U16 {
    type Output = [u16; 4];

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        [
            u16::from(x[0]),
            u16::from(x[1]),
            u16::from(x[2]),
            u16::from(x[3]),
        ]
    }

    fn cast_u16(x: [u16; 4]) -> Self::Output {
        x
    }
}
