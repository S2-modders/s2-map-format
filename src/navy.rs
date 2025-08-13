use crate::helper_structs::*;
use crate::{buildings::Building, movement::NavyPath, player::Stock};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Navy {
    #[brw(args("Navy System"))]
    version: Version<2>, //TODO: why 2?
    init: Bool,
    ships: Array<(PlayerId, Ship)>,
}

#[binrw]
#[derive(Debug)]
pub struct Ship {
    #[brw(args("Navy Ship"))]
    version: Version<3>,
    id: Uuid,
    construction: Construction,
    path: NavyPath,
    stock: Stock,
    building_ref: Ref<Building>,
    idk: u32,
    expedition: Expedition,
    building_ref2: Ref<Building>,
    #[brw(if(version.version > 0))]
    receivers: Array<ShipReceiver>,
    #[brw(if(version.version > 1))]
    pos: Option<PatternCursor>,
    #[brw(if(version.version > 2))]
    ship_name: Option<CapedU32<19>>,
}

impl Ided for Ship {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct Construction {
    #[brw(args("NavyConstruction"))]
    version: Version<1>,
    progress: f32,
    frame_progress: f32,
    pos: PatternCursor,
}

#[binrw]
#[derive(Debug)]
struct Expedition {
    #[brw(args("Navy Expedtion"))]
    version: Version<0>,
    building_ref: Ref<Building>,
    pos: PatternCursor,
    idk: Bool,
    possible_targets: [PatternCursor; 6],
}

#[binrw]
#[derive(Debug)]
struct ShipReceiver {
    #[brw(args("Navy ShipReceiver"))]
    version: Version<0>,
    receivers: Array<(Good, BuildingType)>,
    building_ref: Ref<Building>,
}
