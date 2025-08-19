use crate::Version;
use crate::VersionI;
use crate::VersionedI;
use crate::helper_structs::*;
use bilge::prelude::*;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct MapSys {
    version: VersionI!("MapSystem"),
    pub width: u32,
    pub height: u32,
    elevation_version: VersionI!(1, "ElevationMap"),
    pub min_elevation: i32,
    pub elevations: Map<i32>,
    pub pattern_map: VersionedI!("PatternMap", Array<PatternType>),
    pub gird_state_map: VersionedI!("GridStatesMap", Array<GridStates>),
    pub resource_map: VersionedI!("Map Resources", Map<(CapedU32<7>, Optional<Good>)>),
    pub territory_map: VersionedI!("Map Territory", Map<Optional<PlayerId>>),
    pub exploration_map: VersionedI!(1, "Map Exploration", PlayerMap<Bool>),
    continent_version: VersionI!(1, "Map Continents"),
    pub continentid_map: Map<ContinentId>,
    pub continentdata: Array<Continent>,
    total_continent_tiles: u32,
}

pub type ContinentId = u32;

#[binrw]
#[derive(Debug)]
pub struct Continent {
    version: Version!(3, "Map Continent"),
    continent_tiles: u32,
    //TODO is it really an init field?
    init: Bool,
    id: ContinentId,
    region: (u32, u32, u32, u32),
    poses: Array<PatternCursor>,
    adjacent_continents: Array<ContinentId>,
}

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum PatternType {
    Border = 0x76d31873,
    Water = 0xfe6bd1b3,
    Acre = 0xca56701a,
    Meadow = 0xbfe4e8e3,
    Meadow1 = 0x4545fac1,
    Meadow2 = 0x4545fac2,
    Meadow3 = 0x4545fac3,
    Meadow4 = 0x4545fac4,
    Meadow5 = 0x4545fac5,
    Meadow6 = 0x4545fac6,
    Meadow7 = 0x4545fac7,
    Meadow9 = 0x4545fac9,
    Sand = 0xbadeb00d,
    Sand1 = 0xbadeb00e,
    Sand2 = 0xbadeb00f,
    Sand3 = 0xbadeb010,
    Sand4 = 0xbadeb011,
    Sand5 = 0xbadeb012,
    Sand6 = 0xbadeb013,
    Rock = 0xd00faffe,
    Rock1 = 0xdeadbeef,
    Rock2 = 0xcafecafe,
    Rock3 = 0xcafecaff,
    Rock4 = 0xcafecb00,
    Rock5 = 0xcafecb01,
    Rock6 = 0xcafecb02,
    Rock7 = 0xcafecb03,
    Rock8 = 0xcafecb04,
    Rock9 = 0xcafecb05,
    Snow = 0xfade0ff,
    Seaground = 0xbabeb00b,
    Seaground1 = 0x13374e4,
    Seaground2 = 0x13374e5,
    Seaground3 = 0x13374e6,
    Seaground4 = 0x13374e7,
    Seaground5 = 0x13374e8,
    Swamp = 0x680004e4,
    Swamp1 = 0x680004e5,
    Swamp2 = 0x680004e6,
    Pavement = 0xdecade01,
    Ground = 0xde5e1110,
    Earth1 = 0x777fa8c0,
    LGround00 = 0xdecade02,
    LGround01 = 0xdecade03,
    LRock00 = 0xdecade04,
    LRock01 = 0xdecade05,
    LRock02 = 0xdecade06,
    LRock03 = 0xca87fab0,
    LGround02 = 0xdecade07,
    LGround03 = 0xdecade08,
    LGround04 = 0xdecade09,
    LGround05 = 0xdecade0a,
    LSand00 = 0xf1cabb70,
    LMeadow00 = 0xf67adb70,
    MMeadow00 = 0xfa1ca560,
    MMeadow01 = 0xfa1ca561,
    MMeadow02 = 0xfa1ca562,
    MMeadow03 = 0xfa1ca563,
    MGround00 = 0xfa1ca570,
    MGround01 = 0xfa1ca571,
    MRock00 = 0xfa1ca580,
    MRock01 = 0xfa1ca581,
    MRock02 = 0xfa1ca582,
    MRock03 = 0xfa1ca583,
    MRock04 = 0xfa1ca584,
    MRock05 = 0xfa1ca585,
    MRock06 = 0xfa1ca586,
    MRock07 = 0xfa1ca587,
    MRock08 = 0xfa1ca588,
    MRock09 = 0xfa1ca589,
    MRock10 = 0xfa1ca58a,
    MSeaground00 = 0xfa1ca590,
    MSeaground01 = 0xfa1ca591,
}

#[bitsize(32)]
#[binrw]
#[derive(DebugBits)]
pub struct GridStates {
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
