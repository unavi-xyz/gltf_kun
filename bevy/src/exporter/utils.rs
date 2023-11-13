use bevy::prelude::Name;

pub fn name_to_string(name: Option<&Name>) -> Option<String> {
    name.map(|name| name.to_string())
}
