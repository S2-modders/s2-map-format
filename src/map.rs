use crate::helper_structs::*;
use binrw::binrw;
use strum::*;

#[binrw]
#[derive(Debug)]
pub struct Map {
    #[brw(args("MapSystem"))]
    version: Version<0>,
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
    #[brw(args("ElevationMap"))]
    version: Version<1>,
    init: Bool,
    idk: u32,
    #[brw(if(version.version > 0))]
    width: u32,
    #[brw(if(version.version > 0))]
    height: u32,
    #[br(count = width*height)]
    #[brw(if(version.version > 0))]
    elevations: Vec<u32>,
    #[brw(if(version.version == 0))]
    elevations_old: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct PatternMap {
    #[brw(args("PatternMap"))]
    version: Version<0>,
    init: Bool,
    patterns: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct GridStatesMap {
    #[brw(args("GridStatesMap"))]
    version: Version<0>,
    init: Bool,
    gridstates: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct ResourceMap {
    #[brw(args("Map Resources"))]
    version: Version<0>,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width*height)]
    resources: Vec<(u32, i32)>,
}

#[binrw]
#[derive(Debug)]
struct TerritoryMap {
    #[brw(args("Map Territory"))]
    version: Version<0>,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    territories: Vec<u32>,
}

#[binrw]
#[derive(Debug)]
struct ExplorationMap {
    #[brw(args("Map Exploration"))] //TODO: why can this be 1?
    version: Version<1>,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    territories: [Vec<u32>; PlayerId::COUNT],
}

#[binrw]
#[derive(Debug)]
struct ContinentMap {
    #[brw(args("Map Continents"))]
    version: Version<1>,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    continentmap: Vec<u32>,
    condinentdata: Array<Continent>,
    #[brw(if(version.version > 0))]
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct Continent {
    #[brw(args("Map Continent"))]
    version: Version<3>,
    idk: u32,
    init: Bool,
    id: u32,
    #[brw(if(version.version > 0))]
    region: Option<(i32, i32, i32, i32)>,
    #[brw(if(version.version > 1))]
    poses: Array<PatternCursor>,
    #[brw(if(version.version > 2))]
    somevec: Array<u32>,
}
