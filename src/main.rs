use std::fmt;

use binrw::{binrw, BinRead, BinWrite};
use decryptor_s2::*;
use simple_eyre::eyre::Result;

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
            let print = &map.logic.trigger_sys.triggers;
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
            println!("{:?}/{:?}", reader.position(), reader.get_ref().len());
            Ok(())
        })?;
    Ok(())
}

#[binrw]
#[derive(Default)]
struct Str {
    #[br(temp)]
    #[bw(calc = str.len() as u32)]
    len: u32,
    #[br(count = len)]
    #[br(try_map = String::from_utf8)]
    #[bw(map = String::as_bytes)]
    str: String,
}

impl fmt::Debug for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.str.fmt(f)
    }
}

#[binrw]
#[derive(Default)]
struct Bool {
    #[br(try_map = |x: u32| (x < 2).then_some(x != 0).ok_or("expected bool"))]
    #[bw(map = |x| *x as u32)]
    bool: bool,
}

impl fmt::Debug for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.bool.fmt(f)
    }
}

#[binrw]
#[derive(Debug, Default)]
struct Array<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug + 'static,
{
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    array: Vec<T>,
}

#[binrw]
struct Uuid {
    #[brw(args("logic UniqueId"))]
    #[br(assert(version.version == 0))]
    version: Version,
    id: i64,
}

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}


#[binrw]
#[derive(Default)]
#[brw(import(name:&str))]
struct Version {
    version: u32,
    #[br(assert(hash == crc32fast::hash(name.as_bytes())))]
    #[bw(calc = crc32fast::hash(name.as_bytes()))]
    hash: u32,
    #[br(assert(len as usize == name.len()))]
    #[bw(calc = name.len() as u32)]
    len: u32,
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.version.fmt(f)
    }
}

#[binrw]
struct CoreUuid {
    #[brw(args("Core UUID"))]
    #[brw(assert(version.version == 0))]
    version: Version,
    init: Bool,
    id: u128,
}

impl fmt::Debug for CoreUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}


#[binrw]
struct ElevationCursor {
    #[brw(args("ElevationCursor"))]
    #[brw(assert(version.version == 0))]
    version: Version,
    x: u32,
    y: u32,
}

impl fmt::Debug for ElevationCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.x, self.y).fmt(f)
    }
}


#[binrw]
#[derive(Default)]
struct PatternCursor {
    #[brw(args("PatternCursor"))]
    #[brw(assert(version.version == 0))]
    version: Version,
    x: u32,
    y: u32,
}

impl fmt::Debug for PatternCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.x, self.y).fmt(f)
    }
}



#[binrw]
#[derive(Debug)]
struct MapFile {
    logic: Logic,
    map: Map,
    resources: Resources,
    doodads: Doodads,
    ambients: Ambients,
    #[brw(if(logic.mapinfo.file_type == 0x14))]
    gamefilelogic: GameFileLogic,
}

#[binrw]
#[derive(Debug)]
struct Logic {
    mapinfo: MapInfo,
    #[brw(args("LogicSystem"))]
    version: Version,
    max_id: u64,
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
    #[brw(args("MapInfo"))]
    version: Version,
    idk: Array<PatternCursor>,
    map_name: Str,
    width: u32,
    height: u32,
    idk2: [u32; 8],
    idk3: [(u32, u32, u32, u32); 8],
    mission_target_type: u32,
    idk4: u32,
    file_type: u32,
    id: CoreUuid,
    #[brw(if(version.version > 7))]
    idk5: Bool,
    #[brw(if(version.version > 7))]
    player_names: [Str; 8],
    #[brw(if(version.version > 8))]
    idk6: u32,
}

#[binrw]
#[derive(Debug)]
struct TriggerSys {
    #[brw(args("TriggerSystem"))]
    version: Version,
    init: Bool,
    triggers: Array<Trigger>,
}

#[binrw]
#[derive(Debug)]
struct Trigger {
    #[brw(args("TriggerObject"))]
    version: Version,
    init: Bool,
    uuid: Uuid,
    trigger_type: u32,
    pos: PatternCursor,
    idk: u32,
    active: Bool,
    name: Str,
    player_id: u32,
    time: f32,
}

