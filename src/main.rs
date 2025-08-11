use binrw::{BinRead, binrw};
use decryptor_s2::*;
use simple_eyre::eyre::Result;
mod map;
use map::Map;
mod helper_structs;
use helper_structs::*;
mod logic;
use logic::Logic;
mod resources;
use resources::Resources;
mod doodads;
use doodads::Doodads;
mod player;
use player::Players;

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
struct Villages {
    #[brw(args(0, "VllageSystem"))]
    version: Version,
    init: Bool,
    buildings: Array<(BuildingType, PlayerId, Building)>,
    remains: Array<(RemainsType, BuildingType, Remains)>, //maybe in wrong order
    orders: Orders,
}

#[binrw]
#[derive(Debug)]
struct Building {
    #[brw(args(7, "VllageBuilding"))]
    version: Version,
    init: Bool,
    pos: PatternCursor,
    ticker: Ticker,
    depot: Depot,
    workers: Workers,
    idk: IDK,
    #[brw(if(version.version > 3))]
    idk2: Option<IDK2>,
    construction: Construction,
    production: Production,
    blocking: Blocking,
    idk3: u32,
    tribe: u32,
    flag_ref: Uuid,
    settler_spawn: SettlerSpawn,
    idk4: IDK3,
    territory_updater: TerritoryUpdater,
    bulldozing: Bulldozing,
    order: OrderContainer,
    idk5: Bool,
    #[brw(if(version.version > 6))] //or if is military building
    military: Option<PlayerMilitary>,
    carrier_refresh: CarrierRefresh,
    good_flags: GoodFlags,
    idk6: u32,
    tick: u32,
    #[brw(if(version.version > 6))] //or if is a catapult building
    catapult: Option<Catapult>,
    #[brw(if(version.version > 1))] //and it is a harbor
    harbor: Option<Harbor>,
    #[brw(if(version.version > 4))]
    upgrade: Option<Upgrade>,
}

#[binrw]
#[derive(Debug)]
struct Ticker {
    #[brw(args(0, "ticker"))]
    version: Version,
    curr_tick: u32,
    target_tick: u32,
}

#[binrw]
#[derive(Debug)]
struct Depot;

#[binrw]
#[derive(Debug)]
struct Workers;

#[binrw]
#[derive(Debug)]
struct IDK;

#[binrw]
#[derive(Debug)]
struct IDK2;

#[binrw]
#[derive(Debug)]
struct Construction;

#[binrw]
#[derive(Debug)]
struct Production;

#[binrw]
#[derive(Debug)]
struct SettlerSpawn;

#[binrw]
#[derive(Debug)]
struct IDK3;

#[binrw]
#[derive(Debug)]
struct TerritoryUpdater;

#[binrw]
#[derive(Debug)]
struct Bulldozing;

#[binrw]
#[derive(Debug)]
struct OrderContainer;

#[binrw]
#[derive(Debug)]
struct PlayerMilitary;

#[binrw]
#[derive(Debug)]
struct CarrierRefresh;

#[binrw]
#[derive(Debug)]
struct GoodFlags;

#[binrw]
#[derive(Debug)]
struct Catapult;

#[binrw]
#[derive(Debug)]
struct Harbor;

#[binrw]
#[derive(Debug)]
struct Upgrade;

#[binrw]
#[derive(Debug)]
struct Remains {
    #[brw(args(1, "VillageRemains"))]
    version: Version,
    init: Bool,
    pos: PatternCursor,
    someproperty: u32,
    blocking: Blocking,
    #[brw(if(version.version > 0))]
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct Blocking {
    #[brw(args(0, "Blocking"))]
    version: Version,
    init: Bool,
    pos: PatternCursor,
    size: u32,
}

#[binrw]
#[derive(Debug)]
struct Orders {
    #[brw(args(0, "Order System"))]
    version: Version,
    init: Bool,
    orders: Array<Order>,
    idk: i32,
}

#[binrw]
#[derive(Debug)]
struct Order {
    #[brw(args(3, "Village Order"))]
    version: Version,
    id: Uuid,
    #[brw(if(version.version < 3))]
    unused: u32,
    ordered: Good,
    #[brw(if(version.version > 0))]
    used: Bool,
    building_ref: Uuid,
    flag_ref: Uuid,
    street_ref: Uuid,
    #[brw(if(version.version > 1))]
    building_ref2: Option<Uuid>,
}

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
