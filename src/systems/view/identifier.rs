use crate::components::identifier::{Identifier, IdentifierView};

pub fn view(identifier: &Identifier, name_known: bool) -> IdentifierView {
    if name_known {
        IdentifierView {
            id: identifier.id.to_string(),
            name: identifier.name.clone(),
            name_known,
        }
    } else {
        IdentifierView {
            id: identifier.id.to_string(),
            name: None,
            name_known,
        }
    }
}
