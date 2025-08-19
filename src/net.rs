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
    pub flags: Array<Flag>,
    pub streets: Array<Street>,
    pub net_graph: [[SerializedGraph; NetType::COUNT]; PlayerId::COUNT],
}

pub type SerializedGraph = Array<(Ref<Flag>, Array<(Ref<Flag>, u32)>)>;

#[binrw]
#[derive(Debug)]
pub struct Flag {
    owner: PlayerId,
    version: Version!(1, "Flag"),
    building_ref: OptRef<Building>,
    id: Uuid,
    pos: PatternCursor,
    links: Array<Versioned!("Net FlagLink", Ref<Flag>, Ref<Street>)>,
    idk: [Ref<Flag>; NetType::COUNT],
    packages: VersionedI!("Package Container", Array<Ref<Package>>),
    specialist: Versioned!("Net Specialist", SettlersContainer),
    orders: Versioned!("Order Container", Array<Ref<Order>>),
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
    carrier0: OptRef<Carrier>,
    carrier1: OptRef<Carrier>,
    map_updater: Version!("Net Street Map Updater"),
    orders: Versioned!("Order Container", Array<Ref<Order>>),
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
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug, EnumCount)]
pub enum NetType {
    Ship = 0,
    Transport = 1, //Ware
    Seltter = 2,
}
