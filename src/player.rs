use crate::Version;

use crate::helper_structs::*;
use binrw::binrw;
use strum::*;

#[binrw]
#[derive(Debug)]
pub struct Players {
    version: Version!(0, "PlayerSystem"),
    #[brw(assert(init.bool))]
    init: Bool,
    pub players: [Optional<Player>; PlayerId::COUNT],
}

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
    tribe: u32,
    locksmith: LockSmith,
    good_priority: GoodPriorities,
    good_arrangement: GoodArrangement,
    military: PlayerMilitary,
    messages: Messages,
    stock: Stock,
    counter: u32,
    ship_names: ShipNames,
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
struct GoodPriorities {
    version: Version!(0, "Good Priorities"),
    #[brw(assert(init.bool))]
    init: Bool,
    settings: Array<GoodPriority>,
}

#[binrw]
#[derive(Debug)]
struct GoodPriority {
    version: Version!(0, "Good Priority"),
    idk: u32,
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
struct GoodArrangement {
    version: Version!(0, "Good Arrangement"),
    #[brw(assert(init.bool))]
    init: Bool,
    arrangementgroups: Array<GoodArrangementGroup>,
}

#[binrw]
#[derive(Debug)]
struct GoodArrangementGroup {
    version: Version!(0, "Player GoodArrangementGroup"),
    #[brw(assert(init.bool))]
    init: Bool,
    base_arrangement: ArrangementBase,
    good: Good,
}

#[binrw]
#[derive(Debug)]
struct ArrangementBase {
    version: Version!(0, "Player ArrangementBase"),
    arrangements: Array<ArrangementObject>,
}

#[binrw]
#[derive(Debug)]
struct ArrangementObject {
    version: Version!(0, "Player ArrangementObject"),
    percentage: f32,
    idk: f32,
    obj_type: u32,
}

#[binrw]
#[derive(Debug)]
struct PlayerMilitary {
    version: Version!(0, "Player Military"),
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
struct Messages {
    version: Version!(0, "Messages"),
    messages: Array<Message>,
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
    id: MsgId,
    more_info: Str,
}

#[binrw]
#[derive(Debug)]
struct MsgId {
    version: Version!(0, "uniqueId"),
    id: u64,
}

#[binrw]
#[derive(Debug)]
pub struct Stock {
    version: Version!(0, "Stock"),
    #[brw(assert(init.bool))]
    init: Bool,
    idk: u32,
    map: Array<(Good, u32)>,
}

#[binrw]
#[derive(Debug)]
struct ShipNames {
    version: Version!(0, "Player ShipNames"),
    ships: CapedU32<19>,
}
