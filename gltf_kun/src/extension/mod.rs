use std::fmt::Debug;

pub trait Extension {
    fn name(&self) -> &str;
}

pub trait ExtensionProperty: Debug + Send + Sync {
    fn extension_name(&self) -> &str;
    fn parent_types(&self) -> Vec<&str>;
}
