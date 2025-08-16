use binrw::{BinRead, BinWrite, binrw};
use bounded_integer::BoundedU32;
use derive_more::IsVariant;
use std::{fmt, marker::PhantomData};
use strum::*;

use crate::Logic;

#[macro_export]
macro_rules! Version {
    ($MAX_VER:literal, $name:literal) => {
        InnerVersion<$MAX_VER, { const_crc32::crc32($name.as_bytes()) }, {$name.len() as u32}>
    };
}

//TODO: versioned option, nonmax uuid
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
    None = 0,
    SouthEast = 1,
    SouthWest = 2,
    West = 3,
    NorthWest = 4,
    NorthEast = 5,
    Ost = 6,
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
#[derive(Debug, EnumCount, FromRepr, PartialEq, Eq)]
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
    id: i64,
}

impl Default for Uuid {
    fn default() -> Self {
        Self {
            version: Default::default(),
            id: -1,
        }
    }
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
#[brw(repr = u32)]
#[derive(Debug)]
pub enum AiType {
    None = -1,
    Weak = 1,
    Normal = 2,
    Strong = 3,
}

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum Good {
    None = u32::MAX, //TODO remove
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
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum PatternType {
    PatternBorder = 0x76d31873,
    PatternWater = 0xfe6bd1b3,
    PatternAcre = 0xca56701a,
    PatternMeadow = 0xbfe4e8e3,
    PatternMeadow1 = 0x4545fac1,
    PatternMeadow2 = 0x4545fac2,
    PatternMeadow3 = 0x4545fac3,
    PatternMeadow4 = 0x4545fac4,
    PatternMeadow5 = 0x4545fac5,
    PatternMeadow6 = 0x4545fac6,
    PatternMeadow7 = 0x4545fac7,
    PatternMeadow9 = 0x4545fac9,
    PatternSand = 0xbadeb00d,
    PatternSand1 = 0xbadeb00e,
    PatternSand2 = 0xbadeb00f,
    PatternSand3 = 0xbadeb010,
    PatternSand4 = 0xbadeb011,
    PatternSand5 = 0xbadeb012,
    PatternSand6 = 0xbadeb013,
    PatternRock = 0xd00faffe,
    PatternRock1 = 0xdeadbeef,
    PatternRock2 = 0xcafecafe,
    PatternRock3 = 0xcafecaff,
    PatternRock4 = 0xcafecb00,
    PatternRock5 = 0xcafecb01,
    PatternRock6 = 0xcafecb02,
    PatternRock7 = 0xcafecb03,
    PatternRock8 = 0xcafecb04,
    PatternRock9 = 0xcafecb05,
    PatternSnow = 0xfade0ff,
    PatternSeaground = 0xbabeb00b,
    PatternSeaground1 = 0x13374e4,
    PatternSeaground2 = 0x13374e5,
    PatternSeaground3 = 0x13374e6,
    PatternSeaground4 = 0x13374e7,
    PatternSeaground5 = 0x13374e8,
    PatternSwamp = 0x680004e4,
    PatternSwamp1 = 0x680004e5,
    PatternSwamp2 = 0x680004e6,
    PatternPavement = 0xdecade01,
    PatternGround = 0xde5e1110,
    PatternEarth1 = 0x777fa8c0,
    PatternLGround00 = 0xdecade02,
    PatternLGround01 = 0xdecade03,
    PatternLRock00 = 0xdecade04,
    PatternLRock01 = 0xdecade05,
    PatternLRock02 = 0xdecade06,
    PatternLRock03 = 0xca87fab0,
    PatternLGround02 = 0xdecade07,
    PatternLGround03 = 0xdecade08,
    PatternLGround04 = 0xdecade09,
    PatternLGround05 = 0xdecade0a,
    PatternLSand00 = 0xf1cabb70,
    PatternLMeadow00 = 0xf67adb70,
    PatternMMeadow00 = 0xfa1ca560,
    PatternMMeadow01 = 0xfa1ca561,
    PatternMMeadow02 = 0xfa1ca562,
    PatternMMeadow03 = 0xfa1ca563,
    PatternMGround00 = 0xfa1ca570,
    PatternMGround01 = 0xfa1ca571,
    PatternMRock00 = 0xfa1ca580,
    PatternMRock01 = 0xfa1ca581,
    PatternMRock02 = 0xfa1ca582,
    PatternMRock03 = 0xfa1ca583,
    PatternMRock04 = 0xfa1ca584,
    PatternMRock05 = 0xfa1ca585,
    PatternMRock06 = 0xfa1ca586,
    PatternMRock07 = 0xfa1ca587,
    PatternMRock08 = 0xfa1ca588,
    PatternMRock09 = 0xfa1ca589,
    PatternMRock10 = 0xfa1ca58a,
    PatternMSeaground00 = 0xfa1ca590,
    PatternMSeaground01 = 0xfa1ca591,
}

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum AmbientType {
    Beach = 0x5bdc4873,
    Desert1 = 0xdf5602f3,
    Desert2 = 0x67ef3d13,
    Desert3 = 0x31f23a23,
    Forest1 = 0x89f59a23,
    Forest2 = 0x118e5363,
    Meadow1 = 0x5aaad2d3,
    Meadow2 = 0x623437d3,
    Water1 = 0xf35757d3,
    Water2 = 0x3b68a763,
    Water3 = 0x00a67113,
    Water4 = 0x875d51f3,
    Lava = 0xa952bba3,
}

#[binrw]
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug)]
pub enum RemainsType {
    Small = 0xd862d443,
    Medium = 0x122489b3,
    Large = 0x595400e3,
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
#[brw(repr = u32)]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum DoodadType {
    Bush0 = 0x6b272ede,
    Stones0 = 0x455c1f4e,
    Stones1 = 0xea36023e,
    Stones2 = 0x9f41d88e,
    Stones3 = 0xa7cbbe6e,
    Stones4 = 0x455c1f4f,
    Stones5 = 0xea36023f,
    Stones6 = 0x9f41d88f,
    Stones7 = 0xa7cbbe6f,
    Deadtree0 = 0xf2bfe81e,
    Deadtree1 = 0xf2bfe81f,
    Bones0 = 0xd5f5a79d,
    Bones1 = 0x175122cd,
    Bones2 = 0x7245a4ad,
    Bones3 = 0x37836e5d,
    Grass0 = 0x3ad137ae,
    Grass1 = 0x3ad137af,
    Grass2 = 0x3ad137b0,
    Grass3 = 0x3ad137b1,
    Mushroom0 = 0xacadeff0,
    Mushroom1 = 0xacadeff1,
    Mushroom2 = 0xacadeff2,
    Mushroom3 = 0xacadeff3,
    Plants0 = 0x482050ee,
    Plants1 = 0xa3068b0e,
    Plants2 = 0xec185b7e,
    Plants3 = 0xfb49cdfe,
    Plants4 = 0x04d434be,
    Plants5 = 0xa12431ce,
    Plants6 = 0x89f5ae33,
    Plants7 = 0x89f5ae34,
    Plants8 = 0x89f5ae35,
    Plants9 = 0x89f5ae36,
    Plants10 = 0x89f5ae37,
    Plants11 = 0x89f5ae38,
    Plants12 = 0x89f5ae39,
    Swamp0 = 0xfadebee1,
    Swamp1 = 0xfadebee2,
    Swamp2 = 0xfadebee3,
    Swamp3 = 0xfadebee4,
    Swamp4 = 0xfadebee5,
    Gate0 = 0xfadebee6,
    Agave0 = 0x8fa67cf0,
    Agave1 = 0x8fa67cf1,
    Agave2 = 0x8fa67cf2,
    EmptySign = 0x28f42343,
    WaterSign = 0x121aa343,
    CoalSignS = 0x90eeb793,
    CoalSignM = 0xc5096143,
    CoalSignL = 0x00ad6ff3,
    IronSignS = 0x45dbe563,
    IronSignM = 0xd33a52e3,
    IronSignL = 0x4b82f123,
    GoldSignS = 0x96771ad3,
    GoldSignM = 0xe06ac3a3,
    GoldSignL = 0xe812d123,
    GranitSignS = 0x3124a193,
    GranitSignM = 0x8ecf0d53,
    GranitSignL = 0x17684773,
    Greenery0 = 0xfa1afe1,
    Greenery1 = 0xfa1afe2,
    Greenery2 = 0xfa1afe3,
    Greenery3 = 0xfa1afe4,
    Greenery4 = 0xfa1afe5,
    Greenery5 = 0xfa1afe6,
    Greenery6 = 0xfa1afe7,
    Greenery7 = 0xfa1afe8,
    Greenery8 = 0xfa1afe9,
    Greenery9 = 0xfa1afea,
    Greenery10 = 0xfa1afeb,
    Greenery11 = 0xfa1afec,
    Greenery12 = 0xfa1afed,
    Greenery13 = 0xfa1afee,
    Greenery14 = 0xfa1afef,
    Greenery15 = 0xfa1aff0,
    Greenery16 = 0xfa1aff1,
    Fingerpost0 = 0xaaa0f1e0,
    Fingerpost1 = 0xaaa0f1e1,
    Fingerpost2 = 0xaaa0f1e2,
    Fingerpost3 = 0xaaa0f1e3,
    Fingerpost4 = 0xaaa0f1e4,
    Fingerpost5 = 0xaaa0f1e5,
    Fingerpost6 = 0xaaa0f1e6,
    Fingerpost7 = 0xaaa0f1e7,
    Rock0 = 0x6fafe0a0,
    Rock1 = 0x6fafe0a1,
    Rock2 = 0x6fafe0a2,
    Rock3 = 0x6fafe0a3,
    Wreck0 = 0xfa11e210,
    Wreck1 = 0xfa11e211,
    Shell0 = 0xfa11e310,
    Shell1 = 0xfa11e311,
    Lavarock0 = 0xcaffeea0,
    Lavarock1 = 0xcaffeea1,
    Lavarock2 = 0xcaffeea2,
    Lavafog1 = 0xaaff17c0,
    Lavafog2 = 0xaaff17c1,
    Lavafog3 = 0xaaff17c2,
    Lavafog4 = 0xaaff17c3,
    Medrock0 = 0xfa91c0a0,
    Medrock1 = 0xfa91c0a1,
    Medrock2 = 0xfa91c0a2,
    Medrock3 = 0xfa91c0a3,
    MedGreenery0 = 0xbca74230,
    MedGreenery1 = 0xbca74231,
    MedGreenery2 = 0xbca74232,
    Waterplant0 = 0xf1c6a230,
    Waterplant1 = 0xf1c6a231,
    Waterplant2 = 0xf1c6a232,
    Waterlily0 = 0xf1d6a230,
    Waterlily1 = 0xf1d6a231,
    Deadsettler = 0xdacb0a13,
}

