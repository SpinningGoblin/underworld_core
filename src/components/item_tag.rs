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
    Whip,
    Wood,
}

impl ItemTag {
    pub fn is_weapon(&self) -> bool {
        vec![ItemTag::Blade, ItemTag::Blunt, ItemTag::Whip].contains(self)
    }

    pub fn is_wearable(&self) -> bool {
        vec![ItemTag::Accessory, ItemTag::Armour, ItemTag::Clothing].contains(self)
    }
}