#[binrw]
#[derive(Debug)]
struct Map {
    #[brw(args("MapSystem"))]
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
    #[brw(args("ElevationMap"))]
    version: Version,
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
    #[brw(args("PatternMap"))]
    version: Version,
    init: Bool,
    patterns: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct GridStatesMap {
    #[brw(args("GridStatesMap"))]
    version: Version,
    init: Bool,
    gridstates: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct ResourceMap {
    #[brw(args("Map Resources"))]
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
    #[brw(args("Map Territory"))]
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
    #[brw(args("Map Exploration"))]
    version: Version,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    territories: [Vec<u32>; 8],
}

#[binrw]
#[derive(Debug)]
struct ContinentMap {
    #[brw(args("Map Continents"))]
    version: Version,
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    continentmap: Vec<u32>,
    condinentdata: Array<Continent>,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct Continent {
    #[brw(args("Map Continent"))]
    version: Version,
    idk: u32,
    init: Bool,
    id: u32,
    region: (i32, i32, i32, i32),
    poses: Array<PatternCursor>,
    somevec: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct Resources {
    #[brw(args("resources"))]
    version: Version,
    init: Bool,
    deposits: Array<(u32, Deposit)>,
    animals: Array<Animal>,
    respawn: AnimalRespawn,
    idk: u32,
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
struct AnimalRespawn {
    #[brw(args("Resources AnimalRespawn"))]
    version: Version,
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
    #[brw(args("deposit"))]
    version: Version,
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
    #[brw(args("Resources Animal"))]
    version: Version,
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
    #[brw(args("Navy Movement"))]
    version: Version,
    path: ResourcePath,
    pattern_cursor: PatternCursor,
    movement_base: MovementBase,
}

#[binrw]
#[derive(Debug, Default)]
struct ResourcePathBase {
    #[brw(args("Movement Path Base"))]
    version: Version,
    init: Bool,
    #[brw(if (init.bool))]
    poses: Array<PatternCursor>,
    #[brw(if (init.bool))]
    idk: Bool,
    #[brw(if (init.bool))]
    idk1: i32,
    #[brw(if (init.bool))]
    idk2: Bool,
}

#[binrw]
#[derive(Debug)]
struct ResourcePath {
    #[brw(args("Resources Path"))]
    version: Version,
    base: ResourcePathBase,
}

#[binrw]
#[derive(Debug)]
struct MovementBase {
    #[brw(args("Movement Base"))]
    version: Version,
    pos: PatternCursor,
    idk: PatternCursor,
    idk1: PatternCursor,
    interpolator: MovementInterpolator,
}

#[binrw]
#[derive(Debug)]
struct MovementInterpolator {
    #[brw(args("Movement Interpolator"))]
    version: Version,
    idk1: f32,
    idk2: f32,
    idk3: f32,
}

#[binrw]
#[derive(Debug)]
struct Doodads {
    #[brw(args("DoodadsSystem"))]
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
    #[brw(args("DoodadsObject"))]
    version: Version,
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
    #[brw(args("Logic Ambients"))]
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
#[derive(Debug, Default)]
struct GameFileLogic {
    #[brw(args("Game File Logic"))]
    version: Version,
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
#[derive(Debug, Default)]
struct Random;

#[binrw]
#[derive(Debug, Default)]
struct Players;

#[binrw]
#[derive(Debug, Default)]
struct Villages;

#[binrw]
#[derive(Debug, Default)]
struct Settlers;

#[binrw]
#[derive(Debug, Default)]
struct TransportSys;

#[binrw]
#[derive(Debug, Default)]
struct Military;

#[binrw]
#[derive(Debug, Default)]
struct Navy;

#[binrw]
#[derive(Debug, Default)]
struct NetSys;

#[binrw]
#[derive(Debug, Default)]
struct Ai;

#[binrw]
#[derive(Debug, Default)]
struct Stats;

#[binrw]
#[derive(Debug, Default)]
struct GameScript {
    version: Version,
    idk: Bool,
    map_name: Str,
    persistent: MapStringu32,
}

#[binrw]
#[derive(Debug, Default)]
struct MapStringu32;
