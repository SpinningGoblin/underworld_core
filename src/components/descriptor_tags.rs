pub trait DescriptorTagged {
    fn descriptor_tag(&self) -> DescriptorTag;
}

#[derive(Clone, Debug, PartialEq)]
pub enum DescriptorTag {
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
