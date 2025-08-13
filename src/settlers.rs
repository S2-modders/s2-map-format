use crate::buildings::Building;
use crate::helper_structs::*;
use crate::movement::SettlerMovement;
use crate::transport::Package;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Settlers {
    #[brw(args("SettlersSystem"))]
    version: Version<0>,
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
pub struct Worker {
    #[brw(args("SettlersWorker"))]
    version: Version<1>,
    work_building_ref: Uuid,
    ship_ref: Uuid,
    test: [u32; 59], //TODO: filler -- decompiling goals takes too long; version 0 has less goals
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
    #[brw(args("SettlersConstructor"))]
    version: Version<0>,
    test: [u32; 6], //TODO: filler -- decompiling goals takes too long
    work_building_ref: Ref<Building>,
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
    #[brw(args("SettlersCarrier"))]
    version: Version<0>,
    test: [u32; 9], //TODO: filler -- decompiling goals takes too long
    idk: Bool,
    package_ref: Ref<Package>,
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
    #[brw(args("SettlersBulldozer"))]
    version: Version<0>,
    test: [u32; 2], //TODO: filler -- decompiling goals takes too long
    building_ref: Ref<Building>,
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
    #[brw(args("SettlersSoldier"))]
    version: Version<3>,
    test: [u32; 2], //TODO: filler -- decompiling goals takes too long
    building_ref: Ref<Building>,
    building_ref2: Ref<Building>,
    settler_ref: Ref<Settler>,
    test0: [u32; 10], //TODO: filler -- decompiling goals takes too long; version dependent
    #[br(if(version.version > 1))]
    building_ref3: Option<Uuid>,
    #[br(if(version.version > 2))]
    idk: Bool,
    live_points: LivePoints,
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
    #[brw(args("SettlersLivePoints"))]
    version: Version<0>,
    idk: f32,
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
pub struct Specialist {
    #[brw(args("SettlersSpecialist"))]
    version: Version<0>,
    #[br(dbg)]
    test: [u32; 20], //TODO: filler -- decompiling goals takes too long -- not tested if right size
    idk: u32,
    destination_flag_ref: Uuid,
    settler: Settler,
}

impl Ided for Specialist {
    fn id(&self) -> Uuid {
        self.settler.id()
    }
}

#[binrw]
#[derive(Debug)]
pub struct Settler {
    #[brw(args("Settlers Settler"))]
    version: Version<0>,
    id: Uuid,
    movement: SettlerMovement,
    animation: Animation,
    package_ref: Uuid,
    settler_type: u32,
    state: u32,
    test: [u32; 5], //TODO: filler -- decompiling goals takes too long
    building_ref: Ref<Building>,
}

impl Ided for Settler {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct Animation {
    #[brw(args("SettlersAnimation"))]
    version: Version<1>,
    remaining_time: f32,
    #[brw(if(version.version == 1))]
    end_time: Option<f32>,
    animation_type: u32,
}
