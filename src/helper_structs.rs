use binrw::{BinRead, BinWrite, binrw};
use bounded_integer::BoundedU32;
use std::fmt;
use strum::*;

use crate::Logic;

#[binrw]
#[derive(Default, derive_more::From)]
pub struct Str {
    #[br(temp)]
    #[bw(calc = str.len() as u32)]
    len: u32,
    #[br(count = len)]
    #[br(try_map = String::from_utf8)]
    #[bw(map = String::as_bytes)]
    pub str: String,
}
impl From<&str> for Str {
    fn from(value: &str) -> Self {
        String::from(value).into()
    }
}

impl fmt::Debug for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.str.fmt(f)
    }
}

#[binrw]
#[derive(Default, derive_more::From, derive_more::Deref)]
pub struct Bool {
    #[br(try_map = |x: u32| (x < 2).then_some(x != 0).ok_or("expected bool"))]
    #[bw(map = |x| *x as u32)]
    pub bool: bool,
}

impl fmt::Debug for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.bool.fmt(f)
    }
}

#[binrw]
pub struct Array<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug + 'static,
{
    #[br(temp)]
    #[bw(calc = array.len().try_into().unwrap())]
    len: u32,
    #[br(count = len)]
    pub array: Vec<T>,
}

impl<T> Default for Array<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug + 'static,
{
    fn default() -> Self {
        Self {
            array: Default::default(),
        }
    }
}

impl<T> fmt::Debug for Array<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.array.fmt(f)
    }
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug, EnumCount)]
pub enum PlayerId {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7, //Not used ingame
}

#[binrw]
#[derive(Default)]
pub struct Uuid {
    #[brw(args("logic UniqueId"))]
    version: Version<0>,
    id: i64,
}

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl Uuid {
    pub fn new(logic: &mut Logic) -> Uuid {
        let res = Uuid {
            id: logic.max_id,
            ..Default::default()
        };
        logic.max_id += 1;
        res
    }
}

#[binrw]
#[derive(Default, Debug)]
pub struct Good(u32); //TODO

#[binrw]
#[derive(Default, Debug)]
pub struct RemainsType(u32); //TODO

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum BuildingType {
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
#[derive(derive_more::From, derive_more::Into)]
#[brw(import(name:&str))]
pub struct Version<const MAX_VER: u32> {
    #[br(try_map = |x: u32| x.try_into())]
    #[bw(map = |x| x.get())]
    pub version: BoundedU32<0, MAX_VER>,
    #[br(assert(hash == crc32fast::hash(name.as_bytes())))]
    #[bw(calc = crc32fast::hash(name.as_bytes()))]
    hash: u32,
    #[br(assert(len as usize == name.len()))]
    #[bw(calc = name.len() as u32)]
    len: u32,
}

impl<const MAX_VER: u32> Version<MAX_VER> {
    pub fn new<const VER: u32>() -> Self {
        Self {
            version: BoundedU32::<0, MAX_VER>::const_new::<VER>(),
        }
    }
}

impl Default for Version<0> {
    fn default() -> Self {
        Self {
            version: Default::default(),
        }
    }
}

impl<const MAX_VER: u32> fmt::Debug for Version<MAX_VER> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.version.fmt(f)
    }
}

#[binrw]
pub struct CoreUuid {
    #[brw(args("Core UUID"))]
    version: Version<0>,
    init: Bool,
    id: u128,
}

impl fmt::Debug for CoreUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

#[binrw]
pub struct ElevationCursor {
    #[brw(args("ElevationCursor"))]
    version: Version<0>,
    pub x: u32,
    pub y: u32,
}

impl fmt::Debug for ElevationCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.x, self.y).fmt(f)
    }
}

#[binrw]
#[derive(Default)]
pub struct PatternCursor {
    #[brw(args("PatternCursor"))]
    version: Version<0>,
    pub x: u32,
    pub y: u32,
}

impl From<(u32, u32)> for PatternCursor {
    fn from(value: (u32, u32)) -> Self {
        PatternCursor {
            x: value.0,
            y: value.1,
            ..Default::default()
        }
    }
}

impl fmt::Debug for PatternCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.x, self.y).fmt(f)
    }
}
