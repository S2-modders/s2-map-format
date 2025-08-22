use crate::Version;
use crate::VersionI;
use crate::helper_structs::*;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Doodads {
    version: VersionI!("DoodadsSystem"),
    pub non_blocking: Array<Doodad>,
    pub life_time: Array<Doodad>,
    pub blocking: Array<Doodad>,
}

#[binrw]
#[derive(Debug)]
pub struct Doodad {
    pub doodad_type: DoodadType,
    version: Version!(1, "DoodadsObject"),
    id: Uuid,
    pub pos: ElevationCursor,
    #[brw(if(doodad_type.has_lifetime()))]
    pub lifetime: Option<u32>,
}

impl Ided for Doodad {
    fn id(&self) -> Uuid {
        self.id
    }
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
    Gate0 = 0xfadebee6, //Portal
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

impl DoodadType {
    pub fn has_lifetime(self) -> bool {
        use DoodadType::*;
        matches!(
            self,
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
}
