use crate::Version;
use crate::VersionI;
use crate::VersionedI;
use crate::helper_structs::*;
use bilge::prelude::*;
use bilge::{DebugBits, bitsize};
use binrw::binrw;
use strum::*;

#[binrw]
#[derive(Debug)]
pub struct Map {
    version: VersionI!("MapSystem"),
    width: u32,
    height: u32,
    elevation_map: ElevationMap,
    pattern_map: VersionedI!("PatternMap", Array<PatternType>),
    gird_state_map: VersionedI!("GridStatesMap", Array<GridStates>),
    resource_map: ResourceMap,
    territory_map: TerritoryMap,
    exploration_map: ExplorationMap,
    contient_map: ContinentMap,
}

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum PatternType {
    PatternBorder = 0x76d31873,
    PatternWater = 0xfe6bd1b3,
    PatternAcre = 0xca56701a,
    PatternMeadow = 0xbfe4e8e3,
    PatternMeadow1 = 0x4545fac1,
    PatternMeadow2 = 0x4545fac2,
    PatternMeadow3 = 0x4545fac3,
    PatternMeadow4 = 0x4545fac4,
    PatternMeadow5 = 0x4545fac5,
    PatternMeadow6 = 0x4545fac6,
    PatternMeadow7 = 0x4545fac7,
    PatternMeadow9 = 0x4545fac9,
    PatternSand = 0xbadeb00d,
    PatternSand1 = 0xbadeb00e,
    PatternSand2 = 0xbadeb00f,
    PatternSand3 = 0xbadeb010,
    PatternSand4 = 0xbadeb011,
    PatternSand5 = 0xbadeb012,
    PatternSand6 = 0xbadeb013,
    PatternRock = 0xd00faffe,
    PatternRock1 = 0xdeadbeef,
    PatternRock2 = 0xcafecafe,
    PatternRock3 = 0xcafecaff,
    PatternRock4 = 0xcafecb00,
    PatternRock5 = 0xcafecb01,
    PatternRock6 = 0xcafecb02,
    PatternRock7 = 0xcafecb03,
    PatternRock8 = 0xcafecb04,
    PatternRock9 = 0xcafecb05,
    PatternSnow = 0xfade0ff,
    PatternSeaground = 0xbabeb00b,
    PatternSeaground1 = 0x13374e4,
    PatternSeaground2 = 0x13374e5,
    PatternSeaground3 = 0x13374e6,
    PatternSeaground4 = 0x13374e7,
    PatternSeaground5 = 0x13374e8,
    PatternSwamp = 0x680004e4,
    PatternSwamp1 = 0x680004e5,
    PatternSwamp2 = 0x680004e6,
    PatternPavement = 0xdecade01,
    PatternGround = 0xde5e1110,
    PatternEarth1 = 0x777fa8c0,
    PatternLGround00 = 0xdecade02,
    PatternLGround01 = 0xdecade03,
    PatternLRock00 = 0xdecade04,
    PatternLRock01 = 0xdecade05,
    PatternLRock02 = 0xdecade06,
    PatternLRock03 = 0xca87fab0,
    PatternLGround02 = 0xdecade07,
    PatternLGround03 = 0xdecade08,
    PatternLGround04 = 0xdecade09,
    PatternLGround05 = 0xdecade0a,
    PatternLSand00 = 0xf1cabb70,
    PatternLMeadow00 = 0xf67adb70,
    PatternMMeadow00 = 0xfa1ca560,
    PatternMMeadow01 = 0xfa1ca561,
    PatternMMeadow02 = 0xfa1ca562,
    PatternMMeadow03 = 0xfa1ca563,
    PatternMGround00 = 0xfa1ca570,
    PatternMGround01 = 0xfa1ca571,
    PatternMRock00 = 0xfa1ca580,
    PatternMRock01 = 0xfa1ca581,
    PatternMRock02 = 0xfa1ca582,
    PatternMRock03 = 0xfa1ca583,
    PatternMRock04 = 0xfa1ca584,
    PatternMRock05 = 0xfa1ca585,
    PatternMRock06 = 0xfa1ca586,
    PatternMRock07 = 0xfa1ca587,
    PatternMRock08 = 0xfa1ca588,
    PatternMRock09 = 0xfa1ca589,
    PatternMRock10 = 0xfa1ca58a,
    PatternMSeaground00 = 0xfa1ca590,
    PatternMSeaground01 = 0xfa1ca591,
}

#[binrw]
#[derive(Debug)]
struct ElevationMap {
    version: VersionI!(1, "ElevationMap"),
    min_elevation: i32,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    elevations: Vec<i32>,
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
    version: VersionI!("Map Resources"),
    width: u32,
    height: u32,
    #[br(count = width*height)]
    resources: Vec<(u32, Good)>,
}

#[binrw]
#[derive(Debug)]
struct TerritoryMap {
    version: VersionI!("Map Territory"),
    width: u32,
    height: u32,
    #[br(count = width * height)]
    territories: Vec<Optional<PlayerId>>,
}

#[binrw]
#[derive(Debug)]
struct ExplorationMap {
    version: VersionI!(1, "Map Exploration"),
    width: u32,
    height: u32,
    #[br(count = width * height)]
    explored: [Vec<Bool>; PlayerId::COUNT],
}

#[binrw]
#[derive(Debug)]
struct ContinentMap {
    version: VersionI!(1, "Map Continents"),
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
