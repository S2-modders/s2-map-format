use crate::helper_structs::*;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Military {
    #[brw(args("MilitarySystem"))]
    version: Version<0>,
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
    #[brw(args("Military Recruting"))]
    version: Version<0>,
    tick: u32, //0 - 100 (current tick mod 101)
}

#[binrw]
#[derive(Debug)]
struct Distances {
    #[brw(args("Military Distances"))]
    version: Version<0>,
    tick: u32, //0 - 10 (current tick mod 11)
}

#[binrw]
#[derive(Debug)]
struct Allocation {
    #[brw(args("Military Allocation"))]
    version: Version<0>,
    tick: u32, //0 - 20 (current tick mod 21)
}

#[binrw]
#[derive(Debug)]
struct Fight {
    #[brw(args("Military Fight"))]
    version: Version<1>,
    fighters: Array<Fighters>,
    fighters2: Array<Fighters>,
    #[brw(if(version.version > 0))]
    fighters3: Array<Fighters>,
}

#[binrw]
#[derive(Debug)]
struct Fighters {
    #[brw(args("Military Fighters"))]
    version: Version<2>,
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
    #[brw(args("Military FightConnections"))]
    version: Version<0>,
    init: Bool,
    settler_ref0: Uuid,
    settler_ref1: Uuid,
    building_ref: Uuid,
}

#[binrw]
#[derive(Debug)]
struct Attack {
    #[brw(args("Military Attack"))]
    version: Version<0>,
}

#[binrw]
#[derive(Debug)]
struct Training {
    #[brw(args("Military Training"))]
    version: Version<0>,
    tick: u32, //0 - 300 (current tick mod 301)
}
