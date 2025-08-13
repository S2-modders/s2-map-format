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
mod settlers;
use settlers::Settlers;
mod transport;
use transport::Transport;
mod military;
use military::Military;

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
    #[brw(args("Logic Ambients"))]
    version: Version<0>,
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
    #[brw(args("Game File Logic"))]
    version: Version<2>,
    #[brw(if(version.version > 0))]
    random: Option<Random>,
    players: Players,
    villages: Villages,
    settlers: Settlers,
    transport: Transport,
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
    #[brw(args("Logic Random"))]
    version: Version<0>,
    init: Bool,
    state: u64,
}

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
    #[brw(args("GameScript"))]
    version: Version<0>,
    idk: Bool,
    map_name: Str,
    persistent: Array<(Str, u32)>,
}
