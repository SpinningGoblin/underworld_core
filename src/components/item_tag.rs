pub trait TaggedItem {
    fn tags(&self) -> Vec<ItemTag>;
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
    Equipped,
    Leather,
    Metal,
    Rope,
    Shield,
    Stone,
    Wood,
}
