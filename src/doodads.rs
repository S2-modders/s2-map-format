use crate::Version;
use crate::helper_structs::*;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Doodads {
    version: Version!(0, "DoodadsSystem"),
    init: Bool,
    map1: Array<Doodad>,
    map2: Array<Doodad>,
    map3: Array<Doodad>,
}

#[binrw]
#[derive(Debug)]
struct Doodad {
    doodad_type: DoodadType,
    version: Version!(1, "DoodadsObject"),
    id: Uuid,
    pos: ElevationCursor,
    #[brw(if(version.version == 0))]
    pos2: Option<ElevationCursor>,
    #[br(if(version.version == 0 || has_lifetime(doodad_type)))]
    #[bw(if(version.version == 0 || has_lifetime(*doodad_type)))]
    lifetime: Option<u32>,
}
