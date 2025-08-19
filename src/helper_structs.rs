use binrw::{BinRead, BinResult, BinWrite, binrw};
use bounded_integer::BoundedU32;
use derive_more::IsVariant;
use grid::Grid;
use nonmax::NonMaxU64;
use std::{fmt, marker::PhantomData, time::Duration};
use strum::*;

#[macro_export]
macro_rules! Versioned {
    ($name:literal, $ty:ty, $ty2:ty) => {
        Versioned2<0, { const_crc32::crc32($name.as_bytes()) }, {$name.len() as u32}, $ty, $ty2>
    };
    ($name:literal, $ty:ty) => {
            Versioned<0, { const_crc32::crc32($name.as_bytes()) }, {$name.len() as u32}, $ty>
    };
    ($MAX_VER:literal, $name:literal, $ty:ty, $ty2:ty) => {
        Versioned2<$MAX_VER, { const_crc32::crc32($name.as_bytes()) }, {$name.len() as u32}, $ty, $ty2>
    };
    ($MAX_VER:literal, $name:literal, $ty:ty) => {
        Versioned<$MAX_VER, { const_crc32::crc32($name.as_bytes()) }, {$name.len() as u32}, $ty>
    };
}

#[macro_export]
macro_rules! VersionedI {
    ($name:literal, $ty:ty) => {
        VersionedI<0, { const_crc32::crc32($name.as_bytes()) }, {$name.len() as u32}, $ty>
    };
    ($MAX_VER:literal, $name:literal, $ty:ty) => {
        VersionedI<$MAX_VER, { const_crc32::crc32($name.as_bytes()) }, {$name.len() as u32}, $ty>
    };
}

#[macro_export]
macro_rules! VersionI {
    ($name:literal) => {
        VersionI<0, { const_crc32::crc32($name.as_bytes()) }, {$name.len() as u32}>
    };
    ($MAX_VER:literal, $name:literal) => {
        VersionI<$MAX_VER, { const_crc32::crc32($name.as_bytes()) }, {$name.len() as u32}>
    };
}

#[macro_export]
macro_rules! Version {
    ($name:literal) => {
        Version<0, { const_crc32::crc32($name.as_bytes()) }, {$name.len() as u32}>
    };
    ($MAX_VER:literal, $name:literal) => {
        Version<$MAX_VER, { const_crc32::crc32($name.as_bytes()) }, {$name.len() as u32}>
    };
}

//TODO: named, positioned, owned
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
#[derive(Debug, EnumCount, FromRepr)]
pub enum Direction {
    East = 0,
    SouthEast = 1,
    SouthWest = 2,
    West = 3,
    NorthWest = 4,
    NorthEast = 5,
    North = 6,
    South = 7,
}

#[binrw]
#[derive(Debug, Default, EnumCount, PartialEq, Eq, IsVariant)]
pub enum OptNone<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug,
{
    #[default]
    #[brw(magic = 0u32)]
    None,
    Some(T),
}

impl<T> OptNone<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug,
{
    pub const fn as_ref(&self) -> Option<&T> {
        match *self {
            OptNone::Some(ref x) => Some(x),
            OptNone::None => None,
        }
    }
}

impl<T> From<OptNone<T>> for Option<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug,
{
    fn from(val: OptNone<T>) -> Self {
        match val {
            OptNone::None => None,
            OptNone::Some(t) => Some(t),
        }
    }
}

#[binrw]
#[derive(Debug, Default, EnumCount, PartialEq, Eq, IsVariant)]
pub enum Optional<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug,
{
    #[default]
    #[brw(magic = 0xffffffffu32)]
    None,
    Some(T),
}

impl<T> Optional<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug,
{
    pub const fn as_ref(&self) -> Option<&T> {
        match *self {
            Optional::Some(ref x) => Some(x),
            Optional::None => None,
        }
    }
}

