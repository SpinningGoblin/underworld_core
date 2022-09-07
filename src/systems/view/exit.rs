use crate::components::rooms::{Exit, ExitView};

pub fn view(exit: &Exit, has_visited_connected_room: bool) -> ExitView {
    ExitView {
        has_visited_connected_room,
        id: exit.id.to_string(),
        name: exit.name.clone(),
        exit_type: exit.exit_type,
        material: exit.material,
        descriptors: exit.descriptors.to_vec(),
        size: exit.size,
    }
}
