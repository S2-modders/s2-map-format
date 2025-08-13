use crate::helper_structs::*;
use crate::movement::SettlerMovement;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Settlers {
    #[brw(args(0, "SettlersSystem"))]
    version: Version,
    init: Bool,
    workers: Array<(PlayerId, Worker)>,
    constructor: Array<(PlayerId, Constructor)>,
    carrier: Array<(PlayerId, Carrier)>,
    bulldoser: Array<(PlayerId, Bulldozer)>,
    soldier: Array<(PlayerId, Soldier)>,
    specialist: Array<(PlayerId, Specialist)>,
}

#[binrw]
#[derive(Debug)]
struct Worker {
    #[brw(args(1, "SettlersWorker"))]
    version: Version,
    work_building_ref: Uuid,
    ship_ref: Uuid,
    test: [u32; 59], //TODO: filler -- decompiling goals takes too long; version 0 has less goals
    settler: Settler,
}

#[binrw]
#[derive(Debug)]
struct Constructor {
    #[brw(args(0, "SettlersConstructor"))]
    version: Version,
    test: [u32; 6], //TODO: filler -- decompiling goals takes too long
    work_building_ref: Uuid,
    settler: Settler,
}

#[binrw]
#[derive(Debug)]
struct Carrier {
    #[brw(args(0, "SettlersCarrier"))]
    version: Version,
    test: [u32; 9], //TODO: filler -- decompiling goals takes too long
    idk: Bool,
    package_ref: Uuid,
    settler: Settler,
}

#[binrw]
#[derive(Debug)]
struct Bulldozer {
    #[brw(args(0, "SettlersBulldozer"))]
    version: Version,
    test: [u32; 2], //TODO: filler -- decompiling goals takes too long
    building_ref: Uuid,
    settler: Settler,
}

#[binrw]
#[derive(Debug)]
struct Soldier {
    #[brw(args(3, "SettlersSoldier"))]
    version: Version,
    test: [u32; 2], //TODO: filler -- decompiling goals takes too long
    building_ref: Uuid,
    building_ref2: Uuid,
    settler_ref: Uuid,
    test0: [u32; 10], //TODO: filler -- decompiling goals takes too long; version dependent
    #[br(if(version.version > 1))]
    building_ref3: Option<Uuid>,
    #[br(if(version.version > 2))]
    idk: Bool,
    live_points: LivePoints,
    settler: Settler,
}

#[binrw]
#[derive(Debug)]
struct LivePoints {
    #[brw(args(0, "SettlersLivePoints"))]
    version: Version,
    idk: f32,
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
struct Specialist {
    #[brw(args(0, "SettlersSpecialist"))]
    version: Version,
    #[br(dbg)]
    test: [u32; 20], //TODO: filler -- decompiling goals takes too long -- not tested if right size
    idk: u32,
    destination_flag_ref: Uuid,
    settler: Settler,
}

#[binrw]
#[derive(Debug)]
struct Settler {
    #[brw(args(0, "Settlers Settler"))]
    version: Version,
    id: Uuid,
    movement: SettlerMovement,
    animation: Animation,
    package_ref: Uuid,
    settler_type: u32,
    state: u32,
    test: [u32; 5], //TODO: filler -- decompiling goals takes too long
    building_ref: Uuid,
}

#[binrw]
#[derive(Debug)]
struct Animation {
    #[brw(args(1, "SettlersAnimation"))]
    version: Version,
    remaining_time: f32,
    #[brw(if(version.version == 1))]
    end_time: Option<f32>,
    animation_type: u32,
}
