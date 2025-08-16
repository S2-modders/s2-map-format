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
    version: VersionI!("Resources AnimalRespawn"),
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
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum BuildingType {
    Empty = u32::MAX, //TODO: requered for Ai constructon order
    Castle = 0xf6e26cb3,
    WoodCutter = 0x5a926fa3,
    Forester = 0x3ff43d23,
    StonePit = 0x043185f3,
    Fisher = 0x3ef3bc43,
    Hunter = 0x3c10e223,
    Spring = 0xc2c7c303,
    Barracks = 0xa7bbc573,
    GuardHouse = 0x12e67603,
    Tower = 0x7d00b493,
    IronMine = 0x18282bd3,
    GoldMine = 0x154c1dae,
    CoalMine = 0x9b027dae,
    StoneMine = 0x6222a09e,
    SawMill = 0x918ff373,
    Mill = 0xf7e2ed93,
    Bakery = 0x0af7bb13,
    SlaughterHouse = 0x5c5e9743,
    Smeltery = 0x154551b3,
    Locksmithery = 0xfaada31e,
    Depot = 0xb3965083,
    ShipYard = 0x78f8184e,
    Brewery = 0xfbca3a8e,
    Smithy = 0xb779ab83,
    Mint = 0xbc203663,
    Catapult = 0xa1445ef3,
    WatchTower = 0x281f0783,
    Farm = 0x07f63873,
    Piggery = 0xa6c72ede,
    DonkeyBreeding = 0x1f0fcd1e,
    Fortress = 0x8c137e93,
    Harbor = 0xcf526ff3,
    Construction = 0x4a2ce4de,
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

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum AnimalType {
    Deer = 0x4a9bef83,
    Rabbit = 0x41797b76,
    Elk = 0x706e7c94,
}
