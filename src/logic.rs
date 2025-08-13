use crate::helper_structs::*;
use binrw::binrw;
use strum::*;

#[binrw]
#[derive(Debug)]
pub struct Logic {
    pub mapinfo: MapInfo,
    #[brw(args("LogicSystem"))]
    pub version: Version<7>,
    pub max_id: i64,
    pub initialized: Bool,
    #[brw(if(version.version > 0 && version.version < 6))]
    pub unused: u32,
    #[brw(if(version.version > 3))]
    pub seconds_per_tick: f32,
    #[brw(if(version.version > 3))]
    pub ticked_seconds: f32,
    #[brw(if(version.version > 3))]
    pub seconds_passed: f32,
    #[brw(if(version.version > 4))]
    pub trigger_sys: Option<TriggerSys>,
    #[brw(if(version.version > 6))]
    pub tick: i32,
}

#[binrw]
#[derive(Debug)]
pub struct MapInfo {
    #[brw(args("MapInfo"))]
    pub version: Version<9>,
    pub idk: Array<PatternCursor>,
    pub map_name: Str,
    #[brw(if(version.version > 1))]
    pub width: u32,
    #[brw(if(version.version > 1))]
    pub height: u32,
    pub idk2: [u32; PlayerId::COUNT],
    #[brw(if(version.version > 2 && version.version < 6))]
    pub idk3: [(u32, u32, u32); PlayerId::COUNT],
    #[brw(if(version.version > 5))]
    pub idk3_2: [(u32, u32, u32, u32); PlayerId::COUNT],
    #[brw(if(version.version > 0))]
    pub mission_target_type: Option<u32>,
    #[brw(if(version.version > 3))]
    pub idk4: u32,
    #[brw(if(version.version > 4))]
    pub file_type: u32,
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
#[derive(Debug)]
pub enum MissionTarget {
    DestroyAllEnemies = 1,
    ConquerTheMap = 2,
    ProduceCoins = 3,
    ReachThePortal = 4,
}

#[binrw]
#[derive(Debug)]
pub struct TriggerSys {
    #[brw(args("TriggerSystem"))]
    pub version: Version<0>,
    pub init: Bool,
    pub triggers: Array<Trigger>,
}

#[binrw]
#[derive(Debug)]
pub struct Trigger {
    #[brw(args("TriggerObject"))]
    version: Version<1>,
    init: Bool,
    id: Uuid,
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
pub enum TriggerType {
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
            version: Version::new::<1>(),
            init: true.into(),
            id: Uuid::new(logic),
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
