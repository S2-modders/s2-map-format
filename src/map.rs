use crate::Version;
use crate::helper_structs::*;
use bilge::prelude::*;
use bilge::{DebugBits, bitsize};
use binrw::binrw;
use strum::*;

#[binrw]
#[derive(Debug)]
pub struct Map {
    version: Version!(0, "MapSystem"),
    #[brw(assert(init.bool))]
    init: Bool,
    width: u32,
    height: u32,
    elevation_map: ElevationMap,
    pattern_map: PatternMap,
    gird_state_map: GridStatesMap,
    resource_map: ResourceMap,
    territory_map: TerritoryMap,
    exploration_map: ExplorationMap,
    contient_map: ContinentMap,
}

#[binrw]
#[derive(Debug)]
struct ElevationMap {
    version: Version!(1, "ElevationMap"),
    #[brw(assert(init.bool))]
    init: Bool,
    min_elevation: i32,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    elevations: Vec<i32>,
}

#[binrw]
#[derive(Debug)]
struct PatternMap {
    version: Version!(0, "PatternMap"),
    #[brw(assert(init.bool))]
    init: Bool,
    patterns: Array<PatternType>,
}

#[binrw]
#[derive(Debug)]
struct GridStatesMap {
    version: Version!(0, "GridStatesMap"),
    #[brw(assert(init.bool))]
    init: Bool,
    gridstates: Array<GridStates>,
}

#[bitsize(32)]
#[binrw]
#[derive(DebugBits)]
struct GridStates {
    is_blocked: bool,
    idk1: bool,
    is_water: bool,
    idk3: bool,
    is_for_mining: bool,
    idk5: bool,
    idk6: bool,
    has_deposit: bool,
    idk8: bool,
    is_for_building: bool,
    idk10: bool,
    something_ship: bool,
    idk12: bool,
    is_for_harbor: bool,
    idk14: bool,
    idk15: bool,
    idk16: bool,
    idk17: bool,
    idk18: bool,
    idk19: bool,
    idk20: bool,
    idk21: bool,
    idk22: bool,
    idk23: bool,
    idk24: bool,
    idk25: bool,
    idk26: bool,
    idk27: bool,
    idk28: bool,
    idk29: bool,
    idk30: bool,
    idk31: bool,
}

#[binrw]
#[derive(Debug)]
struct ResourceMap {
    version: Version!(0, "Map Resources"),
    #[brw(assert(init.bool))]
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width*height)]
    resources: Vec<(u32, Good)>,
}

#[binrw]
#[derive(Debug)]
struct TerritoryMap {
    version: Version!(0, "Map Territory"),
    #[brw(assert(init.bool))]
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    territories: Vec<Optional<PlayerId>>,
}

#[binrw]
#[derive(Debug)]
struct ExplorationMap {
    version: Version!(1, "Map Exploration"),
    #[brw(assert(init.bool))]
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    exploration: [Vec<u32>; PlayerId::COUNT],
}

#[binrw]
#[derive(Debug)]
struct ContinentMap {
    version: Version!(1, "Map Continents"),
    #[brw(assert(init.bool))]
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    continentmap: Vec<u32>,
    continentdata: Array<Continent>,
    total_continent_tiles: u32,
}

#[binrw]
#[derive(Debug)]
struct Continent {
    version: Version!(3, "Map Continent"),
    continent_tiles: u32,
    //TODO is it really an init field?
    init: Bool,
    id: u32,
    region: (u32, u32, u32, u32),
    poses: Array<PatternCursor>,
    adjacent_continents: Array<u32>,
}
