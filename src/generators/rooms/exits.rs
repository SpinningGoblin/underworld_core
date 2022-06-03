use enum_iterator::IntoEnumIterator;
use rand::{prelude::ThreadRng, Rng};
use uuid::Uuid;

use crate::components::{
    material::Material,
    rooms::{
        exit::Exit, exit_descriptor::ExitDescriptor, exit_type::ExitType, room_type::RoomType,
    },
    size::Size,
};

pub fn build_exits(room_type: &RoomType, entrance_id: Option<Uuid>) -> Vec<Exit> {
    let mut rng = rand::thread_rng();
    let num_exits = num_exits(&mut rng, room_type);

    (0..num_exits)
        .map(|index| {
            let id = if index == 0 {
                match entrance_id {
                    Some(it) => it,
                    None => Uuid::new_v4(),
                }
            } else {
                Uuid::new_v4()
            };

            let exit_type = exit_type(&mut rng, room_type);
            let material = material(&mut rng, &exit_type);
            let size = size(&mut rng, &exit_type);
            let descriptors = descriptors(&mut rng, &exit_type, &material);

            Exit {
                exit_type,
                material,
                size,
                descriptors,
                id,
                name: None,
            }
        })
        .into_iter()
        .collect()
}

fn num_exits(rng: &mut ThreadRng, room_type: &RoomType) -> usize {
    match *room_type {
        RoomType::PrisonCell => rng.gen_range(1..=2),
        RoomType::Cavern
        | RoomType::TavernHall
        | RoomType::Mausoleum
        | RoomType::Cemetery
        | RoomType::Crypt
        | RoomType::TempleHall
        | RoomType::Cave
        | RoomType::Room => rng.gen_range(2..=4),
        RoomType::EntryWay => 2,
    }
}

fn exit_type(rng: &mut ThreadRng, room_type: &RoomType) -> ExitType {
    let possible_types: Vec<ExitType> = match *room_type {
        RoomType::PrisonCell => vec![
            ExitType::DugOutTunnelEntrance,
            ExitType::Door,
            ExitType::OpeningToTheVoid,
            ExitType::HoleInTheFloor,
            ExitType::HoleInTheWall,
        ],
        _ => ExitType::into_enum_iter().collect(),
    };

    let index = rng.gen_range(0..possible_types.len());
    possible_types.get(index).unwrap().to_owned()
}

fn material(rng: &mut ThreadRng, exit_type: &ExitType) -> Option<Material> {
    let possible_materials: Vec<Material> = match *exit_type {
        ExitType::Door | ExitType::StaircaseUp | ExitType::StaircaseDown => vec![
            Material::Iron,
            Material::Wooden,
            Material::Steel,
            Material::Stone,
            Material::Bone,
        ],
        ExitType::HoleInTheWall
        | ExitType::OpeningToTheVoid
        | ExitType::HoleInTheFloor
        | ExitType::Hallway
        | ExitType::DugOutTunnelEntrance => return None,
    };

    let index = rng.gen_range(0..possible_materials.len());
    possible_materials.get(index).cloned()
}

fn descriptors(
    rng: &mut ThreadRng,
    exit_type: &ExitType,
    material: &Option<Material>,
) -> Vec<ExitDescriptor> {
    let num_descriptors: usize = rng.gen_range(0..=2);

    if num_descriptors == 0 {
        return Vec::new();
    }

    let exit_type_descriptors: Vec<ExitDescriptor> = match *exit_type {
        ExitType::Door | ExitType::StaircaseUp | ExitType::StaircaseDown => {
            vec![ExitDescriptor::Old]
        }
        _ => Vec::new(),
    };

    let material_descriptors: Vec<ExitDescriptor> = material
        .as_ref()
        .map(|m| match *m {
            Material::Iron | Material::Steel => vec![ExitDescriptor::Rusty],
            _ => Vec::new(),
        })
        .unwrap_or_default();

    let mut possible_descriptors: Vec<ExitDescriptor> = exit_type_descriptors
        .into_iter()
        .chain(material_descriptors.into_iter())
        .collect();

    if possible_descriptors.is_empty() {
        return Vec::new();
    }

    (0..num_descriptors)
        .flat_map(|_| {
            if possible_descriptors.is_empty() {
                None
            } else {
                let index = rng.gen_range(0..possible_descriptors.len());
                Some(possible_descriptors.remove(index))
            }
        })
        .collect()
}

fn size(rng: &mut ThreadRng, exit_type: &ExitType) -> Option<Size> {
    let possible_sizes: Vec<Size> = match *exit_type {
        ExitType::Door
        | ExitType::HoleInTheWall
        | ExitType::HoleInTheFloor
        | ExitType::DugOutTunnelEntrance => vec![
            Size::Average,
            Size::Huge,
            Size::Large,
            Size::Massive,
            Size::Short,
            Size::Small,
            Size::Squat,
            Size::Wide,
            Size::Tall,
            Size::Tiny,
        ],
        ExitType::OpeningToTheVoid => vec![Size::Huge, Size::Large, Size::Massive, Size::Tiny],
        ExitType::StaircaseUp | ExitType::StaircaseDown => {
            vec![Size::Long, Size::Narrow, Size::Massive, Size::Huge]
        }
        ExitType::Hallway => vec![Size::Long, Size::Short, Size::Wide, Size::Narrow],
    };

    let index = rng.gen_range(0..possible_sizes.len());
    possible_sizes.get(index).cloned()
}
