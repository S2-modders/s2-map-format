use crate::Version;
use crate::VersionI;
use crate::Versioned;
use crate::VersionedI;

use crate::helper_structs::*;
use binrw::binrw;
use strum::*;

#[binrw]
#[derive(Debug)]
pub struct Player {
    id2: PlayerId,
    version: Version!(5, "PlayerObject"),
    #[brw(assert(init.bool))]
    pub init: Bool,
    name: Str,
    id: PlayerId,
    idk: u32,
    idk2: u32,
    tribe: Tribe,
    locksmith: LockSmith,
    good_priority: VersionedI!("Good Priorities", Array<GoodPriority>),
    good_arrangement: VersionedI!("Good Arrangement", Array<GoodArrangementGroup>),
    military: PlayerMilitary,
    messages: Versioned!("Messages", Array<Message>),
    stock: Stock,
    counter: u32,
    /// contains the current number the ship_names are on (the numbers are translated to names)
    ship_names: Versioned!("Player ShipNames", CapedU32<19>),
    idk4: u32,
    seen: [(u32, u32); PlayerId::COUNT], //Seen by and Seen
    stock2: Stock,
}

#[binrw]
#[derive(Debug)]
struct LockSmith {
    version: Version!(1, "PlayerLocksmith"),
    settings: [(f32, f32); 12],
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct GoodPriority {
    version: Version!("Good Priority"),
    good: GoodOrSettler,
    transport_priority: u32,
}

#[binrw]
#[derive(Debug)]
struct GoodArrangementGroup {
    version: VersionI!("Player GoodArrangementGroup"),
    base_arrangement: Versioned!("Player ArrangementBase", Array<ArrangementObject>),
    good: Good,
}

#[binrw]
#[derive(Debug)]
struct ArrangementObject {
    version: Version!("Player ArrangementObject"),
    percentage: f32,
    idk: f32,
    obj_type: BuildingType,
}

#[binrw]
#[derive(Debug)]
struct PlayerMilitary {
    version: Version!("Player Military"),
    idk0: f32,
    idk1: f32,
    idk2: f32,
    catapultscorediv: f32,
    attackstrengh: f32,
    is_attacker_save: Bool,
    intercepting_factor: f32,
}

#[binrw]
#[derive(Debug)]
struct Message {
    version: Version!(2, "Message"),
    idk: f32,
    pos: OptionalPatternCursor,
    msg: Str,
    desc: Str,
    msg_type: u32,
    idk2: u32,
    id: MsgId, //TODO
    more_info: Str,
}

#[binrw]
#[derive(Debug)]
struct MsgId {
    version: Version!("uniqueId"),
    id: u64, //TODO Nonmax
}

#[binrw]
#[derive(Debug)]
pub struct Stock {
    version: VersionI!("Stock"),
    idk: u32,
    map: Array<(GoodOrSettler, u32)>,
}
