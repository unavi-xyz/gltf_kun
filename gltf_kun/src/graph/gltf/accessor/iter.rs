use gltf::{
    accessor::{util::ItemIter, Iter},
    json::accessor::{ComponentType, Type},
};
use serde_json::Number;

pub enum AccessorIter<'a> {
    F32(Iter<'a, f32>),
    F32x2(Iter<'a, [f32; 2]>),
    F32x3(Iter<'a, [f32; 3]>),
    F32x4(Iter<'a, [f32; 4]>),
    U32(Iter<'a, u32>),
    U32x2(Iter<'a, [u32; 2]>),
    U32x3(Iter<'a, [u32; 3]>),
    U32x4(Iter<'a, [u32; 4]>),
    U16(Iter<'a, u16>),
    U16x2(Iter<'a, [u16; 2]>),
    U16x3(Iter<'a, [u16; 3]>),
    U16x4(Iter<'a, [u16; 4]>),
    U8(Iter<'a, u8>),
    U8x2(Iter<'a, [u8; 2]>),
    U8x3(Iter<'a, [u8; 3]>),
    U8x4(Iter<'a, [u8; 4]>),
    I16(Iter<'a, i16>),
    I16x2(Iter<'a, [i16; 2]>),
    I16x3(Iter<'a, [i16; 3]>),
    I16x4(Iter<'a, [i16; 4]>),
    I8(Iter<'a, i8>),
    I8x2(Iter<'a, [i8; 2]>),
    I8x3(Iter<'a, [i8; 3]>),
    I8x4(Iter<'a, [i8; 4]>),
}

impl AccessorIter<'_> {
    pub fn element_type(&self) -> Type {
        match self {
            Self::F32(_) => Type::Scalar,
            Self::F32x2(_) => Type::Vec2,
            Self::F32x3(_) => Type::Vec3,
            Self::F32x4(_) => Type::Vec4,
            Self::U32(_) => Type::Scalar,
            Self::U32x2(_) => Type::Vec2,
            Self::U32x3(_) => Type::Vec3,
            Self::U32x4(_) => Type::Vec4,
            Self::U16(_) => Type::Scalar,
            Self::U16x2(_) => Type::Vec2,
            Self::U16x3(_) => Type::Vec3,
            Self::U16x4(_) => Type::Vec4,
            Self::U8(_) => Type::Scalar,
            Self::U8x2(_) => Type::Vec2,
            Self::U8x3(_) => Type::Vec3,
            Self::U8x4(_) => Type::Vec4,
            Self::I16(_) => Type::Scalar,
            Self::I16x2(_) => Type::Vec2,
            Self::I16x3(_) => Type::Vec3,
            Self::I16x4(_) => Type::Vec4,
            Self::I8(_) => Type::Scalar,
            Self::I8x2(_) => Type::Vec2,
            Self::I8x3(_) => Type::Vec3,
            Self::I8x4(_) => Type::Vec4,
        }
    }

    pub fn component_type(&self) -> ComponentType {
        match self {
            Self::F32(_) => ComponentType::F32,
            Self::F32x2(_) => ComponentType::F32,
            Self::F32x3(_) => ComponentType::F32,
            Self::F32x4(_) => ComponentType::F32,
            Self::U32(_) => ComponentType::U32,
            Self::U32x2(_) => ComponentType::U32,
            Self::U32x3(_) => ComponentType::U32,
            Self::U32x4(_) => ComponentType::U32,
            Self::U16(_) => ComponentType::U16,
            Self::U16x2(_) => ComponentType::U16,
            Self::U16x3(_) => ComponentType::U16,
            Self::U16x4(_) => ComponentType::U16,
            Self::U8(_) => ComponentType::U8,
            Self::U8x2(_) => ComponentType::U8,
            Self::U8x3(_) => ComponentType::U8,
            Self::U8x4(_) => ComponentType::U8,
            Self::I16(_) => ComponentType::I16,
            Self::I16x2(_) => ComponentType::I16,
            Self::I16x3(_) => ComponentType::I16,
            Self::I16x4(_) => ComponentType::I16,
            Self::I8(_) => ComponentType::I8,
            Self::I8x2(_) => ComponentType::I8,
            Self::I8x3(_) => ComponentType::I8,
            Self::I8x4(_) => ComponentType::I8,
        }
    }

    pub fn max(&self) -> Element {
        match self {
            Self::F32(iter) => Element::F32(
                iter.clone()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            ),
            Self::F32x2(iter) => Element::F32x2(
                iter.clone()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            ),
            Self::F32x3(iter) => Element::F32x3(
                iter.clone()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            ),
            Self::F32x4(iter) => Element::F32x4(
                iter.clone()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            ),
            Self::U32(iter) => Element::U32(iter.clone().max().unwrap()),
            Self::U32x2(iter) => Element::U32x2(iter.clone().max().unwrap()),
            Self::U32x3(iter) => Element::U32x3(iter.clone().max().unwrap()),
            Self::U32x4(iter) => Element::U32x4(iter.clone().max().unwrap()),
            Self::U16(iter) => Element::U16(iter.clone().max().unwrap()),
            Self::U16x2(iter) => Element::U16x2(iter.clone().max().unwrap()),
            Self::U16x3(iter) => Element::U16x3(iter.clone().max().unwrap()),
            Self::U16x4(iter) => Element::U16x4(iter.clone().max().unwrap()),
            Self::U8(iter) => Element::U8(iter.clone().max().unwrap()),
            Self::U8x2(iter) => Element::U8x2(iter.clone().max().unwrap()),
            Self::U8x3(iter) => Element::U8x3(iter.clone().max().unwrap()),
            Self::U8x4(iter) => Element::U8x4(iter.clone().max().unwrap()),
            Self::I16(iter) => Element::I16(iter.clone().max().unwrap()),
            Self::I16x2(iter) => Element::I16x2(iter.clone().max().unwrap()),
            Self::I16x3(iter) => Element::I16x3(iter.clone().max().unwrap()),
            Self::I16x4(iter) => Element::I16x4(iter.clone().max().unwrap()),
            Self::I8(iter) => Element::I8(iter.clone().max().unwrap()),
            Self::I8x2(iter) => Element::I8x2(iter.clone().max().unwrap()),
            Self::I8x3(iter) => Element::I8x3(iter.clone().max().unwrap()),
            Self::I8x4(iter) => Element::I8x4(iter.clone().max().unwrap()),
        }
    }

    pub fn min(&self) -> Element {
        match self {
            Self::F32(iter) => Element::F32(
                iter.clone()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            ),
            Self::F32x2(iter) => Element::F32x2(
                iter.clone()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            ),
            Self::F32x3(iter) => Element::F32x3(
                iter.clone()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            ),
            Self::F32x4(iter) => Element::F32x4(
                iter.clone()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            ),
            Self::U32(iter) => Element::U32(iter.clone().min().unwrap()),
            Self::U32x2(iter) => Element::U32x2(iter.clone().min().unwrap()),
            Self::U32x3(iter) => Element::U32x3(iter.clone().min().unwrap()),
            Self::U32x4(iter) => Element::U32x4(iter.clone().min().unwrap()),
            Self::U16(iter) => Element::U16(iter.clone().min().unwrap()),
            Self::U16x2(iter) => Element::U16x2(iter.clone().min().unwrap()),
            Self::U16x3(iter) => Element::U16x3(iter.clone().min().unwrap()),
            Self::U16x4(iter) => Element::U16x4(iter.clone().min().unwrap()),
            Self::U8(iter) => Element::U8(iter.clone().min().unwrap()),
            Self::U8x2(iter) => Element::U8x2(iter.clone().min().unwrap()),
            Self::U8x3(iter) => Element::U8x3(iter.clone().min().unwrap()),
            Self::U8x4(iter) => Element::U8x4(iter.clone().min().unwrap()),
            Self::I16(iter) => Element::I16(iter.clone().min().unwrap()),
            Self::I16x2(iter) => Element::I16x2(iter.clone().min().unwrap()),
            Self::I16x3(iter) => Element::I16x3(iter.clone().min().unwrap()),
            Self::I16x4(iter) => Element::I16x4(iter.clone().min().unwrap()),
            Self::I8(iter) => Element::I8(iter.clone().min().unwrap()),
            Self::I8x2(iter) => Element::I8x2(iter.clone().min().unwrap()),
            Self::I8x3(iter) => Element::I8x3(iter.clone().min().unwrap()),
            Self::I8x4(iter) => Element::I8x4(iter.clone().min().unwrap()),
        }
    }
}

impl<'a> From<ItemIter<'a, f32>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, f32>) -> Self {
        Self::F32(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [f32; 2]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [f32; 2]>) -> Self {
        Self::F32x2(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [f32; 3]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [f32; 3]>) -> Self {
        Self::F32x3(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [f32; 4]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [f32; 4]>) -> Self {
        Self::F32x4(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, u32>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, u32>) -> Self {
        Self::U32(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [u32; 2]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [u32; 2]>) -> Self {
        Self::U32x2(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [u32; 3]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [u32; 3]>) -> Self {
        Self::U32x3(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [u32; 4]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [u32; 4]>) -> Self {
        Self::U32x4(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, u16>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, u16>) -> Self {
        Self::U16(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [u16; 2]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [u16; 2]>) -> Self {
        Self::U16x2(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [u16; 3]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [u16; 3]>) -> Self {
        Self::U16x3(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [u16; 4]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [u16; 4]>) -> Self {
        Self::U16x4(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, u8>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, u8>) -> Self {
        Self::U8(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [u8; 2]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [u8; 2]>) -> Self {
        Self::U8x2(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [u8; 3]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [u8; 3]>) -> Self {
        Self::U8x3(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [u8; 4]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [u8; 4]>) -> Self {
        Self::U8x4(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, i16>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, i16>) -> Self {
        Self::I16(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [i16; 2]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [i16; 2]>) -> Self {
        Self::I16x2(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [i16; 3]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [i16; 3]>) -> Self {
        Self::I16x3(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [i16; 4]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [i16; 4]>) -> Self {
        Self::I16x4(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, i8>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, i8>) -> Self {
        Self::I8(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [i8; 2]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [i8; 2]>) -> Self {
        Self::I8x2(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [i8; 3]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [i8; 3]>) -> Self {
        Self::I8x3(Iter::Standard(iter))
    }
}

impl<'a> From<ItemIter<'a, [i8; 4]>> for AccessorIter<'a> {
    fn from(iter: ItemIter<'a, [i8; 4]>) -> Self {
        Self::I8x4(Iter::Standard(iter))
    }
}

#[derive(Clone, Debug)]
pub enum Element {
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

impl From<Element> for serde_json::Value {
    fn from(value: Element) -> Self {
        match value {
            Element::F32(value) => Self::Number(Number::from_f64(value as f64).unwrap()),
            Element::F32x2(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number(Number::from_f64(*value as f64).unwrap()))
                    .collect(),
            ),
            Element::F32x3(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number(Number::from_f64(*value as f64).unwrap()))
                    .collect(),
            ),
            Element::F32x4(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number(Number::from_f64(*value as f64).unwrap()))
                    .collect(),
            ),
            Element::U32(value) => Self::Number(value.into()),
            Element::U32x2(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::U32x3(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::U32x4(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::U16(value) => Self::Number(value.into()),
            Element::U16x2(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::U16x3(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::U16x4(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::U8(value) => Self::Number(value.into()),
            Element::U8x2(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::U8x3(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::U8x4(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::I16(value) => Self::Number(value.into()),
            Element::I16x2(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::I16x3(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::I16x4(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::I8(value) => Self::Number(value.into()),
            Element::I8x2(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::I8x3(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
            Element::I8x4(value) => Self::Array(
                value
                    .iter()
                    .map(|value| Self::Number((*value).into()))
                    .collect(),
            ),
        }
    }
}
