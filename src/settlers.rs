use crate::Version;
use crate::VersionI;
use crate::navy::Ship;
use crate::net::Flag;

use crate::buildings::Building;
use crate::helper_structs::*;
use crate::movement::SettlerMovement;
use crate::transport::Package;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Settlers {
    version: VersionI!("SettlersSystem"),
    pub workers: Array<Worker>,
    pub constructor: Array<Constructor>,
    pub carrier: Array<Carrier>,
    pub bulldoser: Array<Bulldozer>,
    pub soldier: Array<Soldier>,
    pub specialist: Array<Specialist>,
}

#[binrw]
#[derive(Debug)]
pub struct Worker {
    #[br(temp)]
    #[bw(calc = settler.owner)]
    owner: PlayerId,
    version: Version!(1, "SettlersWorker"),
    work_building_ref: Ref<Building>,
    ship_ref: OptRef<Ship>,
    test: [u32; 59], //TODO: filler -- decompiling goals takes too long; version 0 has less goals
    #[br(args(owner))]
    settler: Settler,
}

impl Ided for Worker {
    fn id(&self) -> Uuid {
        self.settler.id()
    }
}

#[binrw]
#[derive(Debug)]
pub struct Constructor {
    #[br(temp)]
    #[bw(calc = settler.owner)]
    owner: PlayerId,
    version: Version!("SettlersConstructor"),
    test: [u32; 6], //TODO: filler -- decompiling goals takes too long
    work_building_ref: OptRef<Building>,
    #[br(args(owner))]
    settler: Settler,
}

impl Ided for Constructor {
    fn id(&self) -> Uuid {
        self.settler.id()
    }
}

#[binrw]
#[derive(Debug)]
pub struct Carrier {
    #[br(temp)]
    #[bw(calc = settler.owner)]
    owner: PlayerId,
    version: Version!("SettlersCarrier"),
    test: [u32; 9], //TODO: filler -- decompiling goals takes too long
    idk: Bool,
    package_ref: OptRef<Package>,
    #[br(args(owner))]
    settler: Settler,
}

impl Ided for Carrier {
    fn id(&self) -> Uuid {
        self.settler.id()
    }
}

#[binrw]
#[derive(Debug)]
pub struct Bulldozer {
    #[br(temp)]
    #[bw(calc = settler.owner)]
    owner: PlayerId,
    version: Version!("SettlersBulldozer"),
    test: [u32; 2], //TODO: filler -- decompiling goals takes too long
    building_ref: OptRef<Building>,
    #[br(args(owner))]
    settler: Settler,
}

impl Ided for Bulldozer {
    fn id(&self) -> Uuid {
        self.settler.id()
    }
}

#[binrw]
#[derive(Debug)]
pub struct Soldier {
    #[br(temp)]
    #[bw(calc = settler.owner)]
    owner: PlayerId,
    version: Version!(3, "SettlersSoldier"),
    test: [u32; 2], //TODO: filler -- decompiling goals takes too long
    building_ref: Ref<Building>,
    building_ref2: OptRef<Building>,
    settler_ref: OptRef<Settler>,
    test0: [u32; 10], //TODO: filler -- decompiling goals takes too long; version dependent
    building_ref3: OptRef<Building>,
    idk: Bool,
    live_points: LivePoints,
    #[br(args(owner))]
    settler: Settler,
}

impl Ided for Soldier {
    fn id(&self) -> Uuid {
        self.settler.id()
    }
}

#[binrw]
#[derive(Debug)]
struct LivePoints {
    version: Version!("SettlersLivePoints"),
    idk: f32,
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
pub struct Specialist {
    #[br(temp)]
    #[bw(calc = settler.owner)]
    owner: PlayerId,
    version: Version!("SettlersSpecialist"),
    #[br(dbg)]
    test: [u32; 20], //TODO: filler -- decompiling goals takes too long -- not tested if right size
    idk: u32,
    destination_flag_ref: Ref<Flag>,
    #[br(args(owner))]
    settler: Settler,
}

impl Ided for Specialist {
    fn id(&self) -> Uuid {
        self.settler.id()
    }
}

#[binrw]
#[derive(Debug)]
#[br(import(owner: PlayerId))]
pub struct Settler {
    #[br(calc = owner)]
    #[bw(ignore)]
    owner: PlayerId,
    version: Version!("Settlers Settler"),
    id: Uuid,
    movement: SettlerMovement,
    animation: Animation,
    package_ref: OptRef<Package>,
    settler_type: SettlerType,
    state: SettlerState,
    test: [u32; 5], //TODO: filler -- decompiling goals takes too long
    building_ref: OptRef<Building>,
}

impl Ided for Settler {
    fn id(&self) -> Uuid {
        self.id
    }
}
#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum SettlerState {
    Dying = 5,
    TargetReached = 26,
    DigForResources = 10,
    MoveToPackage = 29,
    Idle = 12,
    BringGoodHome = 36,
    PreInit = 0,
    RemoveMe = 2,
    BulldozeAroundBuilding = 22,
    EnterTarget = 25,
    LeaveTarget = 27,
    PickupPackage = 30,
    PutDownPackage = 31,
    ExecuteWorkingProcess = 33,
    CaptureBuilding = 15,
    ComputeNextWork = 35,
    AttackDefender = 14,
    ConstructBuilding = 32,
    BuildingProductionDisabled = 38,
    GoHome = 37,
    Initializing = 1,
    MoveFromCastleToWorkingPlace = 4,
    Attack = 18,
    MoveToWaitPosition = 19,
    LeaveMap = 3,
    WaitForLeave = 21,
    Dead = 6,
    ReturnToWorkingPosition = 11,
    DefenceWait = 39,
    MoveToFarget = 24,
    MoveToWorkingPlace = 8,
    ReturnToMilitaryBuilding = 16,
    LeaveHome = 23,
    Wait = 20,
    SearchPosition = 9,
    LeaveBuilding = 7,
    Defend = 17,
    MoveToAttackBuilding = 13,
    WaitForPackage = 28,
    ExecuteAlternativeWorkingProcess = 34,
}

#[binrw]
#[derive(Debug)]
struct Animation {
    version: Version!(1, "SettlersAnimation"),
    remaining_time: Time,
    end_time: Time,
    animation_type: AnimationType,
}

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum AnimationType {
    Stand = 0,
    Walk = 1,
    SwimIdle = 2,
    SwimForward = 3,
    WorkLoop0 = 4,
    WorkLoop1 = 5,
    WorkLoop2 = 6,
    Pickup = 7,
    PutDown = 8,
    Hit = 9,
}
