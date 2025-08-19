use crate::Version;
use crate::VersionI;
use crate::VersionedI;

use crate::{buildings::Building, helper_structs::*, net::Flag, settlers::Settler};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Transport {
    version: VersionI!("Transport System"),
    pub packages: VersionedI!("Package System", Array<Package>),
    pub package_needs: VersionedI!("Transport PackageNeedSystem", Array<PackageNeedTarget>),
    pub building_needs: VersionedI!("Transport BuildingNeedSystem", Array<BuildingNeedGood>),
}

#[binrw]
#[derive(Debug)]
pub struct Package {
    owner: PlayerId,
    version: Version!(1, "Transport Package"),
    id: Uuid,
    idk: Bool,
    idk1: Bool,
    building_ref: OptRef<Building>,
    settler_ref: OptRef<Settler>,
    flag_ref: OptRef<Flag>,
    good: Good,
    pos: OptionalPatternCursor,
    idk2: Bool,
    idk3: u32,
    idk4: u32,
    idk5: OptionalPatternCursor,
    idk6: OptionalPatternCursor,
    net_ref: OptRef<Flag>, //TODO what?
}

impl Ided for Package {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
pub struct PackageNeedTarget {
    version: Version!("Transport PackageNeedTarget"),
    package_ref: Ref<Package>,
    building_ref: Ref<Building>,
}

#[binrw]
#[derive(Debug)]
pub struct BuildingNeedGood {
    version: Version!(1, "Transport BuildingNeedGood"),
    building_ref: Ref<Building>,
    good: Good,
    package_ref: OptRef<Package>,
    idk: u32,
    building_ref2: OptRef<Building>,
}
