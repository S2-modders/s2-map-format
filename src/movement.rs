use crate::helper_structs::*;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct SettlerMovement {
    #[brw(args(1, "MovementMovement"))]
    version: Version,
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
#[derive(Debug, Default)]
pub struct AnimalMovement {
    #[brw(args(1, "Navy Movement"))]
    version: Version,
    path: ResourcePath,
    #[brw(if(version.version > 0))]
    pos: Option<PatternCursor>, //Get this instead in movementBase.pos if None
    movement_base: Base,
}

#[binrw]
#[derive(Debug, Default)]
struct PathBase {
    #[brw(args(1, "Movement Path Base"))]
    version: Version,
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
#[derive(Debug, Default)]
struct ResourcePath {
    #[brw(args(0, "Resources Path"))]
    version: Version,
    base: PathBase,
}

#[binrw]
#[derive(Debug, Default)]
struct Base {
    #[brw(args(0, "Movement Base"))]
    version: Version,
    pos: PatternCursor,
    idk: PatternCursor,
    idk1: PatternCursor,
    interpolator: Interpolator,
}

#[binrw]
#[derive(Debug)]
struct Rough {
    #[brw(args(0, "Movement Rough"))]
    version: Version,
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
    #[brw(args(0, "Path"))]
    version: Version,
    idk: u32,
    pos: PatternCursor,
    free_path: FreePath,
    street_path: StreetPath,
}

#[binrw]
#[derive(Debug)]
struct FreePath {
    #[brw(args(0, "Movement Free Path"))]
    version: Version,
    init: Bool,
    path_base: PathBase,
}

#[binrw]
#[derive(Debug)]
struct StreetPath {
    #[brw(args(0, "Movement Street Path"))]
    version: Version,
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
    #[brw(args(0, "Movement Fine Path"))]
    version: Version,
    init: Bool,
    path: Array<ElevationCursor>,
    idk: Bool,
    idk2: i32,
}

#[binrw]
#[derive(Debug, Default)]
struct Interpolator {
    #[brw(args(0, "Movement Interpolator"))]
    version: Version,
    idk1: f32,
    idk2: f32,
    idk3: f32,
}
