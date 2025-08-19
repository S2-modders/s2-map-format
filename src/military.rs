use crate::Version;
use crate::VersionI;
use crate::Versioned;

use crate::{buildings::Building, helper_structs::*, settlers::Settler};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Military {
    version: VersionI!("MilitarySystem"),
    recruiting: Versioned!("Military Recruting", Cooldown<100>),
    distances: Versioned!("Military Distances", Cooldown<10>),
    allocation: Versioned!("Military Allocation", Cooldown<20>),
    fight: Fight,
    attack: Version!("Military Attack"),
    training: Versioned!("Military Training", Cooldown<300>),
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
    version: VersionI!("Military FightConnections"),
    settler_ref0: Ref<Settler>,
    settler_ref1: Ref<Settler>,
    building_ref: Ref<Building>,
}
