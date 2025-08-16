use crate::Version;
use crate::VersionI;
use crate::VersionedI;

use crate::{buildings::Building, helper_structs::*, net::Flag, settlers::Settler};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Transport {
    version: VersionI!("Transport System"),
    packages: Packages,
    package_needs: VersionedI!("Transport PackageNeedSystem", Array<PackageNeedTarget>),
    building_needs: VersionedI!("Transport BuildingNeedSystem", Array<BuildingNeedGood>),
}

#[binrw]
#[derive(Debug)]
struct Packages {
    version: VersionI!("Package System"),
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
    pos: OptionalPatternCursor,
    idk2: Bool,
    idk3: u32,
    idk4: u32,
    idk5: OptionalPatternCursor,
    idk6: OptionalPatternCursor,
    id2: Uuid, //TODO what?
}

impl Ided for Package {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct PackageNeedTarget {
    version: Version!("Transport PackageNeedTarget"),
    package_ref: Ref<Package>,
    building_ref: Ref<Building>,
}

#[binrw]
#[derive(Debug)]
struct BuildingNeedGood {
    version: Version!(1, "Transport BuildingNeedGood"),
    building_ref: Ref<Building>,
    good: Good,
    package_ref: Ref<Package>,
    idk: u32,
    building_ref2: Ref<Building>,
}
