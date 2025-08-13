use crate::helper_structs::*;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct SettlerMovement {
    #[brw(args("MovementMovement"))]
    version: Version<1>,
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
    #[brw(args("Navy Movement"))]
    version: Version<1>,
    path: ResourcePath,
    #[brw(if(version.version > 0))]
    pos: Option<PatternCursor>, //Get this instead in movementBase.pos if None
    movement_base: Base,
}

#[binrw]
#[derive(Debug)]
struct PathBase {
    #[brw(args("Movement Path Base"))]
    version: Version<1>,
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
    #[brw(args("Resources Path"))]
    version: Version<0>,
    base: PathBase,
}

#[binrw]
#[derive(Debug, Default)]
struct Base {
    #[brw(args("Movement Base"))]
    version: Version<0>,
    pos: PatternCursor,
    idk: PatternCursor,
    idk1: PatternCursor,
    interpolator: Interpolator,
}

#[binrw]
#[derive(Debug)]
struct Rough {
    #[brw(args("Movement Rough"))]
    version: Version<0>,
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
    #[brw(args("Path"))]
    version: Version<0>,
    idk: u32,
    pos: PatternCursor,
    free_path: FreePath,
    street_path: StreetPath,
}

#[binrw]
#[derive(Debug)]
struct FreePath {
    #[brw(args("Movement Free Path"))]
    version: Version<0>,
    init: Bool,
    path_base: PathBase,
}

#[binrw]
#[derive(Debug)]
struct StreetPath {
    #[brw(args("Movement Street Path"))]
    version: Version<0>,
    idk0: Bool,
    idk1: Bool,
    idk2: Bool,
    idk3: i32,
    pos: PatternCursor,
    path: Array<PatternCursor>,
    street_ref: Uuid,
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
    #[brw(args("Movement Fine Path"))]
    version: Version<0>,
    init: Bool,
    path: Array<ElevationCursor>,
    idk: Bool,
    idk2: i32,
}

#[binrw]
#[derive(Debug, Default)]
struct Interpolator {
    #[brw(args("Movement Interpolator"))]
    version: Version<0>,
    idk1: f32,
    idk2: f32,
    idk3: f32,
}
