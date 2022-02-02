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
    Container,
    Equipped,
    Fixture,
    Leather,
    Metal,
    Rope,
    Shield,
    Stone,
    Wood,
}
