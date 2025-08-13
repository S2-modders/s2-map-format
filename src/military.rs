use crate::helper_structs::*;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Military {
    #[brw(args(0, "MilitarySystem"))]
    version: Version,
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
    #[brw(args(0, "Military Recruting"))]
    version: Version,
    tick: u32, //0 - 100 (current tick mod 101)
}

#[binrw]
#[derive(Debug)]
struct Distances {
    #[brw(args(0, "Military Distances"))]
    version: Version,
    tick: u32, //0 - 10 (current tick mod 11)
}

#[binrw]
#[derive(Debug)]
struct Allocation {
    #[brw(args(0, "Military Allocation"))]
    version: Version,
    tick: u32, //0 - 20 (current tick mod 21)
}

#[binrw]
#[derive(Debug)]
struct Fight {
    #[brw(args(1, "Military Fight"))]
    version: Version,
    fighters: Array<Fighters>,
    fighters2: Array<Fighters>,
    #[brw(if(version.version > 0))]
    fighters3: Array<Fighters>,
}

#[binrw]
#[derive(Debug)]
struct Fighters {
    #[brw(args(2, "Military Fighters"))]
    version: Version,
    init: Bool,
    #[brw(if(version.version < 1))]
    settler_ref0: Option<Uuid>,
    #[brw(if(version.version < 1))]
    settler_ref1: Option<Uuid>,
    #[brw(if(version.version < 1))]
    building_ref: Option<Uuid>,
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
    #[brw(args(0, "Military FightConnections"))]
    version: Version,
    init: Bool,
    settler_ref0: Uuid,
    settler_ref1: Uuid,
    building_ref: Uuid,
}

#[binrw]
#[derive(Debug)]
struct Attack {
    #[brw(args(0, "Military Attack"))]
    version: Version,
}

#[binrw]
#[derive(Debug)]
struct Training {
    #[brw(args(0, "Military Training"))]
    version: Version,
    tick: u32, //0 - 300 (current tick mod 301)
}
