use crate::Version;
use crate::VersionI;
use crate::Versioned;
use crate::VersionedI;
use crate::buildings::Order;

use crate::helper_structs::*;
use crate::{
    buildings::{Building, SettlersContainer},
    settlers::Carrier,
    transport::Package,
};
use binrw::binrw;
use strum::EnumCount;

#[binrw]
#[derive(Debug)]
pub struct NetSys {
    version: VersionI!(1, "Net System"),
    flags: Array<(PlayerId, Flag)>,
    streets: Array<Street>,
    idk: [[Array<Idk>; 3]; PlayerId::COUNT],
}

#[binrw]
#[derive(Debug)]
pub struct Flag {
    version: Version!(1, "Flag"), //TODO: idk why
    building_ref: Ref<Building>,
    id: Uuid,
    pos: PatternCursor,
    links: Array<Versioned!(0, "Net FlagLink", Ref<Flag>, Ref<Street>)>,
    idk: [Uuid; 3],
    packages: VersionedI!(0, "Package Container", Array<Ref<Package>>),
    specialist: Versioned!(0, "Net Specialist", SettlersContainer),
    orders: Versioned!(0, "Order Container", Array<Ref<Order>>),
    idk2: u32,
}

impl Ided for Flag {
    fn id(&self) -> Uuid {
        self.id
    }
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
    map_updater: Version!(0, "Net Street Map Updater"),
    orders: Versioned!(0, "Order Container", Array<Ref<Order>>),
    transported_good_count: u32,
    ticks: CapedU32<6000>,
    stone_score: f32,
    is_stone: Bool,
    is_waterway: Bool,
    idk: Bool,
    has_second_carrier: Bool,
    is_not_waterway: Bool,
}

impl Ided for Street {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct Idk(Uuid, Array<(Uuid, u32)>);
