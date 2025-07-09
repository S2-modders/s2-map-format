use binrw::{binrw, BinRead};
use simple_eyre::eyre::Result;

fn main() -> Result<()> {
    simple_eyre::install()?;
    std::env::args().collect::<Vec<String>>()[1..]
        .iter()
        .try_for_each(|s| -> Result<()> {
            let reader = &mut std::io::Cursor::new(std::fs::read(s)?);
            let map = MapFile::read_le(reader)?;
            let print = map.ambients.ambients;
            println!("{print:?}");
            println!("{:?}/{:?}", reader.position(), reader.get_ref().len());
            Ok(())
        })?;
    Ok(())
}

#[binrw]
#[derive(Debug)]
struct Str {
    #[br(temp)]
    #[bw(calc = array.len() as u32)]
    len: u32,
    #[br(count = len)]
    array: Vec<u8>,
}

#[binrw]
#[derive(Debug, Default)]
struct Bool {
    // #[br(map = |x: u32| x != 0)]
    // #[bw(map = |x: &bool| *x as u32)]
    bool: u32,
}

#[binrw]
#[derive(Debug)]
struct ArrayAmbient {
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<Ambient>,
}

#[binrw]
#[derive(Debug)]
struct ArrayDoodad {
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<Doodad>,
}

#[binrw]
#[derive(Debug)]
struct ArrayAnimal {
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<Animal>,
}

#[binrw]
#[derive(Debug)]
struct ArrayDeposit {
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<Deposit>,
}

#[binrw]
#[derive(Debug)]
struct ArrayGoodCount {
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<(u32, i32)>,
}

#[binrw]
#[derive(Debug)]
struct ArrayU32 {
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<u32>,
}

#[binrw]
#[derive(Debug)]
struct ArrayTrigger {
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<Trigger>,
}

#[binrw]
#[derive(Debug, Default)]
struct ArrayPatternCursor {
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<PatternCursor>,
}

#[binrw]
#[derive(Debug)]
struct ArrayContinent {
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<Continent>,
}

#[binrw]
#[derive(Debug)]
struct ArrayPos {
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<Pos>,
}

#[binrw]
#[derive(Debug)]
struct Hash {
    // #[br(assert(hash_type < 20))]
    hash_type: u32,
    hash: u32,
    // #[br(assert(len < 25))]
    len: u32,
}

#[binrw]
#[derive(Debug)]
struct MapFile {
    logic: Logic,
    map: Map,
    resources: Resources,
    doodads: Doodads,
    ambients: Ambients,
    // gamefilelogic: GameFileLogic,
}

#[binrw]
#[derive(Debug)]
struct Logic {
    mapinfo: MapInfo,
    hash: Hash,
    counter: u64,
    initialized: Bool,
    seconds_per_tick: f32,
    ticked_seconds: f32,
    seconds_passed: f32,
    trigger_sys: TriggerSys,
    tick: i32,
}

#[binrw]
#[derive(Debug)]
struct MapInfo {
    hash: Hash,
    idk: ArrayPatternCursor,
    map_name: Str,
    width: u32,
    height: u32,
    idk2: [u32; 8],
    idk3: [(u32, u32, u32, u32); 8],
    mission_target_type: u32,
    idk4: u32,
    file_type: u32,
    id: CoreUuid,
    //if some bool
    idk5: Bool,
    player_names: [Str; 8],
}

#[binrw]
#[derive(Debug)]
struct CoreUuid {
    hash: Hash,
    init: Bool,
    id: u128,
}

#[binrw]
#[derive(Debug)]
struct ElevationCursor {
    hash: Hash,
    idk: u32,
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
struct PatternCursor {
    hash: Hash,
    x: u32,
    y: u32,
}

#[binrw]
#[derive(Debug)]
struct TriggerSys {
    hash: Hash,
    init: Bool,
    trigger: ArrayTrigger,
}

#[binrw]
#[derive(Debug)]
struct Trigger {
    hash: Hash,
    init: Bool,
    uuid: Uuid,
    trigger_type: u32,
    pos: Pos,
    idk: u32,
    active: Bool,
    name: Str,
    player_id: u32,
    time: f32,
}

#[binrw]
#[derive(Debug)]
struct Uuid {
    hash: Hash,
    id: i64,
}

#[binrw]
#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[binrw]
#[derive(Debug)]
struct Map {
    hash: Hash,
    init: Bool,
    width: u32,
    height: u32,
    elevation_map: ElevationMap,
    pattern_map: PatternMap,
    gird_state_map: GridStateMap,
    resource_map: ResourceMap,
    territory_map: TerritoryMap,
    exploration_map: ExplorationMap,
    contient_map: ContinentMap,
}

#[binrw]
#[derive(Debug)]
struct ElevationMap {
    hash: Hash,
    init: Bool,
    idk: u32,
    width: u32,
    height: u32,
    #[br(count = width*height)]
    elevations: Vec<u32>,
}

#[binrw]
#[derive(Debug)]
struct PatternMap {
    hash: Hash,
    init: Bool,
    patterns: ArrayU32,
}

#[binrw]
#[derive(Debug)]
struct GridStateMap {
    hash: Hash,
    init: Bool,
    gridstates: ArrayU32,
}

#[binrw]
#[derive(Debug)]
struct ResourceMap {
    hash: Hash,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width*height)]
    resources: Vec<(u32, i32)>,
}

