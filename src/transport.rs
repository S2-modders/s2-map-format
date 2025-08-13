use crate::helper_structs::*;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Transport {
    #[brw(args(0, "Transport System"))]
    version: Version,
    init: Bool,
    packages: Packages,
    package_needs: PackageNeeds,
    building_needs: BuildingNeeds,
}

#[binrw]
#[derive(Debug)]
struct Packages {
    #[brw(args(0, "Package System"))]
    version: Version,
    init: Bool,
    package: Array<(PlayerId, Package)>,
}

#[binrw]
#[derive(Debug)]
struct Package {
    #[brw(args(1, "Transport Package"))]
    version: Version,
    id: Uuid,
    idk: Bool,
    idk1: Bool,
    building_ref: Uuid,
    settler_ref: Uuid,
    flag_ref: Uuid,
    good: Good,
    pos: PatternCursor,
    idk2: Bool,
    idk3: u32,
    idk4: u32,
    #[brw(if(version.version > 0))]
    idk5: PatternCursor,
    #[brw(if(version.version > 0))]
    idk6: PatternCursor,
    #[brw(if(version.version > 0))]
    id2: Uuid,
}

#[binrw]
#[derive(Debug)]
struct PackageNeeds {
    #[brw(args(0, "Transport PackageNeedSystem"))]
    version: Version,
    init: Bool,
    package: Array<PackageNeedTarget>,
}

#[binrw]
#[derive(Debug)]
struct PackageNeedTarget {
    #[brw(args(0, "Transport PackageNeedTarget"))]
    version: Version,
    package_ref: Uuid,
    building_ref: Uuid,
}

#[binrw]
#[derive(Debug)]
struct BuildingNeeds {
    #[brw(args(0, "Transport BuildingNeedSystem"))]
    version: Version,
    init: Bool,
    building_need_goods: Array<BuildingNeedGood>,
}

#[binrw]
#[derive(Debug)]
struct BuildingNeedGood {
    #[brw(args(1, "Transport BuildingNeedGood"))]
    version: Version,
    building_ref: Uuid,
    good: Good,
    package_ref: Uuid,
    idk: u32,
    #[brw(if(version.version > 0))]
    building_ref2: Uuid,
}
