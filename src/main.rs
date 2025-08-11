use binrw::{BinRead, binrw};
use decryptor_s2::*;
use simple_eyre::eyre::Result;
use strum::*;
mod helper_structs;
use helper_structs::*;

fn main() -> Result<()> {
    simple_eyre::install()?;
    std::env::args().collect::<Vec<String>>()[1..]
        .iter()
        .try_for_each(|s| -> Result<()> {
            let reader = &mut std::io::Cursor::new(std::fs::read(s)?);
            let decompressed = CompressedFile::read_args(reader, (s,))?;
            assert!(
                matches!(decompressed.game, Game::Dng),
                "Map is from wrong game"
            );
            let reader = &mut std::io::Cursor::new(decompressed.data);
            let map = MapFile::read_le(reader)?;
            // let print = &map.logic.mapinfo;
            // println!("{print:?}");
            let print = &map.logic.trigger_sys.unwrap().triggers;
            println!("{print:?}");
            // let mut writer = std::io::Cursor::new(Vec::new());
            // map.write_le(&mut writer)?;
            // CompressedFile {
            //     data: writer.into_inner(),
            //     game: Game::Dng,
            // }
            // .write_args(&mut BufWriter::new(&mut File::open(s)?), (s,))?;
            let remaining = &reader.get_ref()[reader.position() as usize..];
            println!("remaining: {}", remaining.len());
            println!("type: {}", map.logic.mapinfo.file_type);
            println!("{:?}/{:?}", reader.position(), reader.get_ref().len());
            println!("remaining bytes (50): {:?}", &remaining[..50]);
            Ok(())
        })?;
    Ok(())
}

#[binrw]
#[derive(Debug)]
struct MapFile {
    logic: Logic,
    map: Map,
    resources: Resources,
    doodads: Doodads,
    ambients: Ambients,
    #[brw(if(logic.mapinfo.file_type == 20 || logic.mapinfo.file_type == 1))]
    gamefilelogic: Option<GameFileLogic>,
}

#[binrw]
#[derive(Debug)]
struct Logic {
    mapinfo: MapInfo,
    #[brw(args(7, "LogicSystem"))]
    version: Version,
    max_id: i64,
    initialized: Bool,
    #[brw(if(version.version> 0 && version.version < 6))]
    unused: u32,
    #[brw(if(version.version > 3))]
    seconds_per_tick: f32,
    #[brw(if(version.version > 3))]
    ticked_seconds: f32,
    #[brw(if(version.version > 3))]
    seconds_passed: f32,
    #[brw(if(version.version > 4))]
    trigger_sys: Option<TriggerSys>,
    #[brw(if(version.version > 6))]
    tick: i32,
}