#[binrw]
#[derive(Debug)]
struct TerritoryMap {
    hash: Hash,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    territories: Vec<u32>,
}

#[binrw]
#[derive(Debug)]
struct ExplorationMap {
    hash: Hash,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    territories: [Vec<u32>; 8],
}

#[binrw]
#[derive(Debug)]
struct ContinentMap {
    hash: Hash,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    continentmap: Vec<u32>,
    condinentdata: ArrayContinent,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct Continent {
    hash: Hash,
    idk: u32,
    init: Bool,
    id: u32,
    region: (i32, i32, i32, i32),
    poses: ArrayPatternCursor,
    somevec: ArrayU32,
}

#[binrw]
#[derive(Debug)]
struct Resources {
    hash: Hash,
    init: Bool,
    deposits: ArrayDeposit,
    animals: ArrayAnimal,
    respawn: AnimalRespawn,
    idk: u32,
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
struct AnimalRespawn {
    hash: Hash,
    init: Bool,
    tick: u32,
    inc: u32,
    pos: UPos,
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
    property_id: i32,
    hash: Hash,
    id: Uuid,
    pos: PatternCursor,
    buildingref: Uuid,
    pos2: ElevationCursor,
    current_grouth: f32,
    age: u32,
    life_time: u32,
}

#[binrw]
#[derive(Debug)]
struct Animal {
    mapkey: u32,
    hash: Hash,
    id: Uuid,
    idk: f32,
    patterncursor: PatternCursor,
    movement: AnimalMovement,
    idk1: u32,
    idk2: u32,
    villagebuildingref: Uuid,
}

#[binrw]
#[derive(Debug)]
struct AnimalMovement {
    hash: Hash,
    path: ResourcePath,
    pattern_cursor: PatternCursor,
    movement_base: MovementBase,
}

#[binrw]
#[derive(Debug)]
struct ResourcePathBase {
    hash: Hash,
    init: Bool,
    #[brw(if (init.bool == 1))]
    poses: ArrayPatternCursor,
    // #[brw(if (init.bool == 1))]
    // idk: Bool,
    // #[brw(if (init.bool == 1))]
    // idk1: i32,
    // #[brw(if (init.bool == 1))]
    // idk2: Bool,
}

#[binrw]
#[derive(Debug)]
struct ResourcePath {
    hash: Hash,
    base: ResourcePathBase,
}

#[binrw]
#[derive(Debug)]
struct MovementBase {
    hash: Hash,
    pos: PatternCursor,
    idk: PatternCursor,
    idk1: PatternCursor,
    interpolator: MovementInterpolator,
}

#[binrw]
#[derive(Debug)]
struct MovementInterpolator {
    hash: Hash,
    idk1: f32,
    idk2: f32,
    idk3: f32,
}

#[binrw]
#[derive(Debug)]
struct Doodads {
    hash: Hash,
    init: Bool,
    map1: ArrayDoodad,
    map2: ArrayDoodad,
    map3: ArrayDoodad,
}

#[binrw]
#[derive(Debug)]
struct Doodad {
    type_id: u32,
    hash: Hash,
    id: Uuid,
    pos: ElevationCursor,
    #[br(if(has_lifetime(type_id)))]
    #[bw(if(has_lifetime(*type_id)))]
    lifetime: u32,
}

fn has_lifetime(type_id: u32) -> bool {
    //aka is sign
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
    hash: Hash,
    init: Bool,
    ambients: ArrayAmbient,
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
    hash: Hash,
    random: Random,
    players: Players,
    villages: Villages,
    settlers: Settlers,
    transport_sys: TransportSys,
    military: Military,
    navy: Navy,
    netsys: NetSys,
    ai: Ai,
    stats: Stats,
    // game_script: GameScript,
}

#[binrw]
#[derive(Debug)]
struct Random;

#[binrw]
#[derive(Debug)]
struct Players;

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
    hash: Hash,
    idk: Bool,
    map_name: Str,
    persistent: MapStringu32,
}

#[binrw]
#[derive(Debug)]
struct MapStringu32;
