pub fn asset_label(base: &str, index: usize, name: Option<&str>) -> String {
    match name {
        Some(n) => format!("{}/{}", base, n),
        None => format!("{}{}", base, index),
    }
}
