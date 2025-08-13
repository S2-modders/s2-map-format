use crate::buildings::Building;
use crate::helper_structs::*;
use crate::movement::AnimalMovement;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Resources {
    #[brw(args("resources"))]
    version: Version<4>,
    init: Bool,
    deposits: Array<(u32, Deposit)>,

    #[brw(if(version.version > 0))]
    animals: Array<Animal>,
    #[brw(if(version.version > 2))]
    respawn: Option<AnimalRespawn>,
    #[brw(if(version.version > 3))]
    idk: u32,
    #[brw(if(version.version > 3))]
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
struct AnimalRespawn {
    #[brw(args("Resources AnimalRespawn"))]
    version: Version<0>,
    init: Bool,
    tick: u32,
    inc: u32,
    pos: UPos, //TODO
}

#[binrw]
#[derive(Debug)]
struct UPos {
    x: u32,
    y: u32,
}

#[binrw]
#[derive(Debug)]
pub struct Deposit {
    #[brw(args("deposit"))]
    version: Version<1>,
    id: Uuid,
    pos: PatternCursor,
    buildingref: Ref<Building>,
    pos2: ElevationCursor,
    current_grouth: f32,
    #[brw(if(version.version > 0))]
    age: u32,
    #[brw(if(version.version > 0))]
    life_time: u32,
}

impl Ided for Deposit {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
pub struct Animal {
    mapkey: u32,
    #[brw(args("Resources Animal"))]
    version: Version<2>,
    id: Uuid,
    idk: f32,
    pos: PatternCursor,
    movement: AnimalMovement,
    idk1: u32,
    #[brw(if(version.version > 0))]
    idk2: u32,
    #[brw(if(version.version > 1))]
    villagebuildingref: Ref<Building>,
}

impl Ided for Animal {
    fn id(&self) -> Uuid {
        self.id
    }
}