impl<T> From<Optional<T>> for Option<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug,
{
    fn from(val: Optional<T>) -> Self {
        match val {
            Optional::None => None,
            Optional::Some(t) => Some(t),
        }
    }
}

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug, EnumCount, FromRepr, PartialEq, Eq, Clone, Copy)]
pub enum PlayerId {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6, //Not used ingame
    P7 = 7, //Not used ingame
}

#[binrw]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Uuid {
    version: Version!(0, "logic UniqueId"),
    #[br(try_map = |x:u64| x.try_into())]
    #[bw(map = |x| x.get())]
    pub id: NonMaxU64,
}

impl From<Uuid> for NonMaxU64 {
    fn from(val: Uuid) -> Self {
        val.id
    }
}

impl From<NonMaxU64> for Uuid {
    fn from(id: NonMaxU64) -> Self {
        Self {
            id,
            version: Default::default(),
        }
    }
}

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum Good {
    Food = 0x65cd3afe,
    Tools = 0xd5aa9bde,
    Wood = 0xcd6ec133,
    Plank = 0xc5c945a3,
    Stone = 0x55e952d3,
    Pig = 0xbb5d0683,
    Corn = 0x17626a03,
    Flour = 0xe70c72d3,
    Fish = 0x4012e5a3,
    Flesh = 0xad0d6c63,
    Bread = 0x90ab8923,
    Water = 0xa9676263,
    Beer = 0x58647da3,
    Coal = 0x7068dcd3,
    Iron = 0x8198bf23,
    Gold = 0x4f41c633,
    Coin = 0x499d4c63,
    Gripper = 0x6cc2d2d3,
    Axe = 0xe1281613,
    Saw = 0x3788ffc3,
    Hammer = 0x69c15033,
    Shovel = 0x67c04e73,
    Pot = 0xed76ae73,
    FishingRod = 0xf3f3dcc3,
    Scythe = 0x1ceacd43,
    Pickaxe = 0x45d899e3,
    RollingPin = 0xc2049323,
    Spear = 0x79f34393,
    Sword = 0xa8483903,
    Shield = 0x1eebcf03,
    Boat = 0xf63de553,
    IronOre = 0xec5020be,
    Hatchet = 0xf2b7453e,
    DonkeyItem = 0x2448356e,
    Weapons = 0x0897d1e3,
    Ship = 0x1d66b4ae,
}

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum SettlerType {
    Constructor = 0xdd2daebe,
    Bulldozer = 0x2740000e,
    Woodcutter = 0x2c50533e,
    Forester = 0x6eba970e,
    Stonecutter = 0x8a3aa1ee,
    Hunter = 0xab43100e,
    Cabinetmaker = 0x2195bc53,
    Baker = 0x862797a3,
    Miller = 0x290e6b9e,
    Farmer = 0x8d0ba75e,
    Helper = 0x7bff15ce,
    Miner = 0x59b7633e,
    Scholar = 0x3355c25e,
    Explorer = 0xa07bf6fe,
    Soldier0 = 0x15e86843,
    Soldier1 = 0xba0e7f83,
    Soldier2 = 0x466c6c23,
    Soldier3 = 0x906014f3,
    Soldier4 = 0x559ddea3,
    Fisher = 0x4ac589ce,
    Brewer = 0x0eaffece,
    Locksmith = 0xe59a65ce,
    Smith = 0x03a2b9f3,
    Coiner = 0xfdf21163,
    Smelter = 0x9c0ae313,
    Pigfarmer = 0x360b16e3,
    Donkey = 0xd4157c5f,
    DonkeyBreeder = 0x35a82813,
    ShipBuilder = 0xae71fbf3,
    Slaughter = 0xf0622d7e,
    ShipHelper = 0x182118d3,
}

#[binrw]
#[repr(u32)]
#[derive(Debug)]
pub enum GoodOrSettler {
    Good(Good),
    Settler(SettlerType),
}

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
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum Tribe {
    Romans = 0,
    Africans = 1,
    Chinese = 2,
}

