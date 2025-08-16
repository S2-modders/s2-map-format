use std::time::Duration;

use crate::Version;
use crate::ai::Ai;
use crate::buildings::Villages;
use crate::doodads::Doodads;
use crate::helper_structs::*;
use crate::map::Map;
use crate::military::Military;
use crate::navy::Navy;
use crate::net::NetSys;
use crate::player::Players;
use crate::resources::Resources;
use crate::settlers::Settlers;
use crate::transport::Transport;
use binrw::binrw;
use strum::EnumCount;

#[binrw]
#[derive(Debug)]
pub struct MapFile {
    pub mapinfo: MapInfo,
    pub logic: Logic,
    pub map: Map,
    pub resources: Resources,
    pub doodads: Doodads,
    pub ambients: Ambients,
    #[brw(if(mapinfo.file_type == FileType::SaveGame))]
    pub gamefilelogic: Option<GameFileLogic>,
}
#[binrw]
#[derive(Debug)]
pub struct Ambients {
    version: Version!(0, "Logic Ambients"),
    init: Bool,
    ambients: Array<(AmbientType, PatternCursor)>,
}

#[binrw]
#[derive(Debug)]
pub struct GameFileLogic {
    version: Version!(2, "Game File Logic"),
    #[brw(if(version.version > 0))]
    pub random: Option<Random>,
    pub players: Players,
    pub villages: Villages,
    pub settlers: Settlers,
    pub transport: Transport,
    pub military: Military,
    pub navy: Navy,
    pub netsys: NetSys,
    #[brw(args(&players.players))]
    pub ai: Ai,
    pub stats: Stats,
    #[brw(if(version.version > 1))]
    pub game_script: Option<GameScript>,
}

#[binrw]
#[derive(Debug)]
pub struct Random {
    version: Version!(0, "Logic Random"),
    init: Bool,
    state: u64,
}

#[binrw]
#[derive(Debug)]
pub struct Stats {
    version: Version!(0, "LogicStatistics"),
    idk: u32,
    stats: Array<(Uuid, u32, f32, u32)>,
    player_stats: [PlayerStats; PlayerId::COUNT],
}

#[binrw]
#[derive(Debug)]
struct PlayerStats {
    version: Version!(2, "LogicPlayerStatistics"),
    stats: [Array<u32>; PlayerId::COUNT],
    stats2: [Array<u32>; 14],
    idk: u32,
    #[brw(if(version.version > 0))]
    died_soldiers: u32,
    #[brw(if(version.version > 1))]
    territory: u32,
}

#[binrw]
#[derive(Debug)]
pub struct GameScript {
    version: Version!(0, "GameScript"),
    init: Bool,
    map_name: Str,
    persistent: Array<(Str, u32)>,
}

#[binrw]
#[derive(Debug)]
pub struct Logic {
    pub version: Version!(7, "LogicSystem"),
    pub uuid_generator: UuidGenerator,
    pub init: Bool,
    #[brw(if(version.version > 0 && version.version < 6))]
    pub unused: u32,
    #[brw(if(version.version > 3))]
    pub duration_between_ticks: Option<Time>,
    #[brw(if(version.version > 3))]
    pub time_ticked: Option<Time>,
    #[brw(if(version.version > 3))]
    pub time_passed: Option<Time>,
    #[brw(if(version.version > 4))]
    pub trigger_sys: Option<TriggerSys>,
    #[brw(if(version.version > 6))]
    pub tick: Option<i32>,
}

#[binrw]
#[derive(Debug)]
pub struct UuidGenerator(i64);

impl UuidGenerator {
    pub fn next_id(&mut self) -> Uuid {
        let res = self.0.into();
        self.0 += 1;
        res
    }
}

#[binrw]
#[derive(Debug)]
pub struct MapInfo {
    pub version: Version!(9, "MapInfo"),
    pub start_positions: Array<PatternCursor>,
    pub map_name: Str,
    #[brw(if(version.version > 1))]
    pub dimensions: Option<Dimensions>,
    pub player_types: [PlayerType; PlayerId::COUNT],
    #[brw(if(version.version > 2 && version.version < 6))]
    pub idk3: Option<[(u32, u32, u32); PlayerId::COUNT]>,
    #[brw(if(version.version > 5))]
    pub idk3_2: Option<[(u32, PlayerId, i32, u32); PlayerId::COUNT]>,
    #[brw(if(version.version > 0))]
    pub mission_target_type: MissionTarget,
    #[brw(if(version.version > 3))]
    pub idk4: u32,
    #[brw(if(version.version > 4))]
    pub file_type: FileType,
    #[brw(if(version.version > 6))]
    pub id: Option<CoreUuid>,
    #[brw(if(version.version > 7))]
    pub idk5: Option<Bool>,
    #[brw(if(version.version > 7))]
    pub player_names: Option<[Str; PlayerId::COUNT]>,
    #[brw(if(version.version > 8))]
    pub idk6: Option<u32>,
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
#[derive(Debug, Default)]
pub enum MissionTarget {
    #[default]
    None = 0,
    DestroyAllEnemies = 1,
    ConquerTheMap = 2,
    ProduceCoins = 3,
    ReachThePortal = 4,
}

#[binrw]
#[derive(Debug)]
pub struct TriggerSys {
    pub version: Version!(0, "TriggerSystem"),
    pub init: Bool,
    pub triggers: Array<Trigger>,
}

#[binrw]
#[derive(Debug)]
pub struct Trigger {
    version: Version!(1, "TriggerObject"),
    init: Bool,
    id: Uuid,
    trigger_type: TriggerType,
    pos: PatternCursor,
    idk: u32,
    active: Bool,
    name: Str,
    player_id: PlayerId,
    #[brw(if(version.version > 0))]
    time: Option<Time>,
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
        player_id: PlayerId,
    ) -> Trigger {
        Trigger {
            version: <Version!(1, "TriggerObject")>::new::<1>(),
            init: true.into(),
            id: id_generator.next_id(),
            active: true.into(),
            time: Some(Time {
                duration: Duration::default(),
            }),
            trigger_type,
            pos: pos.into(),
            idk,
            name: name.into(),
            player_id,
        }
    }
}
