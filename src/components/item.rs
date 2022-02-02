use super::{equipped::equip_location_descriptor::EquipLocationDescriptor, material::Material};

pub trait Item {
    fn look_at(&self, is_equipped: bool) -> String;
    fn is_multiple(&self) -> bool;
    fn material(&self) -> Option<Material>;
}

pub trait EquippableItem {
    fn possible_equip_locations(&self) -> Vec<EquipLocationDescriptor>;
}
