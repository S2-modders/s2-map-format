use binrw::{binrw, BinRead};
use simple_eyre::eyre::Result;

fn main() -> Result<()> {
    simple_eyre::install()?;
    std::env::args().collect::<Vec<String>>()[1..]
        .iter()
        .try_for_each(|s| -> Result<()> {
            let reader = &mut std::io::Cursor::new(std::fs::read(s)?);
            let map = MapFile::read_le(reader)?;
            println!("{map:?}");
            println!("{:?}", reader.position());
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
#[derive(Debug)]
struct Bool {
    // #[br(map = |x: u32| x != 0)]
    // #[bw(map = |x: &bool| *x as u32)]
    bool: u32,
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
#[derive(Debug)]
struct ArrayPatternCursor {
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<PatternCursor>,
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
    hash_type: u32,
    hash: u32,
    len: u32,
}

#[binrw]
#[derive(Debug)]
struct MapFile {
    logic: Logic,
    map: Map,
    // resources: Resources,
    // doodas: Doodas,
    // ambients: Ambients,
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
struct PatternCursor {
    hash: Hash,
    idk: u32,
    idk2: u32,
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
    idk: u32,
    width: u32,
    height: u32,
    #[br(count = width*height)]
    resources: Vec<(u32, i32)>,
}

#[binrw]
#[derive(Debug)]
struct TerritoryMap;

#[binrw]
#[derive(Debug)]
struct ExplorationMap;

#[binrw]
#[derive(Debug)]
struct ContinentMap;

#[binrw]
#[derive(Debug)]
struct Resources;

#[binrw]
#[derive(Debug)]
struct Doodas;

#[binrw]
#[derive(Debug)]
struct Ambients;

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
    game_script: GameScript,
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
