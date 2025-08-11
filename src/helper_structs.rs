use binrw::{BinRead, BinWrite, binrw};
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
    #[brw(args(0, "logic UniqueId"))]
    version: Version,
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
#[derive(Default, Debug)]
pub struct BuildingType(pub u32); //TODO

#[binrw]
#[derive(Default, derive_more::From, derive_more::Into)]
#[brw(import(max_version: u32, name:&str))]
pub struct Version {
    #[br(assert(version <= max_version))]
    #[bw(assert(*version <= max_version))]
    pub version: u32,
    #[br(assert(hash == crc32fast::hash(name.as_bytes())))]
    #[bw(calc = crc32fast::hash(name.as_bytes()))]
    hash: u32,
    #[br(assert(len as usize == name.len()))]
    #[bw(calc = name.len() as u32)]
    len: u32,
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.version.fmt(f)
    }
}

#[binrw]
pub struct CoreUuid {
    #[brw(args(0, "Core UUID"))]
    version: Version,
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
    #[brw(args(0, "ElevationCursor"))]
    version: Version,
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
    #[brw(args(0, "PatternCursor"))]
    version: Version,
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
