use crate::helper_structs::*;
use binrw::binrw;
use strum::*;

#[binrw]
#[derive(Debug)]
pub struct Map {
    #[brw(args(0, "MapSystem"))]
    version: Version,
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
    #[brw(args(1, "ElevationMap"))]
    version: Version,
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
    #[brw(args(0, "PatternMap"))]
    version: Version,
    init: Bool,
    patterns: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct GridStatesMap {
    #[brw(args(0, "GridStatesMap"))]
    version: Version,
    init: Bool,
    gridstates: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct ResourceMap {
    #[brw(args(0, "Map Resources"))]
    version: Version,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width*height)]
    resources: Vec<(u32, i32)>,
}

#[binrw]
#[derive(Debug)]
struct TerritoryMap {
    #[brw(args(0, "Map Territory"))]
    version: Version,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    territories: Vec<u32>,
}

#[binrw]
#[derive(Debug)]
struct ExplorationMap {
    #[brw(args(1, "Map Exploration"))] //TODO: why can this be 1?
    version: Version,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    territories: [Vec<u32>; PlayerId::COUNT],
}

#[binrw]
#[derive(Debug)]
struct ContinentMap {
    #[brw(args(1, "Map Continents"))]
    version: Version,
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
#[derive(Debug, Default)]
struct Continent {
    #[brw(args(3, "Map Continent"))]
    version: Version,
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
