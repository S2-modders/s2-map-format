use crate::Version;
use crate::buildings::Building;
use crate::{helper_structs::*, player::OptionalPlayer};
use binrw::binrw;
use bounded_integer::BoundedU32;
use strum::*;

#[binrw]
#[derive(Debug)]
#[brw(import(players: &[OptionalPlayer]))]
pub struct Ai {
    version: Version!(2, "AI System"),
    init: Bool,
    #[brw(args(version.version, players))]
    ai_players: [AiPlayer; PlayerId::COUNT],
    tick: u32,
    resource_map: ResourceMap,
    small_resource_map: SmallResourceMap,
    should_tick: Bool,
}

#[binrw]
#[derive(Debug)]
#[brw(import(aiVersion: BoundedU32<0, 2>, players: &[OptionalPlayer]))]
struct AiPlayer {
    version: Version!(6, "AI Player"),
    init: Bool,
    #[brw(if(version.version > 1))]
    #[br(dbg)]
    ai_type: Option<AiType>,
    #[brw(if(version.version < 6 || init.bool))]
    #[brw(args(version.version))]
    initialized_ai_player: Option<InitAiPlayer>,
    #[brw(if(aiVersion > 1 && players[0].player.as_ref().is_some_and(|p|p.init.bool)))] //TODO
    #[br(try)]
    military_map: Option<MilitaryMap>,
}
#[binrw]
#[derive(Debug)]
#[brw(import(version: BoundedU32<0, 6>))]
struct InitAiPlayer {
    headquarters: PatternCursor,
    construction: Construction,
    needs_arrangement: NeedsArrangement,
    needs_construction: NeedsConstruction,
    resource_map_adds: ResourceMapAdds,
    small_resource_map_adds: SmallResourceMapAdds,
    buildings: Array<AiBuilding>,
    general_needs: Array<Need>,
    food_needs: Array<Need>,
    productions: Array<Production>,
    territory_map: TerritoryMap,
    cells: Cells,
    expansion: Expansion,
    military: AiMilitary,
    resources: AiResources,
    destruction: Destruction,
    need_creation: NeedCreation,
    lock_smith: LockSmith,
    #[brw(if(version > 0))]
    food_arrangement: Option<FoodArrangement>,
    #[brw(if(version > 2))]
    goods_arrangement: Option<GoodsArrangement>,
    #[brw(if(version > 2))]
    production_control: Option<ProductionControl>,
    #[brw(if(version > 4))]
    conquered_continents: Option<ConqueredContinents>,
}

