use crate::Version;

use crate::helper_structs::*;
use crate::{
    buildings::{Building, OrderContainer, SettlersContainer},
    settlers::Carrier,
    transport::Package,
};
use binrw::binrw;
use strum::EnumCount;

#[binrw]
#[derive(Debug)]
pub struct NetSys {
    version: Version!(1, "Net System"),
    init: Bool,
    flags: Array<(PlayerId, Flag)>,
    streets: Array<Street>,
    #[brw(if(version.version > 0))]
    idk: [[Array<Idk>; 3]; PlayerId::COUNT],
}

#[binrw]
#[derive(Debug)]
pub struct Flag {
    version: Version!(1, "Flag"), //TODO: idk why
    building_ref: Ref<Building>,
    id: Uuid,
    pos: PatternCursor,
    links: Array<FlagLink>,
    idk: [Uuid; 3],
    packages: PackageContainer,
    specialist: Specialist,
    orders: OrderContainer,
    idk2: u32,
}

impl Ided for Flag {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct FlagLink {
    version: Version!(0, "Net FlagLink"),
    flag_ref: Ref<Flag>,
    street_ref: Ref<Street>,
}

#[binrw]
#[derive(Debug)]
struct PackageContainer {
    version: Version!(0, "Package Container"),
    init: Bool,
    packages: Array<Ref<Package>>,
}

#[binrw]
#[derive(Debug)]
struct Specialist {
    version: Version!(0, "Net Specialist"),
    settlers: SettlersContainer,
}

#[binrw]
#[derive(Debug)]
pub struct Street {
    owner: PlayerId,
    #[brw(magic = 0x43d79823u32)]
    version: Version!(1, "NetStreet"),
    id: Uuid,
    poses: Array<PatternCursor>,
    segments: u32,
    start: Ref<Flag>,
    end: Ref<Flag>,
    carrier0: Ref<Carrier>,
    carrier1: Ref<Carrier>,
    map_updater: MapUpdater,
    orders: OrderContainer,
    transported_good_count: u32,
    ticks: CapedU32<6000>,
    stone_score: f32,
    is_stone: Bool,
    is_waterway: Bool,
    #[brw(if(version.version > 0))]
    idk: Option<Bool>,
    #[brw(if(version.version > 0))]
    has_second_carrier: Option<Bool>,
    #[brw(if(version.version > 0))]
    is_not_waterway: Option<Bool>,
}

impl Ided for Street {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct MapUpdater {
    version: Version!(0, "Net Street Map Updater"),
}

#[binrw]
#[derive(Debug)]
struct Idk(Uuid, Array<(Uuid, u32)>);
