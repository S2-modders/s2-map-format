use crate::helper_structs::*;
use crate::movement::AnimalMovement;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Resources {
    #[brw(args(4, "resources"))]
    version: Version,
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
    #[brw(args(0, "Resources AnimalRespawn"))]
    version: Version,
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
struct Deposit {
    #[brw(args(1, "deposit"))]
    version: Version,
    id: Uuid,
    pos: PatternCursor,
    buildingref: Uuid,
    pos2: ElevationCursor,
    current_grouth: f32,
    #[brw(if(version.version > 0))]
    age: u32,
    #[brw(if(version.version > 0))]
    life_time: u32,
}

#[binrw]
#[derive(Debug, Default)]
struct Animal {
    mapkey: u32,
    #[brw(args(2, "Resources Animal"))]
    version: Version,
    id: Uuid,
    idk: f32,
    pos: PatternCursor,
    movement: AnimalMovement,
    idk1: u32,
    #[brw(if(version.version > 0))]
    idk2: u32,
    #[brw(if(version.version > 1))]
    villagebuildingref: Uuid,
}
