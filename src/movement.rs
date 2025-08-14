use crate::Version;
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
    idk2: Bool,
    #[brw(if(version.version > 0))]
    pos: Option<PatternCursor>,
}

#[binrw]
#[derive(Debug)]
pub struct AnimalMovement {
    version: Version!(1, "Navy Movement"),
    path: ResourcePath,
    #[brw(if(version.version > 0))]
    pos: Option<PatternCursor>, //Get this instead in movementBase.pos if None
    movement_base: Base,
}

#[binrw]
#[derive(Debug)]
pub struct NavyPath {
    version: Version!(0, "Navy Path"),
    path_base: PathBase,
}

#[binrw]
#[derive(Debug)]
struct PathBase {
    version: Version!(1, "Movement Path Base"),
    init: Bool,
    #[brw(if (init.bool || version.version == 0))]
    poses: Array<PatternCursor>,
    #[brw(if (init.bool || version.version == 0))]
    idk: Bool,
    #[brw(if (init.bool || version.version == 0))]
    idk1: i32,
    #[brw(if (init.bool || version.version == 0))]
    idk2: Bool,
}

#[binrw]
#[derive(Debug)]
struct ResourcePath {
    version: Version!(0, "Resources Path"),
    base: PathBase,
}

#[binrw]
#[derive(Debug, Default)]
struct Base {
    version: Version!(0, "Movement Base"),
    pos: PatternCursor,
    idk: PatternCursor,
    idk1: PatternCursor,
    interpolator: Interpolator,
}

#[binrw]
#[derive(Debug)]
struct Rough {
    version: Version!(0, "Movement Rough"),
    init: Bool,
    pos0: PatternCursor,
    pos1: PatternCursor,
    pos2: PatternCursor,
    pos3: PatternCursor,
    path: Path,
}

#[binrw]
#[derive(Debug)]
struct Path {
    version: Version!(0, "Path"),
    idk: u32,
    pos: PatternCursor,
    free_path: FreePath,
    street_path: StreetPath,
}

#[binrw]
#[derive(Debug)]
struct FreePath {
    version: Version!(0, "Movement Free Path"),
    init: Bool,
    path_base: PathBase,
}

#[binrw]
#[derive(Debug)]
struct StreetPath {
    version: Version!(0, "Movement Street Path"),
    idk0: Bool,
    idk1: Bool,
    idk2: Bool,
    idk3: i32,
    pos: PatternCursor,
    path: Array<PatternCursor>,
    street_ref: Ref<Street>,
    idk4: Bool,
    idk5: Bool,
}

#[binrw]
#[derive(Debug)]
struct Fine {
    init: Bool,
    pos0: ElevationCursor,
    pos1: ElevationCursor,
    pos2: ElevationCursor,
    pos3: ElevationCursor,
    movement_fine_path: FinePath,
}

#[binrw]
#[derive(Debug)]
struct FinePath {
    version: Version!(0, "Movement Fine Path"),
    init: Bool,
    path: Array<ElevationCursor>,
    idk: Bool,
    idk2: i32,
}

#[binrw]
#[derive(Debug, Default)]
struct Interpolator {
    version: Version!(0, "Movement Interpolator"),
    idk1: f32,
    idk2: f32,
    idk3: f32,
}
