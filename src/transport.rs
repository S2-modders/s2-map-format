use crate::{buildings::Building, helper_structs::*, net::Flag, settlers::Settler};
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
pub struct Package {
    #[brw(args("Transport Package"))]
    version: Version<1>,
    id: Uuid,
    idk: Bool,
    idk1: Bool,
    building_ref: Ref<Building>,
    settler_ref: Ref<Settler>,
    flag_ref: Ref<Flag>,
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

impl Ided for Package {
    fn id(&self) -> Uuid {
        self.id
    }
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
    package_ref: Ref<Package>,
    building_ref: Ref<Building>,
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
    building_ref: Ref<Building>,
    good: Good,
    package_ref: Ref<Package>,
    idk: u32,
    #[brw(if(version.version > 0))]
    building_ref2: Ref<Building>,
}
