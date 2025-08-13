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
    #[brw(args("Net System"))]
    version: Version<1>,
    init: Bool,
    flags: Array<(PlayerId, Flag)>,
    streets: Array<Street>,
    idk: [Array<Idk>; PlayerId::COUNT],
}

#[binrw]
#[derive(Debug)]
pub struct Flag {
    #[brw(args("Flag"))]
    version: Version<1>, //TODO: idk why
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
    #[brw(args("Net FlagLink"))]
    version: Version<0>,
    flag_ref: Ref<Flag>,
    street_ref: Ref<Street>,
}

#[binrw]
#[derive(Debug)]
struct PackageContainer {
    #[brw(args("Package Container"))]
    version: Version<0>,
    init: Bool,
    packages: Array<Ref<Package>>,
}

#[binrw]
#[derive(Debug)]
struct Specialist {
    #[brw(args("Net Specialist"))]
    version: Version<0>,
    settlers: SettlersContainer,
}

#[binrw]
#[derive(Debug)]
pub struct Street {
    owner: PlayerId,
    #[brw(magic = 0x43d79823u32)]
    #[brw(args("NetStreet"))]
    version: Version<1>,// TODO: why?
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
    idk: [Bool; 3],
}

impl Ided for Street {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct MapUpdater {
    #[brw(args("Net Street Map Updater"))]
    version: Version<0>,
}

#[binrw]
#[derive(Debug)]
struct Idk(Uuid, Array<(Uuid, u32)>);
