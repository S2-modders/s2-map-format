use crate::VersionI;
use crate::buildings::Building;
use crate::movement::AnimalMovement;
use crate::{Version, helper_structs::*};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Resources {
    version: VersionI!(4, "resources"),
    deposits: Array<Deposit>,
    animals: Array<Animal>,
    respawn: AnimalRespawn,
    idk: u32,
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
struct AnimalRespawn {
    version: VersionI!(0, "Resources AnimalRespawn"),
    tick: CapedU32<999>,
    tick_increment: u32,
    pos: MapIdxPos,
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
    age: u32,
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
    age: Time,
    pos: PatternCursor,
    movement: AnimalMovement,
    idk1: u32, // 2 = is interpolating movement
    idk2: u32,
    villagebuildingref: Ref<Building>,
}

impl Ided for Animal {
    fn id(&self) -> Uuid {
        self.id
    }
}
