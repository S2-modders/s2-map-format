use crate::Version;
use crate::Versioned;
use crate::{helper_structs::*, net::Street};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct SettlerMovement {
    version: Version!(1, "MovementMovement"),
    movement_rough: Rough,
    movement_fine: Fine,
    movement_interpolator: Interpolator,
    idk: u32,
    sub_tile_pos: ElevationCursor,
    idk2: Bool, //is swimming maybe
    pos: PatternCursor,
}

#[binrw]
#[derive(Debug)]
pub struct AnimalMovement {
    version: Version!(1, "Navy Movement"),
    path: Versioned!("Resources Path", PathBase),
    pos: PatternCursor,
    base_version: Version!("Movement Base"),
    #[br(assert(pos == base_pos))]
    #[bw(calc = *pos)]
    base_pos: PatternCursor,
    idk: PatternCursor,
    idk1: OptionalPatternCursor,
    interpolator: Interpolator,
}

#[binrw]
#[derive(Debug)]
pub struct PathBase {
    version: Version!(1, "Movement Path Base"),
    init: Bool,
    #[brw(if (init.bool))]
    poses: Array<PatternCursor>,
    #[brw(if (init.bool))]
    idk: Option<Bool>,
    #[brw(if (init.bool))]
    idk1: Option<i32>,
    #[brw(if (init.bool))]
    idk2: Option<Bool>,
}

#[binrw]
#[derive(Debug)]
struct Rough {
    version: Version!("Movement Rough"),
    init: Bool,
    pos0: OptionalPatternCursor,
    pos1: OptionalPatternCursor,
    pos2: OptionalPatternCursor,
    pos3: OptionalPatternCursor,
    path: Path,
}

#[binrw]
#[derive(Debug)]
struct Path {
    version: Version!("Path"),
    idk: u32,
    pos: OptionalPatternCursor,
    free_path: FreePath,
    street_path: StreetPath,
}

#[binrw]
#[derive(Debug)]
struct FreePath {
    version: Version!("Movement Free Path"),
    init: Bool,
    path_base: PathBase,
}

#[binrw]
#[derive(Debug)]
struct StreetPath {
    version: Version!("Movement Street Path"),
    idk0: Bool,
    idk1: Bool,
    idk2: Bool,
    idk3: i32,
    pos: OptionalPatternCursor,
    path: Array<PatternCursor>,
    street_ref: Ref<Street>,
    idk4: Bool,
    idk5: Bool,
}

#[binrw]
#[derive(Debug)]
struct Fine {
    init: Bool,
    pos0: OptionalElevationCursor,
    pos1: OptionalElevationCursor,
    pos2: OptionalElevationCursor,
    pos3: OptionalElevationCursor,
    fine_path: FinePath,
}

#[binrw]
#[derive(Debug)]
struct FinePath {
    version: Version!("Movement Fine Path"),
    init: Bool,
    path: Array<ElevationCursor>,
    idk: Bool,
    idk2: i32,
}

#[binrw]
#[derive(Debug, Default)]
struct Interpolator {
    version: Version!("Movement Interpolator"),
    idk1: f32,
    idk2: f32,
    idk3: f32,
}
