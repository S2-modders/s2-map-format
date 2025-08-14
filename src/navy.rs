use crate::Version;
use crate::helper_structs::*;
use crate::{buildings::Building, movement::NavyPath, player::Stock};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Navy {
    version: Version!(2, "Navy System"),
    init: Bool,
    ships: Array<(PlayerId, Ship)>,
}

#[binrw]
#[derive(Debug)]
pub struct Ship {
    version: Version!(3, "Navy Ship"),
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
    version: Version!(1, "NavyConstruction"),
    progress: f32,
    frame_progress: f32,
    pos: PatternCursor,
}

#[binrw]
#[derive(Debug)]
struct Expedition {
    version: Version!(0, "Navy Expedtion"),
    building_ref: Ref<Building>,
    pos: PatternCursor,
    idk: Bool,
    possible_targets: [PatternCursor; 6],
}

#[binrw]
#[derive(Debug)]
struct ShipReceiver {
    version: Version!(0, "Navy ShipReceiver"),
    receivers: Array<(Good, BuildingType)>,
    building_ref: Ref<Building>,
}