#[binrw]
#[derive(derive_more::From, derive_more::Into, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Versioned2<const MAX_VER: u32, const CRC: u32, const LEN: u32, T, D>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug + 'static,
    for<'a> D: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug + 'static,
{
    version: Version<MAX_VER, CRC, LEN>,
    pub first: T,
    pub second: D,
}

#[binrw]
#[derive(derive_more::From, derive_more::Into, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Versioned<const MAX_VER: u32, const CRC: u32, const LEN: u32, T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug + 'static,
{
    version: Version<MAX_VER, CRC, LEN>,
    pub data: T,
}

#[binrw]
#[derive(derive_more::From, derive_more::Into, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct VersionedI<const MAX_VER: u32, const CRC: u32, const LEN: u32, T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug + 'static,
{
    version: VersionI<MAX_VER, CRC, LEN>,
    pub data: T,
}

#[binrw]
#[derive(derive_more::From, derive_more::Into, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct VersionI<const MAX_VER: u32, const CRC: u32, const LEN: u32> {
    #[br(assert(version == MAX_VER))]
    #[bw(calc = MAX_VER)]
    version: u32,
    #[bw(calc = CRC)]
    hash: u32,
    #[br(assert(len == LEN, "version name length mismatch: expected {LEN}, found {len}"))]
    #[br(assert(hash == CRC))]
    #[bw(calc = LEN)]
    len: u32,
    #[br(assert(init == 1))]
    #[bw(calc = 1)]
    init: u32,
}

#[binrw]
#[derive(derive_more::From, derive_more::Into, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Version<const MAX_VER: u32, const CRC: u32, const LEN: u32> {
    #[br(assert(version == MAX_VER))]
    #[bw(calc = MAX_VER)]
    version: u32,
    #[bw(calc = CRC)]
    hash: u32,
    #[br(assert(len == LEN, "version name length mismatch: expected {LEN}, found {len}"))]
    #[br(assert(hash == CRC))]
    #[bw(calc = LEN)]
    len: u32,
}

#[binrw]
pub struct Idx<I: Ided> {
    idx: u32,
    _marker: PhantomData<I>,
}

impl<I: Ided> Idx<I> {
    fn resolve<'a, O: Owner<I>>(&self, owner: &'a O) -> Option<&'a I> {
        owner.get().get(self.idx as usize)
    }
}

impl<I: Ided> fmt::Debug for Idx<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.idx.fmt(f)
    }
}

#[binrw]
pub struct Ref<I: Ided> {
    pub id: Uuid,
    _marker: PhantomData<I>,
}

impl<I: Ided> fmt::Debug for Ref<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

#[binrw]
pub struct OptRef<I: Ided> {
    version: Version!(0, "logic UniqueId"),
    #[br(map = |x: u64| NonMaxU64::try_from(x).ok().map(|i|i.into()))]
    #[bw(map = |x| x.map(|i|i.into()).map(|i:NonMaxU64|i.get()).unwrap_or(u64::MAX))]
    pub id: Option<Uuid>,
    _marker: PhantomData<I>,
}

trait Owner<I: Ided> {
    fn get(&self) -> &[I];
}

impl<I: Ided> fmt::Debug for OptRef<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl<I: Ided> Ref<I> {
    pub fn get<'a>(&self, provider: &'a [I]) -> Option<&'a I> {
        provider.iter().find(|b| self.id == b.id())
    }
}

impl<I: Ided> Default for OptRef<I> {
    fn default() -> Self {
        Self {
            version: Default::default(),
            id: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<I: Ided> OptRef<I> {
    pub fn get<'a>(&self, provider: &'a [I]) -> Option<&'a I> {
        self.id
            .and_then(|id| provider.iter().find(|b| id == b.id()))
    }
}

pub trait Ided: 'static {
    fn id(&self) -> Uuid;
}

