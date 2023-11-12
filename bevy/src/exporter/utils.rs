use bevy::prelude::Name;

pub fn bevy_name_to_string(name: Option<&Name>) -> Option<String> {
    match name {
        Some(name) => Some(name.to_string()),
        None => None,
    }
}
