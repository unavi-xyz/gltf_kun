pub struct Assets(Vec<AssetRef>);

pub struct AssetRef {
    pub uri: String,
    pub ndoes: Vec<String>,
}