#[binrw]
#[derive(Debug)]
struct ConqueredContinents {
    version: Version!(0, "Ai ConqueredContinents"),
    init: Bool,
    continent_ids: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct ProductionControl {
    version: Version!(0, "AI ProductionControl"),
    init: Bool,
    current: Good,
}

#[binrw]
#[derive(Debug)]
struct GoodsArrangement {
    version: Version!(0, "AI GoodsArrangement"),
    init: Bool,
    stage: GoodsArrangementStage,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
enum GoodsArrangementStage {
    DepotSettings = 0,
    HarborSettings = 1,
    MilitaryArrangement1 = 2,
    MoveSoldiersFromDepots1 = 3,
    Nothing1 = 4,
    Nothing2 = 5,
    Nothing3 = 6,
    MilitaryArrangement2 = 7,
    MoveSoldiersFromDepots2 = 8,
    Nothing4 = 9,
}

#[binrw]
#[derive(Debug)]
struct FoodArrangement {
    version: Version!(0, "AI FoodArrangement"),
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct LockSmith {
    version: Version!(1, "AI LockSmith"),
    init: Bool,
    tool_need_ref: Ref<Need>,
    #[brw(if(version.version > 0))]
    last_tool_produced: Option<Good>,
}

#[binrw]
#[derive(Debug)]
struct NeedCreation {
    version: Version!(0, "AI NeedCreation"),
    init: Bool,
    state: NeedCreationState,
    /// the time the when the next check if enough stone and plank is present is executed (in
    /// 10mins)
    next_stone_and_plank_check_time: u32,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
/// Represents what the ai currently needs/is working on
enum NeedCreationState {
    None = 0,
    StoneAndPlank = 1,
    SwordAndBear = 2,
    Donkey = 3,
    Coin = 4,
}

#[binrw]
#[derive(Debug)]
struct Destruction {
    version: Version!(2, "Ai DestructionSystem"),
    init: Bool,
    #[brw(if(version.version > 1))]
    curr_flag_idx: u32,
    #[brw(if(version.version < 2))]
    idk: u64,
    #[brw(if(version.version > 0))]
    some_ref: u64,
}

#[binrw]
#[derive(Debug)]
struct AiResources {
    version: Version!(0, "Ai ResourceSystem"),
    init: Bool,
    needs: NeedRefs,
    expansion_target: ExpansionTarget,
    need: Good,
}

#[binrw]
#[derive(Debug)]
struct AiMilitary {
    version: Version!(8, "Ai MilitarySystem"),
    init: Bool,
    military_building_creation: MilitaryBuildingCreation,
    military_settings: MilitarySettings,
    #[brw(if(version.version < 7))]
    idk: u32,
    attack_system: AttackSystem,
    soldier_creation: SoldierCreation,
    catapult_construction: CatapultConstruction,
    #[brw(if(version.version > 2))]
    castle_settings: Option<CastleSettings>,
    #[brw(if(version.version > 3))]
    weapon_production: Option<WeaponProduction>,
    #[brw(if(version.version > 4))]
    coin_production: Option<CoinProduction>,
    #[brw(if(version.version > 5))]
    coin_arrangement: Option<CoinArrangement>,
    #[brw(if(version.version > 7))]
    military_upgrade: Option<MilitaryUpgrade>,
}

#[binrw]
#[derive(Debug)]
struct SoldierCreation {
    version: Version!(0, "AI SoldierCreation"),
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct MilitaryUpgrade {
    version: Version!(1, "AI MilitaryUpgrade"),
    init: Bool,
    #[brw(if(version.version > 0))]
    curr_military_building_idx: u32,
    #[brw(if(version.version == 0))]
    idk: u64,
}

#[binrw]
#[derive(Debug)]
struct CoinArrangement {
    version: Version!(0, "AI CoinArrangement"),
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct CoinProduction {
    version: Version!(0, "AI CoinProduction"),
    init: Bool,
    idk: u32,
    idk2: f32, //same as in WeaponProduction
}

#[binrw]
#[derive(Debug)]
struct WeaponProduction {
    version: Version!(0, "AI WeaponProduction"),
    init: Bool,
    idk: u32,
    idk2: f32,
}

#[binrw]
#[derive(Debug)]
struct CastleSettings {
    version: Version!(0, "AI CastleSettings"),
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct CatapultConstruction {
    version: Version!(0, "AI CatapultConstruction"),
    init: Bool,
    curr_pos: MapIdxPos,
    best_score: f32,
    best_pos: MapIdxPos,
    max_iterations: i32,
    order: ConstructionOrder,
}

#[binrw]
#[derive(Debug)]
struct AttackSystem {
    version: Version!(1, "Ai AttackSystem"),
    init: Bool,
    target_selection: TargetSelection,
    attack_execution: AttackExecution,
    #[brw(if(version.version > 0))]
    allowed_attack_count: u32,
}

#[binrw]
#[derive(Debug)]
struct TargetSelection {
    version: Version!(0, "Ai AttackTargetSelection"),
    init: Bool,
    target: AttackTarget,
}

#[binrw]
#[derive(Debug)]
struct AttackTarget {
    version: Version!(0, "Ai AttackTarget"),
    target_pos: MapIdxPos,
    distance: u32,
    score: u32,
    target_owner: Optional<PlayerId>,
}

#[binrw]
#[derive(Debug)]
struct AttackExecution {
    version: Version!(1, "Ai AttackExecution"),
    init: Bool,
    #[brw(if(version.version > 0))]
    pos: Option<PatternCursor>,
    #[brw(if(version.version > 0))]
    expansion_target: Option<ExpansionTarget>,
}

#[binrw]
#[derive(Debug)]
struct MilitarySettings {
    version: Version!(0, "AI MilitarySettings"),
    init: Bool,
    tick: CapedU32<50>,
}

#[binrw]
#[derive(Debug)]
struct MilitaryBuildingCreation {
    version: Version!(0, "Ai MilitaryBuildingCreation"),
    init: Bool,
    order: ConstructionOrder,
    target: ExpansionTarget,
    tick: CapedU32<3>,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct Expansion {
    version: Version!(1, "Ai ExpansionSystem"),
    init: Bool,
    target: ExpansionTarget,
    target2: ExpansionTarget,
    expansion_request_type: ExpansionRequestType,
    #[brw(if(version.version > 0))]
    expedition: Option<Expedition>,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
enum ExpansionRequestType {
    None = 0,
    Resources = 1,
    Cells = 2,
    Needs = 3,
    Military = 4,
}

#[binrw]
#[derive(Debug)]
struct Expedition {
    version: Version!(0, "AI Expedition"),
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct Cells {
    version: Version!(3, "Ai CellSystem"),
    init: Bool,
    cell_depot_creation: CellDepotCreation,
    cell_expansion: CellExpansion,
    cells: Array<Cell>,
    #[brw(if(version.version > 0))]
    cell_validator: Option<CellValidator>,
    #[brw(if(version.version > 1))]
    cell_clearing: Option<CellClearing>,
    #[brw(if(version.version > 2))]
    harbor_creation: Option<HarborCreation>,
}

#[binrw]
#[derive(Debug)]
struct CellDepotCreation {
    version: Version!(0, "Ai CellDepotCreation"),
    init: Bool,
    order: ConstructionOrder,
}

#[binrw]
#[derive(Debug)]
struct CellExpansion {
    version: Version!(0, "Ai CellExpansion"),
    init: Bool,
    target: ExpansionTarget,
}

#[binrw]
#[derive(Debug)]
struct CellValidator {
    version: Version!(0, "AI CellValidator"),
    init: Bool,
    start: MapIdxPos,
    end: MapIdxPos,
}

#[binrw]
#[derive(Debug)]
struct CellClearing {
    version: Version!(0, "AI CellClearing"),
    init: Bool,
    start: MapIdxPos,
    end: MapIdxPos,
    order: ConstructionOrder,
}

#[binrw]
#[derive(Debug)]
struct HarborCreation {
    version: Version!(1, "Ai HarborCreation"),
    init: Bool,
    order: ConstructionOrder,
    #[brw(if(version.version > 0, Bool { bool: true }))]
    active: Bool,
}

#[binrw]
#[derive(Debug)]
struct Cell {
    version: Version!(5, "Ai Cell"),
    init: Bool,
    id: Uuid,
    cell_idx_pos: MapIdxPos,
    pos: PatternCursor,
    #[brw(if(version.version > 4))]
    depots: Option<AiBuildingRefs>,
    civil_buildings: AiBuildingRefs,
    military_buildings: AiBuildingRefs,
    productions: ProductionRefs,
    needs: NeedRefs,
    expansion_maybe: CellConstruction,
    depot_constructon: CellConstruction,
    #[brw(if(version.version > 0))]
    cell_full: Option<CellFull>,
    destruction_site: CellConstruction,
    destruction_size: u32,
    aibuilding_ref: Ref<AiBuilding>,
    #[brw(if(version.version == 0))]
    idk0: i32,
    #[brw(if(version.version > 3))]
    harbors: Option<AiBuildingRefs>,
    #[brw(if(version.version > 1))]
    continent_id: Option<u32>,
    #[brw(if(version.version > 1))]
    idk: Option<CellConstruction>,
    #[brw(if(version.version > 2))]
    aibuilding_ref2: Ref<AiBuilding>,
}

impl Ided for Cell {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct AiBuildingRefs {
    version: Version!(0, "Ai ReferncesBuilding"),
    aibuildings: Array<Ref<AiBuilding>>,
}

#[binrw]
#[derive(Debug)]
struct CellFull {
    version: Version!(0, "AI CellFull"),
    init: Bool,
    elements: Array<CellFullElement>,
}

#[binrw]
#[derive(Debug)]
struct CellFullElement {
    good: Good,
    version: Version!(0, "AI CellFullElement"),
    cell_construction: CellConstruction,
    idk: i32,
}

#[binrw]
#[derive(Debug)]
struct CellConstruction {
    version: Version!(1, "Ai Cell"), //TODO why not 0?
    init: Bool,
    time: f32,
    prio: f32,
}

#[binrw]
#[derive(Debug)]
struct TerritoryMap {
    version: Version!(0, "Ai TerritoryMap"),
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    elements: Vec<TerritoryMapElement>,
}

#[binrw]
#[derive(Debug)]
struct TerritoryMapElement {
    version: Version!(2, "AI TerritoryMap Element"),
    id: Uuid,
    idk: i32,
    attack_fc: i32,
    #[brw(if(version.version > 0))]
    catapults_constructioned: Option<i32>,
    #[brw(if(version.version > 1))]
    military_ic: Option<i32>,
}

#[binrw]
#[derive(Debug)]
struct Production {
    version: Version!(0, "Ai Production"),
    init: Bool,
    id: Uuid,
    product: Good,
    cell_ref: Ref<Cell>,
    need_ref: Ref<Need>,
    aibuilding_ref: Ref<AiBuilding>,
}

impl Ided for Production {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct Need {
    version: Version!(0, "Ai Need"),
    init: Bool,
    id: Uuid,
    need_type: Good,
    cell_ref: Ref<Cell>,
    production_ref: Ref<Production>,
    aibuilding_ref: Ref<AiBuilding>,
    pos_resource_element_adds: MapIdxPos,
    is_in_resource_system: Bool,
    child: Ref<Need>,
    parent: Ref<Need>,
}

impl Ided for Need {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct AiBuilding {
    version: Version!(1, "Ai Building"), //TODO: why?
    init: Bool,
    id: Uuid,
    building_ref: Ref<Building>,
    cell_ref: Ref<Cell>,
    need_refs: NeedRefs,
    production_refs: ProductionRefs,
}

impl Ided for AiBuilding {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct NeedRefs {
    version: Version!(0, "Ai ReferncesNeed"),
    needs: Array<Ref<Need>>,
}

#[binrw]
#[derive(Debug)]
struct ProductionRefs {
    version: Version!(0, "Ai ReferncesProduction"),
    productions: Array<Ref<Production>>,
}

#[binrw]
#[derive(Debug)]
struct SmallResourceMapAdds {
    version: Version!(0, "Ai LowResResourceMapAdds"),
    init: Bool,
    needs: Array<SmallResourceMapAddsElement>,
}

#[binrw]
#[derive(Debug)]
struct SmallResourceMapAddsElement {
    version: Version!(0, "Ai LowResResourceMapAddsElement"),
    idk: i32,
}

#[binrw]
#[derive(Debug)]
struct ResourceMapAdds {
    version: Version!(0, "Ai ResourceMapAdds"),
    init: Bool,
    needs: Array<ResourceMapAddsElement>,
}

#[binrw]
#[derive(Debug)]
struct ResourceMapAddsElement {
    version: Version!(0, "Ai ResourceMapAddsElement"),
    need: Ref<Need>,
    idk: Bool,
}

#[binrw]
#[derive(Debug)]
struct NeedsConstruction {
    version: Version!(0, "AI NeedsConstruction"),
    init: Bool,
    order: ConstructionOrder,
    need: Ref<Need>,
}

#[binrw]
#[derive(Debug)]
struct NeedsArrangement {
    version: Version!(0, "AI NeedsArrangement"),
    init: Bool,
    target: ExpansionTarget,
}

#[binrw]
#[derive(Debug)]
struct ExpansionTarget {
    version: Version!(0, "Ai ExpansionTarget"),
    target_type: ExpansionTargetType,
    target: PatternCursor,
    time: f32,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
enum ExpansionTargetType {
    None = 0,
    ToPosition = 1,
    NearPosition = 2,
    UnallocatedPosition = 3,
}

#[binrw]
#[derive(Debug)]
struct Construction {
    version: Version!(3, "Ai ConstructionSystem"),
    init: Bool,
    order: ConstructionOrder,
    order_type: ConstructionOrderType,
    building_construction: BuildingConstruction,
    street_construction: StreetConstruction,
    forester_construction: ForesterConstruction,
    street_optimizer: StreetOptimizer,
    street_route_optimizer: StreetRouteOptimizer,
    #[brw(if(version.version > 0))]
    ship_construction: Option<ShipConstruction>,
    #[brw(if(version.version > 1))]
    hunter_construction: Option<HunterConstruction>,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
enum ConstructionOrderType {
    None = 0,
    Depot = 1,
    Needs = 2,
    Military = 3,
    Forester = 4,
    Catapult = 5,
    CellClearing = 6,
    Harbor = 7,
    Ship = 8,
    Hunter = 9,
}

#[binrw]
#[derive(Debug)]
struct StreetConstruction {
    version: Version!(2, "AI StreetConstruction"),
    init: Bool,
    #[brw(if(version.version == 0))]
    idk0: u64,
    #[brw(if(version.version > 0))]
    curr_flag_idx: i32,
    #[brw(if(version.version > 1))]
    target_flag: u64, //Flag ref
    idk: u32,
    tried_flag_poses: Array<PatternCursor>,
    idk2: Array<(u32, Uuid)>,
    idk3: PatternCursor,
    idk4: u32,
    idk5: u32,
    idk6: u32,
    idk7: u32,
    idk8: PatternCursor,
}

#[binrw]
#[derive(Debug)]
struct ForesterConstruction {
    version: Version!(1, "AI ForesterConstruction"),
    init: Bool,
    order: ConstructionOrder,
    #[br(if(version.version > 0))]
    curr_cell_idx: u32,
}

#[binrw]
#[derive(Debug)]
struct StreetOptimizer {
    version: Version!(1, "AI StreetOptimizer"),
    init: Bool,
    #[brw(if(version.version > 0))]
    idx: u32,
    #[brw(if(version.version == 0))]
    idk: u64,
}

#[binrw]
#[derive(Debug)]
struct StreetRouteOptimizer {
    version: Version!(1, "AI StreetRouteOptimizer"),
    init: Bool,
    aibuilding_ref: u64,
    poses: Array<PatternCursor>,
    idk: i32,
    idk2: u32,
    idk3: i32,
    #[brw(if(version.version > 0))]
    idk4: u32,
}

#[binrw]
#[derive(Debug)]
struct ShipConstruction {
    version: Version!(0, "AI ShipConstruction"),
    init: Bool,
    order: ConstructionOrder,
    aibuilding_ref: Ref<AiBuilding>, //TODO
}

#[binrw]
#[derive(Debug)]
struct HunterConstruction {
    version: Version!(0, "AI ConstructionHunter"),
    init: Bool,
    order: ConstructionOrder,
    cell_idk: u32, //TODO
}

#[binrw]
#[derive(Debug)]
struct BuildingConstruction {
    version: Version!(0, "AI BuildingConstruction"),
    init: Bool,
    state: ConstructionState,
    order: ConstructionOrder,
    searcher: SearchConstructionPlace,
    building_pos: PatternCursor,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
enum ConstructionState {
    None = 0,
    CheckingResources = 1,
    SearchingBuildingPosition = 2,
    PlacingConstructionSite = 3,
    FailedNoResources = 4,
    FailedNoPosition = 5,
    Done = 6,
}

#[binrw]
#[derive(Debug)]
struct ConstructionOrder {
    version: Version!(0, "Ai ConstructionOrder"),
    idk: u32,
    curr_tick_pos: (u32, u32),
    pos: PatternCursor,
    idk2: u32,
    building_type: BuildingType,
    priority: u32,
    ticked_seconds: f32,
}

#[binrw]
#[derive(Debug)]
struct SearchConstructionPlace {
    version: Version!(0, "AI SearchConstructionPlace"),
    building_type: BuildingType,
    found: Bool,
    pos_iterator: CellPositionIterator,
    max_iterations: i32,
    curr_iteration: i32,
    curr_direction: Direction,
    some_upperbound: i32,
    final_pos: PatternCursor,
    final_pos2: PatternCursor,
    last_pos: PatternCursor,
    curr_iteration2: i32,
    score: f32,
    some_upperbound2: u32,
}

#[binrw]
#[derive(Debug)]
struct CellPositionIterator {
    version: Version!(0, "AI CellPositionItertator"),
    curr_tick_pos: (u32, u32),
    pos: PatternCursor,
    counter1: u32,
    counter2: u32,
}

#[binrw]
#[derive(Debug)]
struct MilitaryMap {
    version: Version!(0, "Ai Military Map"),
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    elements: Vec<MilitaryMapElement>,
    curr_tick_pos: MapIdxPos,
}

#[binrw]
#[derive(Debug)]
struct ResourceMap {
    version: Version!(1, "Ai ResourceMap"), //TODO why not 0?
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    #[brw(if(version.version == 0))]
    elements: Vec<ResourceMapElement>,
    #[br(count = width * height)]
    #[brw(if(version.version > 0))]
    element_vecs: Vec<ResourceMapElements>,
    tick_pos: MapIdxPos,
    tick_pos2: MapIdxPos,
    len: u32,
}

#[binrw]
#[derive(Debug)]
struct ResourceMapElements {
    version: Version!(0, "AI ResourceMapElements"),
    elements: Array<ResourceMapElement>,
}

#[binrw]
#[derive(Debug)]
struct ResourceMapElement {
    version: Version!(0, "AI ResourceMapElement"),
    good: Good,
    pattern_type: u32, //idk
    deposit_number: u32,
    continent_id: u32,
}

#[binrw]
#[derive(Debug)]
struct SmallResourceMap {
    version: Version!(0, "Ai LowResResourceMap"),
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
    element_vecs: Vec<ResourceMapElements>,
    curr_tick_pos: MapIdxPos,
}

#[binrw]
#[derive(Debug)]
struct MilitaryMapElement {
    version: Version!(0, "AI Military Map Element"),
    treat: (u32, u32), // second one is with unfinised buildings
    ai_score: u32,
}
