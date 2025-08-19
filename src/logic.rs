use crate::VersionI;
use crate::VersionedI;
use crate::player::Player;
use std::time::Duration;

use crate::Version;
use crate::ai::Ai;
use crate::buildings::Villages;
use crate::doodads::Doodads;
use crate::helper_structs::*;
use crate::map::MapSys;
use crate::military::Military;
use crate::navy::Ship;
use crate::net::NetSys;
use crate::resources::Resources;
use crate::settlers::Settlers;
use crate::transport::Transport;
use binrw::binrw;
use nonmax::NonMaxU64;
use strum::EnumCount;

#[binrw]
#[derive(Debug)]
pub struct MapFile {
    pub mapinfo: MapInfo,
    pub logic: Logic,
    pub map: MapSys,
    pub resources: Resources,
    pub doodads: Doodads,
    pub ambients: VersionedI!("Logic Ambients", Array<(AmbientType, PatternCursor)>),
    #[brw(if(mapinfo.file_type == FileType::SaveGame))]
    pub save_file_info: Option<GameFileLogic>,
}

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum AmbientType {
    Beach = 0x5bdc4873,
    Desert1 = 0xdf5602f3,
    Desert2 = 0x67ef3d13,
    Desert3 = 0x31f23a23,
    Forest1 = 0x89f59a23,
    Forest2 = 0x118e5363,
    Meadow1 = 0x5aaad2d3,
    Meadow2 = 0x623437d3,
    Water1 = 0xf35757d3,
    Water2 = 0x3b68a763,
    Water3 = 0x00a67113,
    Water4 = 0x875d51f3,
    Lava = 0xa952bba3,
}

#[binrw]
#[derive(Debug)]
pub struct GameFileLogic {
    version: Version!(2, "Game File Logic"),
    pub random: VersionedI!("Logic Random", u64),
    pub players: VersionedI!("PlayerSystem", [Optional<Player>; PlayerId::COUNT]),
    pub villages: Villages,
    pub settlers: Settlers,
    pub transport: Transport,
    pub military: Military,
    pub navy: VersionedI!(2, "Navy System", Array<Ship>),
    pub netsys: NetSys,
    #[brw(args(&players.data))]
    pub ai: Ai,
    pub stats: Stats,
    pub game_script: GameScript,
}

#[binrw]
#[derive(Debug)]
pub struct Stats {
    version: Version!("LogicStatistics"),
    idk: u32,
    stats: Array<(Uuid, u32, f32, u32)>, //TODO idk
    player_stats: [PlayerStats; PlayerId::COUNT],
}

#[binrw]
#[derive(Debug)]
struct PlayerStats {
    version: Version!(2, "LogicPlayerStatistics"),
    stats: [Array<u32>; PlayerId::COUNT],
    stats2: [Array<u32>; 14],
    idk: u32,
    died_soldiers: u32,
    territory: u32,
}

#[binrw]
#[derive(Debug)]
pub struct GameScript {
    version: VersionI!("GameScript"),
    map_name: Str,
    lua_persistent: Array<(Str, u32)>,
}

#[binrw]
#[derive(Debug)]
pub struct Logic {
    pub version: Version!(7, "LogicSystem"),
    pub uuid_generator: UuidGenerator,
    #[brw(assert(init.bool))]
    pub init: Bool,
    pub duration_between_ticks: Time,
    pub time_ticked: Time,
    pub time_passed: Time,
    pub trigger_sys: VersionedI!("TriggerSystem", Array<Trigger>),
    pub tick: i32,
}

#[binrw]
#[derive(Debug)]
pub struct UuidGenerator {
    #[br(try_map = |x:u64| x.try_into())]
    #[bw(map = |x| x.get())]
    state: NonMaxU64,
}

impl UuidGenerator {
    pub fn next_id(&mut self) -> Uuid {
        let res = self.state.into();
        self.state = NonMaxU64::new(self.state.get() + 1).unwrap(); //unwrap_or_default()
        res
    }
}

#[binrw]
#[derive(Debug)]
pub struct MapInfo {
    pub version: Version!(9, "MapInfo"),
    pub start_positions: Array<PatternCursor>,
    pub map_name: Str,
    pub dimensions: (u32, u32),
    pub player_types: [PlayerType; PlayerId::COUNT],
    pub idk3: [(u32, PlayerId, i32, u32); PlayerId::COUNT],
    pub mission_target_type: OptNone<MissionTarget>,
    pub idk4: u32,
    pub file_type: FileType,
    pub id: CoreUuid,
    pub idk5: Bool,
    pub player_names: [Str; PlayerId::COUNT],
    pub idk6: u32,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum PlayerType {
    None = 0,
    Ai = 1,
    Player = 2,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug, Default, PartialEq, Eq)]
pub enum FileType {
    #[default]
    None = 0,
    SaveGame = 1,
    Map = 10,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
pub enum MissionTarget {
    DestroyAllEnemies = 1,
    ConquerTheMap = 2,
    ProduceCoins = 3,
    ReachThePortal = 4,
}

#[binrw]
#[derive(Debug)]
pub struct Trigger {
    version: VersionI!(1, "TriggerObject"),
    id: Uuid,
    trigger_type: TriggerType,
    pos: PatternCursor,
    idk: u32,
    active: Bool,
    name: Str,
    owner: PlayerId,
    time: Time,
}

impl Ided for Trigger {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
pub enum TriggerType {
    Type1 = 1,
    Type2 = 2,
    Win = 3,
}

impl Trigger {
    pub fn new(
        id_generator: &mut UuidGenerator,
        trigger_type: TriggerType,
        pos: (u32, u32),
        idk: u32,
        name: &str,
        owner: PlayerId,
    ) -> Trigger {
        Trigger {
            version: Default::default(),
            id: id_generator.next_id(),
            active: true.into(),
            time: Time {
                duration: Duration::default(),
            },
            trigger_type,
            pos: pos.into(),
            idk,
            name: name.into(),
            owner,
        }
    }
}