#[binrw]
#[derive(Debug)]
struct MapInfo {
    #[brw(args(9, "MapInfo"))]
    version: Version,
    idk: Array<PatternCursor>,
    map_name: Str,
    #[brw(if(version.version > 1))]
    width: u32,
    #[brw(if(version.version > 1))]
    height: u32,
    idk2: [u32; PlayerId::COUNT],
    #[brw(if(version.version > 2 && version.version < 6))]
    idk3: [(u32, u32, u32); PlayerId::COUNT],
    #[brw(if(version.version > 5))]
    idk3_2: [(u32, u32, u32, u32); PlayerId::COUNT],
    #[brw(if(version.version > 0))]
    mission_target_type: Option<u32>,
    #[brw(if(version.version > 3))]
    idk4: u32,
    #[brw(if(version.version > 4))]
    file_type: u32,
    #[brw(if(version.version > 6))]
    id: Option<CoreUuid>,
    #[brw(if(version.version > 7))]
    idk5: Option<Bool>,
    #[brw(if(version.version > 7))]
    player_names: Option<[Str; PlayerId::COUNT]>,
    #[brw(if(version.version > 8))]
    idk6: Option<u32>,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
enum MissionTarget {
    DestroyAllEnemies = 1,
    ConquerTheMap = 2,
    ProduceCoins = 3,
    ReachThePortal = 4,
}

#[binrw]
#[derive(Debug)]
struct TriggerSys {
    #[brw(args(0, "TriggerSystem"))]
    version: Version,
    init: Bool,
    triggers: Array<Trigger>,
}

#[binrw]
#[derive(Debug)]
struct Trigger {
    #[brw(args(1, "TriggerObject"))]
    version: Version,
    init: Bool,
    uuid: Uuid,
    trigger_type: TriggerType,
    pos: PatternCursor,
    idk: u32,
    active: Bool,
    name: Str,
    player_id: PlayerId,
    #[brw(if(version.version > 0))]
    time: f32,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
enum TriggerType {
    Type1 = 1,
    Type2 = 2,
    Win = 3,
}

impl Trigger {
    pub fn new(
        logic: &mut Logic,
        trigger_type: TriggerType,
        pos: (u32, u32),
        idk: u32,
        name: &str,
        player_id: PlayerId,
    ) -> Trigger {
        Trigger {
            version: 1.into(),
            init: true.into(),
            uuid: Uuid::new(logic),
            active: true.into(),
            time: 0.0,
            trigger_type,
            pos: pos.into(),
            idk,
            name: name.into(),
            player_id,
        }
    }
}

#[binrw]
#[derive(Debug)]
struct Map {
    #[brw(args(0, "MapSystem"))]
    version: Version,
    init: Bool,
    width: u32,
    height: u32,
    elevation_map: ElevationMap,
    pattern_map: PatternMap,
    gird_state_map: GridStatesMap,
    resource_map: ResourceMap,
    territory_map: TerritoryMap,
    exploration_map: ExplorationMap,
    contient_map: ContinentMap,
}

#[binrw]
#[derive(Debug)]
struct ElevationMap {
    #[brw(args(1, "ElevationMap"))]
    version: Version,
    init: Bool,
    idk: u32,
    #[brw(if(version.version > 0))]
    width: u32,
    #[brw(if(version.version > 0))]
    height: u32,
    #[br(count = width*height)]
    #[brw(if(version.version > 0))]
    elevations: Vec<u32>,
    #[brw(if(version.version == 0))]
    elevations_old: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct PatternMap {
    #[brw(args(0, "PatternMap"))]
    version: Version,
    init: Bool,
    patterns: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct GridStatesMap {
    #[brw(args(0, "GridStatesMap"))]
    version: Version,
    init: Bool,
    gridstates: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct ResourceMap {
    #[brw(args(0, "Map Resources"))]
    version: Version,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width*height)]
    resources: Vec<(u32, i32)>,
}

#[binrw]
#[derive(Debug)]
struct TerritoryMap {
    #[brw(args(0, "Map Territory"))]
    version: Version,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    territories: Vec<u32>,
}

#[binrw]
#[derive(Debug)]
struct ExplorationMap {
    #[brw(args(0, "Map Exploration"))]
    version: Version,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    territories: [Vec<u32>; PlayerId::COUNT],
}

#[binrw]
#[derive(Debug)]
struct ContinentMap {
    #[brw(args(1, "Map Continents"))]
    version: Version,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    continentmap: Vec<u32>,
    condinentdata: Array<Continent>,
    #[brw(if(version.version > 0))]
    idk: u32,
}

#[binrw]
#[derive(Debug, Default)]
struct Continent {
    #[brw(args(3, "Map Continent"))]
    version: Version,
    idk: u32,
    init: Bool,
    id: u32,
    #[brw(if(version.version > 0))]
    region: Option<(i32, i32, i32, i32)>,
    #[brw(if(version.version > 1))]
    poses: Array<PatternCursor>,
    #[brw(if(version.version > 2))]
    somevec: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct Resources {
    #[brw(args(4, "resources"))]
    version: Version,
    init: Bool,
    deposits: Array<(u32, Deposit)>,

    #[brw(if(version.version > 0))]
    animals: Array<Animal>,
    #[brw(if(version.version > 2))]
    respawn: Option<AnimalRespawn>,
    #[brw(if(version.version > 3))]
    idk: u32,
    #[brw(if(version.version > 3))]
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
struct AnimalRespawn {
    #[brw(args(0, "Resources AnimalRespawn"))]
    version: Version,
    init: Bool,
    tick: u32,
    inc: u32,
    pos: UPos, //TODO
}

#[binrw]
#[derive(Debug)]
struct UPos {
    x: u32,
    y: u32,
}

#[binrw]
#[derive(Debug)]
struct Deposit {
    #[brw(args(1, "deposit"))]
    version: Version,
    id: Uuid,
    pos: PatternCursor,
    buildingref: Uuid,
    pos2: ElevationCursor,
    current_grouth: f32,
    #[brw(if(version.version > 0))]
    age: u32,
    #[brw(if(version.version > 0))]
    life_time: u32,
}

#[binrw]
#[derive(Debug, Default)]
struct Animal {
    mapkey: u32,
    #[brw(args(2, "Resources Animal"))]
    version: Version,
    id: Uuid,
    idk: f32,
    pos: PatternCursor,
    movement: AnimalMovement,
    idk1: u32,
    #[brw(if(version.version > 0))]
    idk2: u32,
    #[brw(if(version.version > 1))]
    villagebuildingref: Uuid,
}

#[binrw]
#[derive(Debug, Default)]
struct AnimalMovement {
    #[brw(args(1, "Navy Movement"))]
    version: Version,
    path: ResourcePath,
    #[brw(if(version.version > 0))]
    pos: Option<PatternCursor>, //Get this instead in movementBase.pos if None
    movement_base: MovementBase,
}

#[binrw]
#[derive(Debug, Default)]
struct MovementPathBase {
    #[brw(args(1, "Movement Path Base"))]
    version: Version,
    init: Bool,
    #[brw(if (init.bool || version.version == 0))]
    poses: Array<PatternCursor>,
    #[brw(if (init.bool || version.version == 0))]
    idk: Bool,
    #[brw(if (init.bool || version.version == 0))]
    idk1: i32,
    #[brw(if (init.bool || version.version == 0))]
    idk2: Bool,
}

#[binrw]
#[derive(Debug, Default)]
struct ResourcePath {
    #[brw(args(0, "Resources Path"))]
    version: Version,
    base: MovementPathBase,
}

#[binrw]
#[derive(Debug, Default)]
struct MovementBase {
    #[brw(args(0, "Movement Base"))]
    version: Version,
    pos: PatternCursor,
    idk: PatternCursor,
    idk1: PatternCursor,
    interpolator: MovementInterpolator,
}

#[binrw]
#[derive(Debug, Default)]
struct MovementInterpolator {
    #[brw(args(0, "Movement Interpolator"))]
    version: Version,
    idk1: f32,
    idk2: f32,
    idk3: f32,
}

#[binrw]
#[derive(Debug)]
struct Doodads {
    #[brw(args(0, "DoodadsSystem"))]
    version: Version,
    init: Bool,
    map1: Array<Doodad>,
    map2: Array<Doodad>,
    map3: Array<Doodad>,
}

#[binrw]
#[derive(Debug)]
struct Doodad {
    type_id: u32,
    #[brw(args(1, "DoodadsObject"))] //TODO: why 1 and not 0?
    version: Version,
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

#[binrw]
#[derive(Debug)]
struct Ambients {
    #[brw(args(0, "Logic Ambients"))]
    version: Version,
    init: Bool,
    ambients: Array<Ambient>,
}

#[binrw]
#[derive(Debug)]
struct Ambient {
    idk: u32,
    pos: PatternCursor,
}

#[binrw]
#[derive(Debug)]
struct GameFileLogic {
    #[brw(args(2, "Game File Logic"))]
    version: Version,
    #[brw(if(version.version > 0))]
    random: Option<Random>,
    players: Players,
    villages: Villages,
    settlers: Settlers,
    transport_sys: TransportSys,
    military: Military,
    navy: Navy,
    netsys: NetSys,
    ai: Ai,
    stats: Stats,
    #[brw(if(version.version > 1))]
    game_script: Option<GameScript>,
}

#[binrw]
#[derive(Debug)]
struct Random {
    #[brw(args(0, "Logic Random"))]
    version: Version,
    init: Bool,
    state: u64,
}

#[binrw]
#[derive(Debug)]
struct Players {
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

#[binrw]
#[derive(Debug)]
struct Villages;

#[binrw]
#[derive(Debug)]
struct Settlers;

#[binrw]
#[derive(Debug)]
struct TransportSys;

#[binrw]
#[derive(Debug)]
struct Military;

#[binrw]
#[derive(Debug)]
struct Navy;

#[binrw]
#[derive(Debug)]
struct NetSys;

#[binrw]
#[derive(Debug)]
struct Ai;

#[binrw]
#[derive(Debug)]
struct Stats;

#[binrw]
#[derive(Debug)]
struct GameScript {
    version: Version,
    idk: Bool,
    map_name: Str,
    persistent: Array<(Str, u32)>,
}
