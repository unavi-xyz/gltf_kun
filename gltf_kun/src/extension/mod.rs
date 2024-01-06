pub trait Extension {
    fn name(&self) -> &str;
}

pub trait ExtensionProperty: std::fmt::Debug {
    fn extension_name(&self) -> &str;
    fn parent_types(&self) -> Vec<&str>;
}