pub type Cooldown<const CAP: u32> = CapedU32<CAP>;
#[binrw]
pub struct CapedU32<const CAP: u32> {
    #[br(try_map = |x: u32| x.try_into())]
    #[bw(map = |x| x.get())]
    val: BoundedU32<0, CAP>,
}

impl<const CAP: u32> fmt::Debug for CapedU32<CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.val.fmt(f)
    }
}

impl<const CAP: u32> CapedU32<CAP> {
    pub fn new<const N: u32>() -> Self {
        Self {
            val: BoundedU32::<0, CAP>::const_new::<N>(),
        }
    }
}

#[binrw]
pub struct CoreUuid {
    version: VersionI!("Core UUID"),
    id: u128,
}

impl fmt::Debug for CoreUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

#[binrw]
#[derive(Debug)]
pub struct OptionalElevationCursor {
    version: Version!(0, "ElevationCursor"),
    pub x: u32,
    pub y: u32,
}

impl Default for OptionalElevationCursor {
    fn default() -> Self {
        Self {
            version: Default::default(),
            x: u32::MAX,
            y: u32::MAX,
        }
    }
}

#[binrw]
#[derive(Clone, Copy)]
pub struct ElevationCursor {
    version: Version!(0, "ElevationCursor"),
    #[br(assert(x != u32::MAX))]
    #[bw(assert(*x != u32::MAX))]
    pub x: u32,
    #[br(assert(y != u32::MAX))]
    #[bw(assert(*y != u32::MAX))]
    pub y: u32,
}

impl fmt::Debug for ElevationCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.x as f32 / 4.0, self.y as f32 / 4.0).fmt(f)
    }
}

#[binrw]
#[derive(Debug)]
pub struct OptionalPatternCursor {
    version: Version!(0, "PatternCursor"),
    pub x: u32,
    pub y: u32,
}

impl Default for OptionalPatternCursor {
    fn default() -> Self {
        Self {
            version: Default::default(),
            x: u32::MAX,
            y: u32::MAX,
        }
    }
}

#[binrw]
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct PatternCursor {
    version: Version!(0, "PatternCursor"),
    #[br(assert(x < 1000))]
    pub x: u32,
    #[br(assert(y < 1000))]
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

#[binrw]
pub struct Time {
    #[br(try_map = Duration::try_from_secs_f32)]
    #[bw(map = Duration::as_secs_f32)]
    pub duration: Duration,
}

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.duration.fmt(f)
    }
}

#[binrw]
#[derive(Debug)]
pub struct MapIdxPos<T, M: Positioned<T>> {
    x: u32,
    y: u32,
    _marker: PhantomData<(T, M)>,
}

pub trait Positioned<T> {
    fn at(&self, x: usize, y: usize) -> &T;
}

#[binrw]
#[derive(Debug)]
pub struct PlayerMap<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug + 'static,
{
    dimensions: (u32, u32),
    #[br(count = dimensions.0 * dimensions.1)]
    #[br(map = |grid: [Vec<T>; PlayerId::COUNT]| grid.map(|x| (x, dimensions.1 as usize).into()))]
    #[bw(write_with = player_map_writer)]
    pub grid: [Grid<T>; PlayerId::COUNT],
}

#[binrw::writer(writer, endian)]
fn player_map_writer<T>(map: &[Grid<T>; PlayerId::COUNT]) -> BinResult<()>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug + 'static,
{
    for grid in map {
        grid.flatten().write_options(writer, endian, ())?;
    }
    Ok(())
}
#[binrw]
#[derive(Debug)]
pub struct Map<T>
where
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + std::fmt::Debug + 'static,
{
    dimensions: (u32, u32),
    #[br(count = dimensions.0 * dimensions.1)]
    #[br(map = |grid: Vec<T>| (grid, dimensions.1 as usize).into())]
    #[bw(map = |grid| grid.flatten())]
    pub grid: Grid<T>,
}
