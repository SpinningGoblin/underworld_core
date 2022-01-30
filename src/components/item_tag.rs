pub trait TaggedItem {
    fn tag(&self) -> ItemTag;
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemTag {
    Accessory,
    Armour,
    Blade,
    Blunt,
    Bone,
    Cloth,
    Clothing,
    Leather,
    Metal,
    Rope,
    Shield,
    Stone,
    Wood,
}
