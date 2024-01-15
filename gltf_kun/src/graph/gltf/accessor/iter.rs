use std::marker::PhantomData;

use byteorder::{ByteOrder, LE};
use gltf::json::accessor::{ComponentType, Type};
use thiserror::Error;

#[derive(Copy, Clone)]
pub enum AccessorIter<'a> {
    F32(ElementIter<'a, f32>),
    F32x2(ElementIter<'a, [f32; 2]>),
    F32x3(ElementIter<'a, [f32; 3]>),
    F32x4(ElementIter<'a, [f32; 4]>),
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
    #[error("Unsupported accessor type {0:?} {1:?}")]
    UnsupportedType(ComponentType, Type),
}

impl<'a> AccessorIter<'a> {
    pub fn new(
        slice: &'a [u8],
        component_type: ComponentType,
        element_type: Type,
    ) -> Result<Self, AccessorIterCreateError> {
        match (component_type, element_type) {
            (ComponentType::F32, Type::Scalar) => Ok(AccessorIter::F32(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::F32, Type::Vec2) => Ok(AccessorIter::F32x2(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::F32, Type::Vec3) => Ok(AccessorIter::F32x3(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::F32, Type::Vec4) => Ok(AccessorIter::F32x4(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U32, Type::Scalar) => Ok(AccessorIter::U32(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U32, Type::Vec2) => Ok(AccessorIter::U32x2(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U32, Type::Vec3) => Ok(AccessorIter::U32x3(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U32, Type::Vec4) => Ok(AccessorIter::U32x4(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U16, Type::Scalar) => Ok(AccessorIter::U16(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U16, Type::Vec2) => Ok(AccessorIter::U16x2(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U16, Type::Vec3) => Ok(AccessorIter::U16x3(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U16, Type::Vec4) => Ok(AccessorIter::U16x4(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U8, Type::Scalar) => Ok(AccessorIter::U8(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U8, Type::Vec2) => Ok(AccessorIter::U8x2(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U8, Type::Vec3) => Ok(AccessorIter::U8x3(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::U8, Type::Vec4) => Ok(AccessorIter::U8x4(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::I16, Type::Scalar) => Ok(AccessorIter::I16(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::I16, Type::Vec2) => Ok(AccessorIter::I16x2(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::I16, Type::Vec3) => Ok(AccessorIter::I16x3(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::I16, Type::Vec4) => Ok(AccessorIter::I16x4(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::I8, Type::Scalar) => Ok(AccessorIter::I8(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::I8, Type::Vec2) => Ok(AccessorIter::I8x2(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::I8, Type::Vec3) => Ok(AccessorIter::I8x3(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (ComponentType::I8, Type::Vec4) => Ok(AccessorIter::I8x4(ElementIter {
                slice,
                normalized: false,
                _phantom: PhantomData,
            })),
            (component_type, element_type) => Err(AccessorIterCreateError::UnsupportedType(
                component_type,
                element_type,
            )),
        }
    }

    pub fn component_type(&self) -> ComponentType {
        match self {
            AccessorIter::F32(_) => ComponentType::F32,
            AccessorIter::F32x2(_) => ComponentType::F32,
            AccessorIter::F32x3(_) => ComponentType::F32,
            AccessorIter::F32x4(_) => ComponentType::F32,
            AccessorIter::U32(_) => ComponentType::U32,
            AccessorIter::U32x2(_) => ComponentType::U32,
            AccessorIter::U32x3(_) => ComponentType::U32,
            AccessorIter::U32x4(_) => ComponentType::U32,
            AccessorIter::U16(_) => ComponentType::U16,
            AccessorIter::U16x2(_) => ComponentType::U16,
            AccessorIter::U16x3(_) => ComponentType::U16,
            AccessorIter::U16x4(_) => ComponentType::U16,
            AccessorIter::U8(_) => ComponentType::U8,
            AccessorIter::U8x2(_) => ComponentType::U8,
            AccessorIter::U8x3(_) => ComponentType::U8,
            AccessorIter::U8x4(_) => ComponentType::U8,
            AccessorIter::I16(_) => ComponentType::I16,
            AccessorIter::I16x2(_) => ComponentType::I16,
            AccessorIter::I16x3(_) => ComponentType::I16,
            AccessorIter::I16x4(_) => ComponentType::I16,
            AccessorIter::I8(_) => ComponentType::I8,
            AccessorIter::I8x2(_) => ComponentType::I8,
            AccessorIter::I8x3(_) => ComponentType::I8,
            AccessorIter::I8x4(_) => ComponentType::I8,
        }
    }

    pub fn element_type(&self) -> Type {
        match self {
            AccessorIter::F32(_) => Type::Scalar,
            AccessorIter::F32x2(_) => Type::Vec2,
            AccessorIter::F32x3(_) => Type::Vec3,
            AccessorIter::F32x4(_) => Type::Vec4,
            AccessorIter::U32(_) => Type::Scalar,
            AccessorIter::U32x2(_) => Type::Vec2,
            AccessorIter::U32x3(_) => Type::Vec3,
            AccessorIter::U32x4(_) => Type::Vec4,
            AccessorIter::U16(_) => Type::Scalar,
            AccessorIter::U16x2(_) => Type::Vec2,
            AccessorIter::U16x3(_) => Type::Vec3,
            AccessorIter::U16x4(_) => Type::Vec4,
            AccessorIter::U8(_) => Type::Scalar,
            AccessorIter::U8x2(_) => Type::Vec2,
            AccessorIter::U8x3(_) => Type::Vec3,
            AccessorIter::U8x4(_) => Type::Vec4,
            AccessorIter::I16(_) => Type::Scalar,
            AccessorIter::I16x2(_) => Type::Vec2,
            AccessorIter::I16x3(_) => Type::Vec3,
            AccessorIter::I16x4(_) => Type::Vec4,
            AccessorIter::I8(_) => Type::Scalar,
            AccessorIter::I8x2(_) => Type::Vec2,
            AccessorIter::I8x3(_) => Type::Vec3,
            AccessorIter::I8x4(_) => Type::Vec4,
        }
    }

    pub fn normalized(&self) -> bool {
        match self {
            AccessorIter::F32(iter) => iter.normalized,
            AccessorIter::F32x2(iter) => iter.normalized,
            AccessorIter::F32x3(iter) => iter.normalized,
            AccessorIter::F32x4(iter) => iter.normalized,
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

    pub fn slice(&self) -> &[u8] {
        match self {
            AccessorIter::F32(iter) => iter.slice,
            AccessorIter::F32x2(iter) => iter.slice,
            AccessorIter::F32x3(iter) => iter.slice,
            AccessorIter::F32x4(iter) => iter.slice,
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
        AccessorElement::F32(value)
    }
}
impl From<[f32; 2]> for AccessorElement {
    fn from(value: [f32; 2]) -> Self {
        AccessorElement::F32x2(value)
    }
}
impl From<[f32; 3]> for AccessorElement {
    fn from(value: [f32; 3]) -> Self {
        AccessorElement::F32x3(value)
    }
}
impl From<[f32; 4]> for AccessorElement {
    fn from(value: [f32; 4]) -> Self {
        AccessorElement::F32x4(value)
    }
}
impl From<u32> for AccessorElement {
    fn from(value: u32) -> Self {
        AccessorElement::U32(value)
    }
}
impl From<[u32; 2]> for AccessorElement {
    fn from(value: [u32; 2]) -> Self {
        AccessorElement::U32x2(value)
    }
}
impl From<[u32; 3]> for AccessorElement {
    fn from(value: [u32; 3]) -> Self {
        AccessorElement::U32x3(value)
    }
}
impl From<[u32; 4]> for AccessorElement {
    fn from(value: [u32; 4]) -> Self {
        AccessorElement::U32x4(value)
    }
}
impl From<u16> for AccessorElement {
    fn from(value: u16) -> Self {
        AccessorElement::U16(value)
    }
}
impl From<[u16; 2]> for AccessorElement {
    fn from(value: [u16; 2]) -> Self {
        AccessorElement::U16x2(value)
    }
}
impl From<[u16; 3]> for AccessorElement {
    fn from(value: [u16; 3]) -> Self {
        AccessorElement::U16x3(value)
    }
}
impl From<[u16; 4]> for AccessorElement {
    fn from(value: [u16; 4]) -> Self {
        AccessorElement::U16x4(value)
    }
}
impl From<u8> for AccessorElement {
    fn from(value: u8) -> Self {
        AccessorElement::U8(value)
    }
}
impl From<[u8; 2]> for AccessorElement {
    fn from(value: [u8; 2]) -> Self {
        AccessorElement::U8x2(value)
    }
}
impl From<[u8; 3]> for AccessorElement {
    fn from(value: [u8; 3]) -> Self {
        AccessorElement::U8x3(value)
    }
}
impl From<[u8; 4]> for AccessorElement {
    fn from(value: [u8; 4]) -> Self {
        AccessorElement::U8x4(value)
    }
}
impl From<i16> for AccessorElement {
    fn from(value: i16) -> Self {
        AccessorElement::I16(value)
    }
}
impl From<[i16; 2]> for AccessorElement {
    fn from(value: [i16; 2]) -> Self {
        AccessorElement::I16x2(value)
    }
}
impl From<[i16; 3]> for AccessorElement {
    fn from(value: [i16; 3]) -> Self {
        AccessorElement::I16x3(value)
    }
}
impl From<[i16; 4]> for AccessorElement {
    fn from(value: [i16; 4]) -> Self {
        AccessorElement::I16x4(value)
    }
}
impl From<i8> for AccessorElement {
    fn from(value: i8) -> Self {
        AccessorElement::I8(value)
    }
}
impl From<[i8; 2]> for AccessorElement {
    fn from(value: [i8; 2]) -> Self {
        AccessorElement::I8x2(value)
    }
}
impl From<[i8; 3]> for AccessorElement {
    fn from(value: [i8; 3]) -> Self {
        AccessorElement::I8x3(value)
    }
}
impl From<[i8; 4]> for AccessorElement {
    fn from(value: [i8; 4]) -> Self {
        AccessorElement::I8x4(value)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ElementIter<'a, T: Element> {
    pub normalized: bool,
    pub slice: &'a [u8],
    _phantom: PhantomData<T>,
}

impl<'a, T: Element> Iterator for ElementIter<'a, T> {
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

impl<'a, T: Element + Copy> ElementIter<'a, T> {
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

    fn gl_max(&self, other: &Self) -> Self;
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
        slice[0] as i8
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
