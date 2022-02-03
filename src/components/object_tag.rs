pub trait TaggedObject {
    fn tags(&self) -> Vec<ObjectTag>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectTag {
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

impl ObjectTag {
    pub fn is_weapon(&self) -> bool {
        vec![ObjectTag::Blade, ObjectTag::Blunt, ObjectTag::Whip].contains(self)
    }

    pub fn is_wearable(&self) -> bool {
        vec![ObjectTag::Accessory, ObjectTag::Armour, ObjectTag::Clothing].contains(self)
    }
}
