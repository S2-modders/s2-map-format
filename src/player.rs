use crate::helper_structs::*;
use binrw::binrw;
use strum::*;

#[binrw]
#[derive(Debug)]
pub struct Players {
    #[brw(args(0, "PlayerSystem"))]
    version: Version,
    init: Bool,
    players: [OptionalPlayer; PlayerId::COUNT],
}

#[binrw]
#[derive(Debug)]
struct OptionalPlayer {
    tmp: i32,
    #[br(if(tmp != -1))]
    #[bw(if(*tmp != -1))]
    player: Option<Player>,
}

#[binrw]
#[derive(Debug)]
struct Player {
    #[brw(args(5, "PlayerObject"))]
    version: Version,
    init: Bool,
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
    #[brw(if(version.version > 0))]
    ship_names: Option<ShipNames>,
    #[brw(if(version.version > 1))]
    idk4: u32,
    #[brw(if(version.version > 2 && version.version < 5))]
    seen_old: Option<[u32; PlayerId::COUNT]>, //Seen by
    #[brw(if(version.version > 4))]
    seen: Option<[(u32, u32); PlayerId::COUNT]>, //Seen by and Seen
    #[brw(if(version.version > 3))]
    stock2: Option<Stock>,
}

#[binrw]
#[derive(Debug)]
struct LockSmith {
    #[brw(args(1, "PlayerLocksmith"))]
    version: Version,
    settings: [(f32, f32); 12],
    #[brw(if(version.version > 0))]
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct GoodPriorities {
    #[brw(args(0, "Good Priorities"))]
    version: Version,
    init: Bool,
    settings: Array<GoodPriority>,
}

#[binrw]
#[derive(Debug)]
struct GoodPriority {
    #[brw(args(0, "Good Priority"))]
    version: Version,
    idk: u32,
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
struct GoodArrangement {
    #[brw(args(0, "Good Arrangement"))]
    version: Version,
    init: Bool,
    arrangementgroups: Array<GoodArrangementGroup>,
}

#[binrw]
#[derive(Debug)]
struct GoodArrangementGroup {
    #[brw(args(0, "Player GoodArrangementGroup"))]
    version: Version,
    init: Bool,
    base_arrangement: ArrangementBase,
    good: Good,
}

#[binrw]
#[derive(Debug)]
struct ArrangementBase {
    #[brw(args(0, "Player ArrangementBase"))]
    version: Version,
    arrangements: Array<ArrangementObject>,
}

#[binrw]
#[derive(Debug)]
struct ArrangementObject {
    #[brw(args(0, "Player ArrangementObject"))]
    version: Version,
    percentage: f32,
    idk: f32,
    obj_type: u32,
}

#[binrw]
#[derive(Debug)]
struct PlayerMilitary {
    #[brw(args(0, "Player Military"))]
    version: Version,
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
    #[brw(args(0, "Messages"))]
    version: Version,
    messages: Array<Message>,
}

#[binrw]
#[derive(Debug)]
struct Message {
    #[brw(args(2, "Message"))]
    version: Version,
    idk: f32,
    pos: PatternCursor,
    msg: Str,
    desc: Str,
    msg_type: u32,
    idk2: u32,
    #[brw(if(version.version < 2))]
    id_old: Option<Uuid>,
    #[brw(if(version.version > 1))]
    id: Option<MsgId>,
    #[brw(if(version.version > 0))]
    more_info: Str,
}

#[binrw]
#[derive(Debug)]
struct MsgId {
    #[brw(args(0, "uniqueId"))]
    version: Version,
    id: u64,
}

#[binrw]
#[derive(Debug)]
struct Stock {
    #[brw(args(0, "Stock"))]
    version: Version,
    init: Bool,
    idk: u32,
    map: Array<(Good, BuildingType)>,
}

#[binrw]
#[derive(Debug)]
struct ShipNames {
    #[brw(args(0, "Player ShipNames"))]
    version: Version,
    ships: u32,
}
