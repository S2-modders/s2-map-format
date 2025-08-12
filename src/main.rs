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
mod buildings;
use buildings::Villages;
mod movement;
use movement::SettlerMovement;

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
struct Settlers {
    #[brw(args(0, "SettlersSystem"))]
    version: Version,
    init: Bool,
    workers: Array<(PlayerId, Worker)>,
    constructor: Array<(PlayerId, Constructor)>,
    carrier: Array<(PlayerId, Carrier)>,
    bulldoser: Array<(PlayerId, Bulldozer)>,
    soldier: Array<(PlayerId, Soldier)>,
    specialist: Array<(PlayerId, Specialist)>,
}

#[binrw]
#[derive(Debug)]
struct Worker {
    #[brw(args(1, "SettlersWorker"))]
    version: Version,
    work_building_ref: Uuid,
    ship_ref: Uuid,
    test: [u32; 59], //TODO: filler -- decompiling goals takes too long; version 0 has less goals
    settler: Settler,
}

#[binrw]
#[derive(Debug)]
struct Constructor {
    #[brw(args(1, "SettlersConstructor"))]
    version: Version,
    #[br(dbg)]
    test: [u32; 30], //TODO: filler -- decompiling goals takes too long
    work_building_ref: Uuid,
    settler: Settler,
}

#[binrw]
#[derive(Debug)]
struct Carrier;

#[binrw]
#[derive(Debug)]
struct Bulldozer;

#[binrw]
#[derive(Debug)]
struct Soldier;

#[binrw]
#[derive(Debug)]
struct Specialist;

#[binrw]
#[derive(Debug)]
struct Settler {
    #[brw(args(0, "Settlers Settler"))]
    version: Version,
    id: Uuid,
    movement: SettlerMovement,
    animation: Animation,
    package_ref: Uuid,
    settler_type: u32,
    state: u32,
    test: [u32; 5], //TODO: filler -- decompiling goals takes too long

    building_ref: Uuid,
}

#[binrw]
#[derive(Debug)]
struct Animation {
    #[brw(args(1, "SettlersAnimation"))]
    version: Version,
    remaining_time: f32,
    #[brw(if(version.version == 1))]
    end_time: Option<f32>,
    animation_type: u32,
}

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
