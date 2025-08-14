use crate::Version;

use crate::{buildings::Building, helper_structs::*, settlers::Settler};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Military {
    version: Version!(0, "MilitarySystem"),
    init: Bool,
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
    #[brw(if(version.version > 0))]
    fighters3: Array<Fighters>,
}

#[binrw]
#[derive(Debug)]
struct Fighters {
    version: Version!(2, "Military Fighters"),
    init: Bool,
    #[brw(if(version.version < 1))]
    settler_ref0: Option<Ref<Settler>>,
    #[brw(if(version.version < 1))]
    settler_ref1: Option<Ref<Settler>>,
    #[brw(if(version.version < 1))]
    building_ref: Option<Ref<Building>>,
    #[brw(if(version.version > 0))]
    fightconnections: Option<FightConnections>,
    idk: u32,
    idk2: Bool,
    idk3: Bool,
    pos: PatternCursor,
    #[brw(if(version.version > 1))]
    idk4: Bool,
    idk5: Bool,
}

#[binrw]
#[derive(Debug)]
struct FightConnections {
    version: Version!(0, "Military FightConnections"),
    init: Bool,
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
