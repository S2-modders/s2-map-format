use crate::Version;
use crate::VersionI;
use crate::helper_structs::*;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Doodads {
    version: VersionI!(0, "DoodadsSystem"),
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
    #[brw(if(doodad_type.has_lifetime()))]
    lifetime: Option<u32>,
}

impl Ided for Doodad {
    fn id(&self) -> Uuid {
        self.id
    }
}
