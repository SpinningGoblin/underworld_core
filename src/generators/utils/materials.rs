use crate::components::{items::ItemType, Material};

pub fn possible_materials(item_type: &ItemType) -> Vec<Material> {
    match *item_type {
        ItemType::Breastplate => vec![Material::Iron, Material::Leather, Material::Steel],
        ItemType::Mask => vec![Material::Bone, Material::Iron],
        ItemType::Cloak => {
            vec![Material::Linen, Material::Hide, Material::Wool]
        }
        ItemType::Shirt => vec![
            Material::Wool,
            Material::Linen,
            Material::Cotton,
            Material::Silk,
        ],
        ItemType::Trousers => vec![
            Material::Hide,
            Material::Leather,
            Material::Wool,
            Material::Linen,
        ],
        ItemType::Crown => {
            vec![Material::Bone, Material::Gold, Material::Stone]
        }
        ItemType::Boots => vec![
            Material::Hide,
            Material::Iron,
            Material::Leather,
            Material::Steel,
        ],
        ItemType::Gloves | ItemType::Vest => vec![Material::Fur, Material::Hide, Material::Leather],
        ItemType::LoinCloth => vec![
            Material::Hide,
            Material::Wool,
            Material::Leather,
            Material::Silk,
            Material::Linen,
            Material::Cotton,
        ],
        ItemType::PlateBoots | ItemType::PlateGauntlets | ItemType::PlateHelmet => {
            vec![Material::Iron, Material::Steel]
        }
        ItemType::Shackles => vec![Material::Iron, Material::Leather, Material::Steel],
        ItemType::Buckler => {
            vec![Material::Hide, Material::Iron, Material::Steel]
        }
        ItemType::Club => vec![Material::Bone, Material::Stone, Material::Wooden],
        ItemType::Dagger => vec![
            Material::Bone,
            Material::Gold,
            Material::Iron,
            Material::Steel,
            Material::Stone,
        ],
        ItemType::Dirk | ItemType::GreatSword => vec![
            Material::Bone,
            Material::Iron,
            Material::Steel,
            Material::Stone,
        ],
        ItemType::Hammer | ItemType::LongSword => {
            vec![Material::Bone, Material::Iron, Material::Steel]
        }
        ItemType::Mace => vec![Material::Iron, Material::Steel],
        ItemType::Morningstar => vec![Material::Iron, Material::Steel],
        ItemType::Shield => vec![
            Material::Hide,
            Material::Iron,
            Material::Leather,
            Material::Steel,
            Material::Wooden,
        ],
        ItemType::ShortSword => vec![Material::Iron, Material::Steel],
        ItemType::Whip => vec![Material::Leather],
        ItemType::Helm => vec![
            Material::Iron,
            Material::Hide,
            Material::Steel,
            Material::Leather,
            Material::Fur,
        ],
        ItemType::Halberd => vec![
            Material::Bone,
            Material::Wooden,
            Material::Steel,
            Material::Iron,
        ],
        ItemType::Pike => vec![
            Material::Bone,
            Material::Wooden,
            Material::Steel,
            Material::Iron,
        ],
        ItemType::Spear => vec![
            Material::Bone,
            Material::Wooden,
            Material::Steel,
            Material::Iron,
        ],
        ItemType::Scroll => vec![
            Material::Paper,
            Material::Papyrus,
            Material::Bone,
            Material::Linen,
        ],
        ItemType::BowlerHat | ItemType::Fedora | ItemType::TopHat => {
            vec![
                Material::Bone,
                Material::Cotton,
                Material::Fur,
                Material::Gold,
                Material::Hide,
                Material::Iron,
                Material::Leather,
                Material::Linen,
                Material::Silk,
                Material::Steel,
                Material::Stone,
                Material::Wooden,
                Material::Wool,
            ]
        }
        ItemType::Pot => vec![Material::Bone, Material::Ceramic],
        ItemType::Flask => vec![
            Material::Ceramic,
            Material::Bone,
            Material::Glass,
            Material::Steel,
            Material::Iron,
            Material::Gold,
        ],
    }
}
