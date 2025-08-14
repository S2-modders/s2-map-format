use crate::Version;

use crate::{buildings::Building, helper_structs::*, net::Flag, settlers::Settler};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Transport {
    version: Version!(0, "Transport System"),
    init: Bool,
    packages: Packages,
    package_needs: PackageNeeds,
    building_needs: BuildingNeeds,
}

#[binrw]
#[derive(Debug)]
struct Packages {
    version: Version!(0, "Package System"),
    init: Bool,
    package: Array<(PlayerId, Package)>,
}

#[binrw]
#[derive(Debug)]
pub struct Package {
    version: Version!(1, "Transport Package"),
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
    version: Version!(0, "Transport PackageNeedSystem"),
    init: Bool,
    package: Array<PackageNeedTarget>,
}

#[binrw]
#[derive(Debug)]
struct PackageNeedTarget {
    version: Version!(0, "Transport PackageNeedTarget"),
    package_ref: Ref<Package>,
    building_ref: Ref<Building>,
}

#[binrw]
#[derive(Debug)]
struct BuildingNeeds {
    version: Version!(0, "Transport BuildingNeedSystem"),
    init: Bool,
    building_need_goods: Array<BuildingNeedGood>,
}

#[binrw]
#[derive(Debug)]
struct BuildingNeedGood {
    version: Version!(1, "Transport BuildingNeedGood"),
    building_ref: Ref<Building>,
    good: Good,
    package_ref: Ref<Package>,
    idk: u32,
    #[brw(if(version.version > 0))]
    building_ref2: Ref<Building>,
}
