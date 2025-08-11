use crate::helper_structs::*;
use binrw::binrw;
use strum::*;

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

#[binrw]
#[derive(Debug, Default)]
struct AnimalMovement {
    #[brw(args(1, "Navy Movement"))]
    version: Version,
    path: ResourcePath,
    #[brw(if(version.version > 0))]
    pos: Option<PatternCursor>, //Get this instead in movementBase.pos if None
    movement_base: MovementBase,
}

#[binrw]
#[derive(Debug, Default)]
struct MovementPathBase {
    #[brw(args(1, "Movement Path Base"))]
    version: Version,
    init: Bool,
    #[brw(if (init.bool || version.version == 0))]
    poses: Array<PatternCursor>,
    #[brw(if (init.bool || version.version == 0))]
    idk: Bool,
    #[brw(if (init.bool || version.version == 0))]
    idk1: i32,
    #[brw(if (init.bool || version.version == 0))]
    idk2: Bool,
}

#[binrw]
#[derive(Debug, Default)]
struct ResourcePath {
    #[brw(args(0, "Resources Path"))]
    version: Version,
    base: MovementPathBase,
}

#[binrw]
#[derive(Debug, Default)]
struct MovementBase {
    #[brw(args(0, "Movement Base"))]
    version: Version,
    pos: PatternCursor,
    idk: PatternCursor,
    idk1: PatternCursor,
    interpolator: MovementInterpolator,
}

#[binrw]
#[derive(Debug, Default)]
struct MovementInterpolator {
    #[brw(args(0, "Movement Interpolator"))]
    version: Version,
    idk1: f32,
    idk2: f32,
    idk3: f32,
}
