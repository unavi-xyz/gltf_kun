use std::marker::PhantomData;

use byteorder::{ByteOrder, LE};
use gltf::json::accessor::{ComponentType, Type};
use thiserror::Error;

#[derive(Copy, Clone, Debug)]
pub enum AccessorIter<'a> {
    F32(ElementIter<'a, f32>),
    F32x2(ElementIter<'a, [f32; 2]>),
    F32x3(ElementIter<'a, [f32; 3]>),
    F32x4(ElementIter<'a, [f32; 4]>),
    F32x16(ElementIter<'a, [f32; 16]>),
    U32(ElementIter<'a, u32>),
    U32x2(ElementIter<'a, [u32; 2]>),
    U32x3(ElementIter<'a, [u32; 3]>),
    U32x4(ElementIter<'a, [u32; 4]>),
    U16(ElementIter<'a, u16>),
    U16x2(ElementIter<'a, [u16; 2]>),
    U16x3(ElementIter<'a, [u16; 3]>),
    U16x4(ElementIter<'a, [u16; 4]>),
    U8(ElementIter<'a, u8>),
    U8x2(ElementIter<'a, [u8; 2]>),
    U8x3(ElementIter<'a, [u8; 3]>),
    U8x4(ElementIter<'a, [u8; 4]>),
    I16(ElementIter<'a, i16>),
    I16x2(ElementIter<'a, [i16; 2]>),
    I16x3(ElementIter<'a, [i16; 3]>),
    I16x4(ElementIter<'a, [i16; 4]>),
    I8(ElementIter<'a, i8>),
    I8x2(ElementIter<'a, [i8; 2]>),
    I8x3(ElementIter<'a, [i8; 3]>),
    I8x4(ElementIter<'a, [i8; 4]>),
}

#[derive(Debug, Error)]
pub enum AccessorIterCreateError {
    #[error("unsupported accessor type {0:?} {1:?}")]
    UnsupportedType(ComponentType, Type),
}

impl<'a> AccessorIter<'a> {
    #[allow(clippy::too_many_lines)]
    pub const fn new(
        slice: &'a [u8],
        component_type: ComponentType,
        element_type: Type,
        normalized: bool,
    ) -> Result<Self, AccessorIterCreateError> {
        match (component_type, element_type) {
            (ComponentType::F32, Type::Scalar) => Ok(AccessorIter::F32(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::F32, Type::Vec2) => Ok(AccessorIter::F32x2(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::F32, Type::Vec3) => Ok(AccessorIter::F32x3(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::F32, Type::Vec4) => Ok(AccessorIter::F32x4(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::F32, Type::Mat4) => Ok(AccessorIter::F32x16(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U32, Type::Scalar) => Ok(AccessorIter::U32(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U32, Type::Vec2) => Ok(AccessorIter::U32x2(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U32, Type::Vec3) => Ok(AccessorIter::U32x3(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U32, Type::Vec4) => Ok(AccessorIter::U32x4(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U16, Type::Scalar) => Ok(AccessorIter::U16(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U16, Type::Vec2) => Ok(AccessorIter::U16x2(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U16, Type::Vec3) => Ok(AccessorIter::U16x3(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U16, Type::Vec4) => Ok(AccessorIter::U16x4(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U8, Type::Scalar) => Ok(AccessorIter::U8(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U8, Type::Vec2) => Ok(AccessorIter::U8x2(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U8, Type::Vec3) => Ok(AccessorIter::U8x3(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::U8, Type::Vec4) => Ok(AccessorIter::U8x4(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::I16, Type::Scalar) => Ok(AccessorIter::I16(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::I16, Type::Vec2) => Ok(AccessorIter::I16x2(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::I16, Type::Vec3) => Ok(AccessorIter::I16x3(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::I16, Type::Vec4) => Ok(AccessorIter::I16x4(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::I8, Type::Scalar) => Ok(AccessorIter::I8(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::I8, Type::Vec2) => Ok(AccessorIter::I8x2(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::I8, Type::Vec3) => Ok(AccessorIter::I8x3(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (ComponentType::I8, Type::Vec4) => Ok(AccessorIter::I8x4(ElementIter {
                slice,
                normalized,
                _phantom: PhantomData,
            })),
            (component_type, element_type) => Err(AccessorIterCreateError::UnsupportedType(
                component_type,
                element_type,
            )),
        }
    }

    pub const fn component_type(&self) -> ComponentType {
        match self {
            AccessorIter::F32(_)
            | AccessorIter::F32x2(_)
            | AccessorIter::F32x3(_)
            | AccessorIter::F32x4(_)
            | AccessorIter::F32x16(_) => ComponentType::F32,
            AccessorIter::U32(_)
            | AccessorIter::U32x2(_)
            | AccessorIter::U32x3(_)
            | AccessorIter::U32x4(_) => ComponentType::U32,
            AccessorIter::U16(_)
            | AccessorIter::U16x2(_)
            | AccessorIter::U16x3(_)
            | AccessorIter::U16x4(_) => ComponentType::U16,
            AccessorIter::U8(_)
            | AccessorIter::U8x2(_)
            | AccessorIter::U8x3(_)
            | AccessorIter::U8x4(_) => ComponentType::U8,
            AccessorIter::I16(_)
            | AccessorIter::I16x2(_)
            | AccessorIter::I16x3(_)
            | AccessorIter::I16x4(_) => ComponentType::I16,
            AccessorIter::I8(_)
            | AccessorIter::I8x2(_)
            | AccessorIter::I8x3(_)
            | AccessorIter::I8x4(_) => ComponentType::I8,
        }
    }

    pub const fn element_type(&self) -> Type {
        match self {
            AccessorIter::F32(_)
            | AccessorIter::U32(_)
            | AccessorIter::U16(_)
            | AccessorIter::U8(_)
            | AccessorIter::I16(_)
            | AccessorIter::I8(_) => Type::Scalar,
            AccessorIter::F32x2(_)
            | AccessorIter::U32x2(_)
            | AccessorIter::U16x2(_)
            | AccessorIter::U8x2(_)
            | AccessorIter::I16x2(_)
            | AccessorIter::I8x2(_) => Type::Vec2,
            AccessorIter::F32x3(_)
            | AccessorIter::U32x3(_)
            | AccessorIter::U16x3(_)
            | AccessorIter::U8x3(_)
            | AccessorIter::I16x3(_)
            | AccessorIter::I8x3(_) => Type::Vec3,
            AccessorIter::F32x4(_)
            | AccessorIter::U32x4(_)
            | AccessorIter::U16x4(_)
            | AccessorIter::U8x4(_)
            | AccessorIter::I16x4(_)
            | AccessorIter::I8x4(_) => Type::Vec4,
            AccessorIter::F32x16(_) => Type::Mat4,
        }
    }

    pub const fn normalized(&self) -> bool {
        match self {
            AccessorIter::F32(iter) => iter.normalized,
            AccessorIter::F32x2(iter) => iter.normalized,
            AccessorIter::F32x3(iter) => iter.normalized,
            AccessorIter::F32x4(iter) => iter.normalized,
            AccessorIter::F32x16(iter) => iter.normalized,
            AccessorIter::U32(iter) => iter.normalized,
            AccessorIter::U32x2(iter) => iter.normalized,
            AccessorIter::U32x3(iter) => iter.normalized,
            AccessorIter::U32x4(iter) => iter.normalized,
            AccessorIter::U16(iter) => iter.normalized,
            AccessorIter::U16x2(iter) => iter.normalized,
            AccessorIter::U16x3(iter) => iter.normalized,
            AccessorIter::U16x4(iter) => iter.normalized,
            AccessorIter::U8(iter) => iter.normalized,
            AccessorIter::U8x2(iter) => iter.normalized,
            AccessorIter::U8x3(iter) => iter.normalized,
            AccessorIter::U8x4(iter) => iter.normalized,
            AccessorIter::I16(iter) => iter.normalized,
            AccessorIter::I16x2(iter) => iter.normalized,
            AccessorIter::I16x3(iter) => iter.normalized,
            AccessorIter::I16x4(iter) => iter.normalized,
            AccessorIter::I8(iter) => iter.normalized,
            AccessorIter::I8x2(iter) => iter.normalized,
            AccessorIter::I8x3(iter) => iter.normalized,
            AccessorIter::I8x4(iter) => iter.normalized,
        }
    }

    pub const fn slice(&self) -> &[u8] {
        match self {
            AccessorIter::F32(iter) => iter.slice,
            AccessorIter::F32x2(iter) => iter.slice,
            AccessorIter::F32x3(iter) => iter.slice,
            AccessorIter::F32x4(iter) => iter.slice,
            AccessorIter::F32x16(iter) => iter.slice,
            AccessorIter::U32(iter) => iter.slice,
            AccessorIter::U32x2(iter) => iter.slice,
            AccessorIter::U32x3(iter) => iter.slice,
            AccessorIter::U32x4(iter) => iter.slice,
            AccessorIter::U16(iter) => iter.slice,
            AccessorIter::U16x2(iter) => iter.slice,
            AccessorIter::U16x3(iter) => iter.slice,
            AccessorIter::U16x4(iter) => iter.slice,
            AccessorIter::U8(iter) => iter.slice,
            AccessorIter::U8x2(iter) => iter.slice,
            AccessorIter::U8x3(iter) => iter.slice,
            AccessorIter::U8x4(iter) => iter.slice,
            AccessorIter::I16(iter) => iter.slice,
            AccessorIter::I16x2(iter) => iter.slice,
            AccessorIter::I16x3(iter) => iter.slice,
            AccessorIter::I16x4(iter) => iter.slice,
            AccessorIter::I8(iter) => iter.slice,
            AccessorIter::I8x2(iter) => iter.slice,
            AccessorIter::I8x3(iter) => iter.slice,
            AccessorIter::I8x4(iter) => iter.slice,
        }
    }

    pub fn count(&self) -> usize {
        match self {
            AccessorIter::F32(iter) => iter.count(),
            AccessorIter::F32x2(iter) => iter.count(),
            AccessorIter::F32x3(iter) => iter.count(),
            AccessorIter::F32x4(iter) => iter.count(),
            AccessorIter::F32x16(iter) => iter.count(),
            AccessorIter::U32(iter) => iter.count(),
            AccessorIter::U32x2(iter) => iter.count(),
            AccessorIter::U32x3(iter) => iter.count(),
            AccessorIter::U32x4(iter) => iter.count(),
            AccessorIter::U16(iter) => iter.count(),
            AccessorIter::U16x2(iter) => iter.count(),
            AccessorIter::U16x3(iter) => iter.count(),
            AccessorIter::U16x4(iter) => iter.count(),
            AccessorIter::U8(iter) => iter.count(),
            AccessorIter::U8x2(iter) => iter.count(),
            AccessorIter::U8x3(iter) => iter.count(),
            AccessorIter::U8x4(iter) => iter.count(),
            AccessorIter::I16(iter) => iter.count(),
            AccessorIter::I16x2(iter) => iter.count(),
            AccessorIter::I16x3(iter) => iter.count(),
            AccessorIter::I16x4(iter) => iter.count(),
            AccessorIter::I8(iter) => iter.count(),
            AccessorIter::I8x2(iter) => iter.count(),
            AccessorIter::I8x3(iter) => iter.count(),
            AccessorIter::I8x4(iter) => iter.count(),
        }
    }

    pub fn max(&self) -> AccessorElement {
        match self {
            AccessorIter::F32(iter) => iter.gl_max().into(),
            AccessorIter::F32x2(iter) => iter.gl_max().into(),
            AccessorIter::F32x3(iter) => iter.gl_max().into(),
            AccessorIter::F32x4(iter) => iter.gl_max().into(),
            AccessorIter::F32x16(iter) => iter.gl_max().into(),
            AccessorIter::U32(iter) => iter.gl_max().into(),
            AccessorIter::U32x2(iter) => iter.gl_max().into(),
            AccessorIter::U32x3(iter) => iter.gl_max().into(),
            AccessorIter::U32x4(iter) => iter.gl_max().into(),
            AccessorIter::U16(iter) => iter.gl_max().into(),
            AccessorIter::U16x2(iter) => iter.gl_max().into(),
            AccessorIter::U16x3(iter) => iter.gl_max().into(),
            AccessorIter::U16x4(iter) => iter.gl_max().into(),
            AccessorIter::U8(iter) => iter.gl_max().into(),
            AccessorIter::U8x2(iter) => iter.gl_max().into(),
            AccessorIter::U8x3(iter) => iter.gl_max().into(),
            AccessorIter::U8x4(iter) => iter.gl_max().into(),
            AccessorIter::I16(iter) => iter.gl_max().into(),
            AccessorIter::I16x2(iter) => iter.gl_max().into(),
            AccessorIter::I16x3(iter) => iter.gl_max().into(),
            AccessorIter::I16x4(iter) => iter.gl_max().into(),
            AccessorIter::I8(iter) => iter.gl_max().into(),
            AccessorIter::I8x2(iter) => iter.gl_max().into(),
            AccessorIter::I8x3(iter) => iter.gl_max().into(),
            AccessorIter::I8x4(iter) => iter.gl_max().into(),
        }
    }

    pub fn min(&self) -> AccessorElement {
        match self {
            AccessorIter::F32(iter) => iter.gl_min().into(),
            AccessorIter::F32x2(iter) => iter.gl_min().into(),
            AccessorIter::F32x3(iter) => iter.gl_min().into(),
            AccessorIter::F32x4(iter) => iter.gl_min().into(),
            AccessorIter::F32x16(iter) => iter.gl_min().into(),
            AccessorIter::U32(iter) => iter.gl_min().into(),
            AccessorIter::U32x2(iter) => iter.gl_min().into(),
            AccessorIter::U32x3(iter) => iter.gl_min().into(),
            AccessorIter::U32x4(iter) => iter.gl_min().into(),
            AccessorIter::U16(iter) => iter.gl_min().into(),
            AccessorIter::U16x2(iter) => iter.gl_min().into(),
            AccessorIter::U16x3(iter) => iter.gl_min().into(),
            AccessorIter::U16x4(iter) => iter.gl_min().into(),
            AccessorIter::U8(iter) => iter.gl_min().into(),
            AccessorIter::U8x2(iter) => iter.gl_min().into(),
            AccessorIter::U8x3(iter) => iter.gl_min().into(),
            AccessorIter::U8x4(iter) => iter.gl_min().into(),
            AccessorIter::I16(iter) => iter.gl_min().into(),
            AccessorIter::I16x2(iter) => iter.gl_min().into(),
            AccessorIter::I16x3(iter) => iter.gl_min().into(),
            AccessorIter::I16x4(iter) => iter.gl_min().into(),
            AccessorIter::I8(iter) => iter.gl_min().into(),
            AccessorIter::I8x2(iter) => iter.gl_min().into(),
            AccessorIter::I8x3(iter) => iter.gl_min().into(),
            AccessorIter::I8x4(iter) => iter.gl_min().into(),
        }
    }
}

pub enum AccessorElement {
    F32(f32),
    F32x2([f32; 2]),
    F32x3([f32; 3]),
    F32x4([f32; 4]),
    F32x16([f32; 16]),
    U32(u32),
    U32x2([u32; 2]),
    U32x3([u32; 3]),
    U32x4([u32; 4]),
    U16(u16),
    U16x2([u16; 2]),
    U16x3([u16; 3]),
    U16x4([u16; 4]),
    U8(u8),
    U8x2([u8; 2]),
    U8x3([u8; 3]),
    U8x4([u8; 4]),
    I16(i16),
    I16x2([i16; 2]),
    I16x3([i16; 3]),
    I16x4([i16; 4]),
    I8(i8),
    I8x2([i8; 2]),
    I8x3([i8; 3]),
    I8x4([i8; 4]),
}

impl From<f32> for AccessorElement {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}
impl From<[f32; 2]> for AccessorElement {
    fn from(value: [f32; 2]) -> Self {
        Self::F32x2(value)
    }
}
impl From<[f32; 3]> for AccessorElement {
    fn from(value: [f32; 3]) -> Self {
        Self::F32x3(value)
    }
}
impl From<[f32; 4]> for AccessorElement {
    fn from(value: [f32; 4]) -> Self {
        Self::F32x4(value)
    }
}
impl From<[f32; 16]> for AccessorElement {
    fn from(value: [f32; 16]) -> Self {
        Self::F32x16(value)
    }
}
impl From<u32> for AccessorElement {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}
impl From<[u32; 2]> for AccessorElement {
    fn from(value: [u32; 2]) -> Self {
        Self::U32x2(value)
    }
}
impl From<[u32; 3]> for AccessorElement {
    fn from(value: [u32; 3]) -> Self {
        Self::U32x3(value)
    }
}
impl From<[u32; 4]> for AccessorElement {
    fn from(value: [u32; 4]) -> Self {
        Self::U32x4(value)
    }
}
impl From<u16> for AccessorElement {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}
impl From<[u16; 2]> for AccessorElement {
    fn from(value: [u16; 2]) -> Self {
        Self::U16x2(value)
    }
}
impl From<[u16; 3]> for AccessorElement {
    fn from(value: [u16; 3]) -> Self {
        Self::U16x3(value)
    }
}
impl From<[u16; 4]> for AccessorElement {
    fn from(value: [u16; 4]) -> Self {
        Self::U16x4(value)
    }
}
impl From<u8> for AccessorElement {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}
impl From<[u8; 2]> for AccessorElement {
    fn from(value: [u8; 2]) -> Self {
        Self::U8x2(value)
    }
}
impl From<[u8; 3]> for AccessorElement {
    fn from(value: [u8; 3]) -> Self {
        Self::U8x3(value)
    }
}
impl From<[u8; 4]> for AccessorElement {
    fn from(value: [u8; 4]) -> Self {
        Self::U8x4(value)
    }
}
impl From<i16> for AccessorElement {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}
impl From<[i16; 2]> for AccessorElement {
    fn from(value: [i16; 2]) -> Self {
        Self::I16x2(value)
    }
}
impl From<[i16; 3]> for AccessorElement {
    fn from(value: [i16; 3]) -> Self {
        Self::I16x3(value)
    }
}
impl From<[i16; 4]> for AccessorElement {
    fn from(value: [i16; 4]) -> Self {
        Self::I16x4(value)
    }
}
impl From<i8> for AccessorElement {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}
impl From<[i8; 2]> for AccessorElement {
    fn from(value: [i8; 2]) -> Self {
        Self::I8x2(value)
    }
}
impl From<[i8; 3]> for AccessorElement {
    fn from(value: [i8; 3]) -> Self {
        Self::I8x3(value)
    }
}
impl From<[i8; 4]> for AccessorElement {
    fn from(value: [i8; 4]) -> Self {
        Self::I8x4(value)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ElementIter<'a, T: Element> {
    pub normalized: bool,
    pub slice: &'a [u8],
    pub _phantom: PhantomData<T>,
}

impl<T: Element> Iterator for ElementIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }

        let stride = T::stride();
        let (head, tail) = self.slice.split_at(stride);
        self.slice = tail;

        Some(T::from_slice(head))
    }
}

impl<T: Element + Copy> ElementIter<'_, T> {
    pub fn count(&self) -> usize {
        self.slice.len() / T::stride()
    }

    pub fn gl_max(&self) -> T {
        let mut max = T::zero();
        for element in *self {
            max = max.gl_max(&element);
        }
        max
    }

    pub fn gl_min(&self) -> T {
        let mut min = T::zero();
        for element in *self {
            min = min.gl_min(&element);
        }
        min
    }
}

pub trait Element {
    fn stride() -> usize {
        Self::element_type().multiplicity() * Self::component_type().size()
    }

    fn component_type() -> ComponentType;
    fn element_type() -> Type;
    fn from_slice(slice: &[u8]) -> Self;
    fn zero() -> Self;

    #[must_use]
    fn gl_max(&self, other: &Self) -> Self;
    #[must_use]
    fn gl_min(&self, other: &Self) -> Self;
}

impl Element for f32 {
    fn component_type() -> ComponentType {
        ComponentType::F32
    }
    fn element_type() -> Type {
        Type::Scalar
    }
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_f32(slice)
    }
    fn zero() -> Self {
        0.0
    }
    fn gl_max(&self, other: &Self) -> Self {
        self.max(*other)
    }
    fn gl_min(&self, other: &Self) -> Self {
        self.min(*other)
    }
}

impl Element for u32 {
    fn component_type() -> ComponentType {
        ComponentType::U32
    }
    fn element_type() -> Type {
        Type::Scalar
    }
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_u32(slice)
    }
    fn zero() -> Self {
        0
    }
    fn gl_max(&self, other: &Self) -> Self {
        *self.max(other)
    }
    fn gl_min(&self, other: &Self) -> Self {
        *self.min(other)
    }
}

impl Element for u16 {
    fn component_type() -> ComponentType {
        ComponentType::U16
    }
    fn element_type() -> Type {
        Type::Scalar
    }
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_u16(slice)
    }
    fn zero() -> Self {
        0
    }
    fn gl_max(&self, other: &Self) -> Self {
        *self.max(other)
    }
    fn gl_min(&self, other: &Self) -> Self {
        *self.min(other)
    }
}

impl Element for u8 {
    fn component_type() -> ComponentType {
        ComponentType::U8
    }
    fn element_type() -> Type {
        Type::Scalar
    }
    fn from_slice(slice: &[u8]) -> Self {
        slice[0]
    }
    fn zero() -> Self {
        0
    }
    fn gl_max(&self, other: &Self) -> Self {
        *self.max(other)
    }
    fn gl_min(&self, other: &Self) -> Self {
        *self.min(other)
    }
}

impl Element for i16 {
    fn component_type() -> ComponentType {
        ComponentType::I16
    }
    fn element_type() -> Type {
        Type::Scalar
    }
    fn from_slice(slice: &[u8]) -> Self {
        LE::read_i16(slice)
    }
    fn zero() -> Self {
        0
    }
    fn gl_max(&self, other: &Self) -> Self {
        *self.max(other)
    }
    fn gl_min(&self, other: &Self) -> Self {
        *self.min(other)
    }
}

impl Element for i8 {
    fn component_type() -> ComponentType {
        ComponentType::I8
    }
    fn element_type() -> Type {
        Type::Scalar
    }
    fn from_slice(slice: &[u8]) -> Self {
        slice[0] as Self
    }
    fn zero() -> Self {
        0
    }
    fn gl_max(&self, other: &Self) -> Self {
        *self.max(other)
    }
    fn gl_min(&self, other: &Self) -> Self {
        *self.min(other)
    }
}

impl<T: Element + Copy> Element for [T; 2] {
    fn component_type() -> ComponentType {
        T::component_type()
    }

    fn element_type() -> Type {
        Type::Vec2
    }
    fn from_slice(slice: &[u8]) -> Self {
        [
            T::from_slice(slice),
            T::from_slice(&slice[std::mem::size_of::<T>()..]),
        ]
    }
    fn zero() -> Self {
        [T::zero(); 2]
    }
    fn gl_max(&self, other: &Self) -> Self {
        [self[0].gl_max(&other[0]), self[1].gl_max(&other[1])]
    }
    fn gl_min(&self, other: &Self) -> Self {
        [self[0].gl_min(&other[0]), self[1].gl_min(&other[1])]
    }
}

impl<T: Element + Copy> Element for [T; 3] {
    fn component_type() -> ComponentType {
        T::component_type()
    }

    fn element_type() -> Type {
        Type::Vec3
    }
    fn from_slice(slice: &[u8]) -> Self {
        [
            T::from_slice(slice),
            T::from_slice(&slice[std::mem::size_of::<T>()..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 2..]),
        ]
    }
    fn zero() -> Self {
        [T::zero(); 3]
    }
    fn gl_max(&self, other: &Self) -> Self {
        [
            self[0].gl_max(&other[0]),
            self[1].gl_max(&other[1]),
            self[2].gl_max(&other[2]),
        ]
    }
    fn gl_min(&self, other: &Self) -> Self {
        [
            self[0].gl_min(&other[0]),
            self[1].gl_min(&other[1]),
            self[2].gl_min(&other[2]),
        ]
    }
}

impl<T: Element + Copy> Element for [T; 4] {
    fn component_type() -> ComponentType {
        T::component_type()
    }

    fn element_type() -> Type {
        Type::Vec4
    }
    fn from_slice(slice: &[u8]) -> Self {
        [
            T::from_slice(slice),
            T::from_slice(&slice[std::mem::size_of::<T>()..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 2..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 3..]),
        ]
    }
    fn zero() -> Self {
        [T::zero(); 4]
    }
    fn gl_max(&self, other: &Self) -> Self {
        [
            self[0].gl_max(&other[0]),
            self[1].gl_max(&other[1]),
            self[2].gl_max(&other[2]),
            self[3].gl_max(&other[3]),
        ]
    }
    fn gl_min(&self, other: &Self) -> Self {
        [
            self[0].gl_min(&other[0]),
            self[1].gl_min(&other[1]),
            self[2].gl_min(&other[2]),
            self[3].gl_min(&other[3]),
        ]
    }
}

impl<T: Element + Copy> Element for [T; 16] {
    fn component_type() -> ComponentType {
        T::component_type()
    }

    fn element_type() -> Type {
        Type::Mat4
    }
    fn from_slice(slice: &[u8]) -> Self {
        [
            T::from_slice(slice),
            T::from_slice(&slice[std::mem::size_of::<T>()..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 2..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 3..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 4..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 5..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 6..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 7..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 8..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 9..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 10..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 11..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 12..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 13..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 14..]),
            T::from_slice(&slice[std::mem::size_of::<T>() * 15..]),
        ]
    }
    fn zero() -> Self {
        [T::zero(); 16]
    }
    fn gl_max(&self, other: &Self) -> Self {
        [
            self[0].gl_max(&other[0]),
            self[1].gl_max(&other[1]),
            self[2].gl_max(&other[2]),
            self[3].gl_max(&other[3]),
            self[4].gl_max(&other[4]),
            self[5].gl_max(&other[5]),
            self[6].gl_max(&other[6]),
            self[7].gl_max(&other[7]),
            self[8].gl_max(&other[8]),
            self[9].gl_max(&other[9]),
            self[10].gl_max(&other[10]),
            self[11].gl_max(&other[11]),
            self[12].gl_max(&other[12]),
            self[13].gl_max(&other[13]),
            self[14].gl_max(&other[14]),
            self[15].gl_max(&other[15]),
        ]
    }
    fn gl_min(&self, other: &Self) -> Self {
        [
            self[0].gl_min(&other[0]),
            self[1].gl_min(&other[1]),
            self[2].gl_min(&other[2]),
            self[3].gl_min(&other[3]),
            self[4].gl_min(&other[4]),
            self[5].gl_min(&other[5]),
            self[6].gl_min(&other[6]),
            self[7].gl_min(&other[7]),
            self[8].gl_min(&other[8]),
            self[9].gl_min(&other[9]),
            self[10].gl_min(&other[10]),
            self[11].gl_min(&other[11]),
            self[12].gl_min(&other[12]),
            self[13].gl_min(&other[13]),
            self[14].gl_min(&other[14]),
            self[15].gl_min(&other[15]),
        ]
    }
}
