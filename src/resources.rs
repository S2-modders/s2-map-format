use crate::VersionI;
use crate::ai::{ResourceMapElement, SmallResourceMap};
use crate::buildings::Building;
use crate::movement::AnimalMovement;
use crate::{Version, helper_structs::*};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Resources {
    version: VersionI!(4, "resources"),
    pub deposits: Array<Deposit>,
    pub animals: Array<Animal>,
    pub respawn: AnimalRespawn,
    idk: u32,
    idk2: u32,
}

#[binrw]
#[derive(Debug)]
pub struct AnimalRespawn {
    version: VersionI!("Resources AnimalRespawn"),
    tick: CapedU32<999>,
    tick_increment: u32,
    pos: MapIdxPos<ResourceMapElement, SmallResourceMap>,
}

#[binrw]
#[derive(Debug)]
pub struct Deposit {
    deposit_type: DepositType,
    version: Version!(1, "deposit"),
    id: Uuid,
    pos: PatternCursor,
    buildingref: OptRef<Building>,
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
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum DepositType {
    Tree01 = 0x7e99ce73,
    Tree02 = 0x063287b3,
    Tree03 = 0x489ccb83,
    Tree04 = 0x119f47b3,
    Tree05 = 0xe6cf21d3,
    Tree06 = 0xadef44c3,
    Tree07 = 0x732d0e73,
    Tree08 = 0xe6cf0e73,
    Tree09 = 0xe7cf1e74,
    Tree10 = 0xe8cf2e75,
    Tree11 = 0xe8cf2e76,
    Tree12 = 0xe8cf2e77,
    Tree13 = 0xe8cf2e78,
    TreeLava01 = 0xe8cf2e79,
    TreeLava02 = 0xe8cf2e7a,
    TreeLava03 = 0xe8cf2e7b,
    Tree14 = 0xe8cf2e7c,
    Field01 = 0xdfed4c9e,
    Stone01 = 0x9f1bd60e,
    Stone02 = 0x5bb1115e,
    Stone03 = 0x21ef5bee,
    Stone04 = 0x1946cd8e,
    Stone05 = 0x5d936a9e,
    Stone06 = 0xe42ba2fe,
    MedStone01 = 0x1dab7fd0,
    MedStone02 = 0x1dab7fd1,
    MedStone03 = 0x1dab7fd2,
    MedStone04 = 0x1dab7fd3,
    MedStone05 = 0x1dab7fd4,
    MedStone06 = 0x1dab7fd5,
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
    buildingref: OptRef<Building>,
}

impl Ided for Animal {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum AnimalType {
    Deer = 0x4a9bef83,
    Rabbit = 0x41797b76,
    Elk = 0x706e7c94,
}
