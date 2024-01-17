//! glTF extensions.
//!
//! Each extension IO is implemented for a specfic [format](crate::io::format).

use std::{collections::HashMap, error::Error, fmt::Debug, sync::Arc};

pub mod omi_physics_body;
pub mod omi_physics_shape;

#[derive(Default)]
pub struct Extensions<D, F> {
    pub map: HashMap<String, Arc<Box<dyn ExtensionIO<D, F>>>>,
}

impl<D, F> Clone for Extensions<D, F> {
    fn clone(&self) -> Self {
        let map = self
            .map
            .iter()
            .map(|(k, v)| (k.clone(), Arc::clone(v)))
            .collect();

        Self { map }
    }
}

pub trait ExtensionIO<D, F> {
    fn name(&self) -> &'static str;

    /// Export the extension from the document to the format.
    fn export(&self, doc: &mut D, format: &mut F) -> Result<(), Box<dyn Error>>;

    /// Import the extension from the format to the document.
    fn import(&self, format: &mut F, doc: &mut D) -> Result<(), Box<dyn Error>>;
}

pub trait ExtensionProperty: Debug {
    fn extension_name(&self) -> &'static str;
}
