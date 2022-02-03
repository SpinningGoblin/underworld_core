use super::material::Material;

pub trait Object {
    fn look_at(&self, is_equipped: bool) -> String;
    fn is_multiple(&self) -> bool;
    fn material(&self) -> Option<Material>;
}
