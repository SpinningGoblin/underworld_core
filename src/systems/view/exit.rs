use crate::components::rooms::exit::{Exit, ExitView};

pub fn look_at(exit: &Exit) -> ExitView {
    ExitView {
        identifier: super::identifier::to_view(&exit.identifier, true),
        exit_type: exit.exit_type.clone(),
        material: exit.material.clone(),
        descriptors: exit.descriptors.to_vec(),
        size: exit.size.clone(),
    }
}
