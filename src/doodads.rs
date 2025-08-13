use crate::helper_structs::*;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Doodads {
    #[brw(args("DoodadsSystem"))]
    version: Version<0>,
    init: Bool,
    map1: Array<Doodad>,
    map2: Array<Doodad>,
    map3: Array<Doodad>,
}

#[binrw]
#[derive(Debug)]
struct Doodad {
    type_id: u32,
    #[brw(args("DoodadsObject"))] //TODO: why 1 and not 0?
    version: Version<1>,
    id: Uuid,
    pos: ElevationCursor,
    #[br(if(has_lifetime(type_id)))]
    #[bw(if(has_lifetime(*type_id)))]
    lifetime: Option<u32>,
}

fn has_lifetime(type_id: u32) -> bool {
    //aka is_sign
    matches!(
        type_id,
        0x28f42343
            | 0x121aa343
            | 0x90eeb793
            | 0xc5096143
            | 0x00ad6ff3
            | 0x45dbe563
            | 0xd33a52e3
            | 0x4b82f123
            | 0x96771ad3
            | 0xe06ac3a3
            | 0xe812d123
            | 0x3124a193
            | 0xaecf0d53
            | 0x17684773
    )
}