pub fn has_lifetime(doodad: DoodadType) -> bool {
    use DoodadType::*;
    matches!(
        doodad,
        EmptySign
            | WaterSign
            | CoalSignS
            | CoalSignM
            | CoalSignL
            | IronSignS
            | IronSignM
            | IronSignL
            | GoldSignS
            | GoldSignM
            | GoldSignL
            | GranitSignS
            | GranitSignM
            | GranitSignL
    )
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

#[binrw]
#[derive(derive_more::From, derive_more::Into, Clone, Copy, PartialEq, Eq)]
pub struct InnerVersion<const MAX_VER: u32, const CRC: u32, const LEN: u32> {
    #[br(try_map = |x: u32| x.try_into().map_err(|_| format!("version too large: expected <= {MAX_VER} got {x}")))]
    #[bw(map = |x| x.get())]
    pub version: BoundedU32<0, MAX_VER>,
    #[bw(calc = CRC)]
    hash: u32,
    #[br(assert(len == LEN, "version name length mismatch: expected {LEN}, found {len}"))]
    #[br(assert(hash == CRC))]
    #[bw(calc = LEN)]
    len: u32,
}

impl<const MAX_VER: u32, const CRC: u32, const LEN: u32> InnerVersion<MAX_VER, CRC, LEN> {
    pub fn new<const VER: u32>() -> Self {
        Self {
            version: BoundedU32::<0, MAX_VER>::const_new::<VER>(),
        }
    }
}

impl<const CRC: u32, const LEN: u32> Default for InnerVersion<0, CRC, LEN> {
    fn default() -> Self {
        Self {
            version: Default::default(),
        }
    }
}

impl<const MAX_VER: u32, const CRC: u32, const LEN: u32> fmt::Debug
    for InnerVersion<MAX_VER, CRC, LEN>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if MAX_VER == 0 {
            return Ok(());
        }
        self.version.get().fmt(f)
    }
}

#[binrw]
pub struct Ref<I: Ided> {
    id: Uuid,
    _marker: PhantomData<I>,
}

impl<I: Ided> fmt::Debug for Ref<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl<I: Ided> Default for Ref<I> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<I: Ided> Ref<I> {
    fn get<'a>(&self, provider: &'a [I]) -> Option<&'a I> {
        provider.iter().find(|b| self.id == b.id())
    }
}

pub trait Ided {
    fn id(&self) -> Uuid;
}

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
    version: Version!(0, "Core UUID"),
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
    version: Version!(0, "ElevationCursor"),
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
    version: Version!(0, "PatternCursor"),
    #[br(assert(x < 1000 || x == u32::MAX))]
    pub x: u32,
    #[br(assert(y < 1000 || y == u32::MAX))]
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
#[derive(Debug)]
pub struct MapIdxPos {
    x: u32,
    y: u32,
}
