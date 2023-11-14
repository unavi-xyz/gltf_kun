#[derive(Default, Debug, Clone)]
pub enum ElementType {
    #[default]
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl ElementType {
    pub fn size(&self) -> usize {
        match self {
            ElementType::Scalar => 1,
            ElementType::Vec2 => 2,
            ElementType::Vec3 => 3,
            ElementType::Vec4 => 4,
            ElementType::Mat2 => 4,
            ElementType::Mat3 => 9,
            ElementType::Mat4 => 16,
        }
    }
}

impl From<ElementType> for gltf::json::accessor::Type {
    fn from(val: ElementType) -> Self {
        match val {
            ElementType::Scalar => gltf::json::accessor::Type::Scalar,
            ElementType::Vec2 => gltf::json::accessor::Type::Vec2,
            ElementType::Vec3 => gltf::json::accessor::Type::Vec3,
            ElementType::Vec4 => gltf::json::accessor::Type::Vec4,
            ElementType::Mat2 => gltf::json::accessor::Type::Mat2,
            ElementType::Mat3 => gltf::json::accessor::Type::Mat3,
            ElementType::Mat4 => gltf::json::accessor::Type::Mat4,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AccessorArray {
    I8(Box<[i8]>),
    U8(Box<[u8]>),
    I16(Box<[i16]>),
    U16(Box<[u16]>),
    U32(Box<[u32]>),
    F32(Box<[f32]>),
}

impl AccessorArray {
    pub fn bytes(&self) -> Box<[u8]> {
        match self {
            AccessorArray::I8(array) => array
                .iter()
                .map(|x| x.to_ne_bytes())
                .flatten()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            AccessorArray::U8(array) => array.clone(),
            AccessorArray::I16(array) => array
                .iter()
                .map(|x| x.to_ne_bytes())
                .flatten()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            AccessorArray::U16(array) => array
                .iter()
                .map(|x| x.to_ne_bytes())
                .flatten()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            AccessorArray::U32(array) => array
                .iter()
                .map(|x| x.to_ne_bytes())
                .flatten()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            AccessorArray::F32(array) => array
                .iter()
                .map(|x| x.to_ne_bytes())
                .flatten()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            AccessorArray::I8(array) => array.len(),
            AccessorArray::U8(array) => array.len(),
            AccessorArray::I16(array) => array.len(),
            AccessorArray::U16(array) => array.len(),
            AccessorArray::U32(array) => array.len(),
            AccessorArray::F32(array) => array.len(),
        }
    }
}

impl Default for AccessorArray {
    fn default() -> Self {
        AccessorArray::F32(Box::new([]))
    }
}

impl From<Vec<usize>> for AccessorArray {
    fn from(vec: Vec<usize>) -> Self {
        let vec = vec.iter().map(|&x| x as u32).collect::<Vec<_>>();
        AccessorArray::U32(vec.into_boxed_slice())
    }
}

impl From<Vec<i8>> for AccessorArray {
    fn from(vec: Vec<i8>) -> Self {
        AccessorArray::I8(vec.into_boxed_slice())
    }
}

impl From<Vec<u8>> for AccessorArray {
    fn from(vec: Vec<u8>) -> Self {
        AccessorArray::U8(vec.into_boxed_slice())
    }
}

impl From<Vec<i16>> for AccessorArray {
    fn from(vec: Vec<i16>) -> Self {
        AccessorArray::I16(vec.into_boxed_slice())
    }
}

impl From<Vec<u16>> for AccessorArray {
    fn from(vec: Vec<u16>) -> Self {
        AccessorArray::U16(vec.into_boxed_slice())
    }
}

impl From<Vec<[u8; 2]>> for AccessorArray {
    fn from(vec: Vec<[u8; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U8(vec.into_boxed_slice())
    }
}

impl From<Vec<[u8; 4]>> for AccessorArray {
    fn from(vec: Vec<[u8; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U8(vec.into_boxed_slice())
    }
}

impl From<Vec<[u16; 2]>> for AccessorArray {
    fn from(vec: Vec<[u16; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U16(vec.into_boxed_slice())
    }
}

impl From<Vec<[u16; 4]>> for AccessorArray {
    fn from(vec: Vec<[u16; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U16(vec.into_boxed_slice())
    }
}

impl From<Vec<u32>> for AccessorArray {
    fn from(vec: Vec<u32>) -> Self {
        AccessorArray::U32(vec.into_boxed_slice())
    }
}

impl From<Vec<[u32; 2]>> for AccessorArray {
    fn from(vec: Vec<[u32; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U32(vec.into_boxed_slice())
    }
}

impl From<Vec<[u32; 3]>> for AccessorArray {
    fn from(vec: Vec<[u32; 3]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U32(vec.into_boxed_slice())
    }
}

impl From<Vec<[u32; 4]>> for AccessorArray {
    fn from(vec: Vec<[u32; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::U32(vec.into_boxed_slice())
    }
}

impl From<Vec<f32>> for AccessorArray {
    fn from(vec: Vec<f32>) -> Self {
        AccessorArray::F32(vec.into_boxed_slice())
    }
}

impl From<Vec<[f32; 2]>> for AccessorArray {
    fn from(vec: Vec<[f32; 2]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::F32(vec.into_boxed_slice())
    }
}

impl From<Vec<[f32; 3]>> for AccessorArray {
    fn from(vec: Vec<[f32; 3]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::F32(vec.into_boxed_slice())
    }
}

impl From<Vec<[f32; 4]>> for AccessorArray {
    fn from(vec: Vec<[f32; 4]>) -> Self {
        let vec = vec.iter().flatten().copied().collect::<Vec<_>>();
        AccessorArray::F32(vec.into_boxed_slice())
    }
}
