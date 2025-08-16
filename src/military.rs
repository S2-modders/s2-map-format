use crate::VersionI;
use crate::Version;

use crate::{buildings::Building, helper_structs::*, settlers::Settler};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Military {
    version: VersionI!(0, "MilitarySystem"),
    // #[brw(assert(init.bool))]
    // init: Bool,
    recruiting: Recruiting,
    distances: Distances,
    allocation: Allocation,
    fight: Fight,
    attack: Attack,
    training: Training,
}

#[binrw]
#[derive(Debug)]
struct Recruiting {
    version: Version!(0, "Military Recruting"),
    tick: CapedU32<100>,
}

#[binrw]
#[derive(Debug)]
struct Distances {
    version: Version!(0, "Military Distances"),
    tick: CapedU32<10>,
}

#[binrw]
#[derive(Debug)]
struct Allocation {
    version: Version!(0, "Military Allocation"),
    tick: CapedU32<20>,
}

#[binrw]
#[derive(Debug)]
struct Fight {
    version: Version!(1, "Military Fight"),
    fighters: Array<Fighters>,
    fighters2: Array<Fighters>,
    fighters3: Array<Fighters>,
}

#[binrw]
#[derive(Debug)]
struct Fighters {
    version: VersionI!(2, "Military Fighters"),
    // #[brw(assert(init.bool))]
    // init: Bool,
    fightconnections: FightConnections,
    idk: u32,
    idk2: Bool,
    idk3: Bool,
    pos: PatternCursor,
    idk4: Bool,
    idk5: Bool,
}

#[binrw]
#[derive(Debug)]
struct FightConnections {
    version: VersionI!(0, "Military FightConnections"),
    // #[brw(assert(init.bool))]
    // init: Bool,
    settler_ref0: Ref<Settler>,
    settler_ref1: Ref<Settler>,
    building_ref: Ref<Building>,
}

#[binrw]
#[derive(Debug)]
struct Attack {
    version: Version!(0, "Military Attack"),
}

#[binrw]
#[derive(Debug)]
struct Training {
    version: Version!(0, "Military Training"),
    tick: CapedU32<300>,
}
