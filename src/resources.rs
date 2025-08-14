use crate::buildings::Building;
use crate::movement::AnimalMovement;
use crate::{Version, helper_structs::*};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Resources {
    version: Version!(4, "resources"),
    init: Bool,
    deposits: Array<Deposit>,
    #[brw(if(version.version > 0))]
    animals: Array<Animal>,
    #[brw(if(version.version > 2))]
    respawn: Option<AnimalRespawn>,
    #[brw(if(version.version > 3))]
    idk: u32,
    #[brw(if(version.version > 3))]
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
struct AnimalRespawn {
    version: Version!(0, "Resources AnimalRespawn"),
    init: Bool,
    tick: CapedU32<999>,
    tick_increment: u32,
    pos: LastTickedPos,
}

#[binrw]
#[derive(Debug)]
pub struct Deposit {
    deposit_type: DepositType,
    version: Version!(1, "deposit"),
    id: Uuid,
    pos: PatternCursor,
    buildingref: Ref<Building>,
    pos2: ElevationCursor,
    current_grouth: f32,
    #[brw(if(version.version > 0))]
    age: u32,
    #[brw(if(version.version > 0))]
    life_time: u32,
}

impl Ided for Deposit {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
pub struct Animal {
    animal_type: AnimalType,
    version: Version!(2, "Resources Animal"),
    id: Uuid,
    idk: f32,
    pos: PatternCursor,
    movement: AnimalMovement,
    idk1: u32, // 2 = is interpolating movement
    #[brw(if(version.version > 0))]
    idk2: u32,
    #[brw(if(version.version > 1))]
    villagebuildingref: Ref<Building>,
}

impl Ided for Animal {
    fn id(&self) -> Uuid {
        self.id
    }
}
