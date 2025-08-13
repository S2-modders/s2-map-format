use crate::helper_structs::*;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Transport {
    #[brw(args("Transport System"))]
    version: Version<0>,
    init: Bool,
    packages: Packages,
    package_needs: PackageNeeds,
    building_needs: BuildingNeeds,
}

#[binrw]
#[derive(Debug)]
struct Packages {
    #[brw(args("Package System"))]
    version: Version<0>,
    init: Bool,
    package: Array<(PlayerId, Package)>,
}

#[binrw]
#[derive(Debug)]
struct Package {
    #[brw(args("Transport Package"))]
    version: Version<1>,
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
    #[brw(args("Transport PackageNeedSystem"))]
    version: Version<0>,
    init: Bool,
    package: Array<PackageNeedTarget>,
}

#[binrw]
#[derive(Debug)]
struct PackageNeedTarget {
    #[brw(args("Transport PackageNeedTarget"))]
    version: Version<0>,
    package_ref: Uuid,
    building_ref: Uuid,
}

#[binrw]
#[derive(Debug)]
struct BuildingNeeds {
    #[brw(args("Transport BuildingNeedSystem"))]
    version: Version<0>,
    init: Bool,
    building_need_goods: Array<BuildingNeedGood>,
}

#[binrw]
#[derive(Debug)]
struct BuildingNeedGood {
    #[brw(args("Transport BuildingNeedGood"))]
    version: Version<1>,
    building_ref: Uuid,
    good: Good,
    package_ref: Uuid,
    idk: u32,
    #[brw(if(version.version > 0))]
    building_ref2: Uuid,
}
