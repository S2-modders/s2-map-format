use crate::Version;
use crate::buildings::Building;
use crate::helper_structs::*;
use crate::player::Player;
use binrw::binrw;
use binrw::helpers::args_iter;

#[binrw]
#[derive(Debug)]
#[brw(import(players: &[Optional<Player>]))]
pub struct Ai {
    version: Version!(2, "AI System"),
    init: Bool,
    #[br(parse_with = args_iter(players.iter().map(|o|o.as_ref())))]
    ai_players: Vec<AiPlayer>,
    tick: u32,
    resource_map: ResourceMap,
    small_resource_map: SmallResourceMap,
    should_tick: Bool,
}

#[binrw]
#[derive(Debug)]
#[brw(import_raw(player: Option<&Player>))]
struct AiPlayer {
    version: Version!(6, "AI Player"),
    init: Bool,
    ai_type: Option<AiType>,
    #[brw(if(init.bool))]
    initialized_ai_player: Option<InitAiPlayer>,
    #[brw(if(player.is_some_and(|p|p.init.bool)))]
    military_map: Option<MilitaryMap>,
}

#[binrw]
#[derive(Debug)]
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
    food_arrangement: FoodArrangement,
    goods_arrangement: GoodsArrangement,
    production_control: ProductionControl,
    conquered_continents: ConqueredContinents,
}

#[binrw]
#[derive(Debug)]
struct ConqueredContinents {
    version: Version!(0, "Ai ConqueredContinents"),
    #[brw(assert(init.bool))]
    init: Bool,
    continent_ids: Array<u32>,
}

#[binrw]
#[derive(Debug)]
struct ProductionControl {
    version: Version!(0, "AI ProductionControl"),
    #[brw(assert(init.bool))]
    init: Bool,
    current: Good,
}

#[binrw]
#[derive(Debug)]
struct GoodsArrangement {
    version: Version!(0, "AI GoodsArrangement"),
    #[brw(assert(init.bool))]
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
    #[brw(assert(init.bool))]
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct LockSmith {
    version: Version!(1, "AI LockSmith"),
    #[brw(assert(init.bool))]
    init: Bool,
    tool_need_ref: Ref<Need>,
    last_tool_produced: Option<Good>,
}

#[binrw]
#[derive(Debug)]
struct NeedCreation {
    version: Version!(0, "AI NeedCreation"),
    #[brw(assert(init.bool))]
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
    #[brw(assert(init.bool))]
    init: Bool,
    curr_flag_idx: u32,
    some_ref: u64,
}

#[binrw]
#[derive(Debug)]
struct AiResources {
    version: Version!(0, "Ai ResourceSystem"),
    #[brw(assert(init.bool))]
    init: Bool,
    needs: NeedRefs,
    expansion_target: ExpansionTarget,
    need: Good,
}

#[binrw]
#[derive(Debug)]
struct AiMilitary {
    version: Version!(8, "Ai MilitarySystem"),
    #[brw(assert(init.bool))]
    init: Bool,
    military_building_creation: MilitaryBuildingCreation,
    military_settings: MilitarySettings,
    attack_system: AttackSystem,
    soldier_creation: SoldierCreation,
    catapult_construction: CatapultConstruction,
    castle_settings: Option<CastleSettings>,
    weapon_production: Option<WeaponProduction>,
    coin_production: Option<CoinProduction>,
    coin_arrangement: Option<CoinArrangement>,
    military_upgrade: Option<MilitaryUpgrade>,
}

#[binrw]
#[derive(Debug)]
struct SoldierCreation {
    version: Version!(0, "AI SoldierCreation"),
    #[brw(assert(init.bool))]
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct MilitaryUpgrade {
    version: Version!(1, "AI MilitaryUpgrade"),
    #[brw(assert(init.bool))]
    init: Bool,
    curr_military_building_idx: u32,
}

#[binrw]
#[derive(Debug)]
struct CoinArrangement {
    version: Version!(0, "AI CoinArrangement"),
    #[brw(assert(init.bool))]
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct CoinProduction {
    version: Version!(0, "AI CoinProduction"),
    #[brw(assert(init.bool))]
    init: Bool,
    idk: u32,
    idk2: f32, //same as in WeaponProduction
}

#[binrw]
#[derive(Debug)]
struct WeaponProduction {
    version: Version!(0, "AI WeaponProduction"),
    #[brw(assert(init.bool))]
    init: Bool,
    idk: u32,
    idk2: f32,
}

#[binrw]
#[derive(Debug)]
struct CastleSettings {
    version: Version!(0, "AI CastleSettings"),
    #[brw(assert(init.bool))]
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct CatapultConstruction {
    version: Version!(0, "AI CatapultConstruction"),
    #[brw(assert(init.bool))]
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
    #[brw(assert(init.bool))]
    init: Bool,
    target_selection: TargetSelection,
    attack_execution: AttackExecution,
    allowed_attack_count: u32,
}

#[binrw]
#[derive(Debug)]
struct TargetSelection {
    version: Version!(0, "Ai AttackTargetSelection"),
    #[brw(assert(init.bool))]
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
    #[brw(assert(init.bool))]
    init: Bool,
    pos: OptionalPatternCursor,
    expansion_target: ExpansionTarget,
}

#[binrw]
#[derive(Debug)]
struct MilitarySettings {
    version: Version!(0, "AI MilitarySettings"),
    #[brw(assert(init.bool))]
    init: Bool,
    tick: CapedU32<50>,
}

#[binrw]
#[derive(Debug)]
struct MilitaryBuildingCreation {
    version: Version!(0, "Ai MilitaryBuildingCreation"),
    #[brw(assert(init.bool))]
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
    #[brw(assert(init.bool))]
    init: Bool,
    target: ExpansionTarget,
    target2: ExpansionTarget,
    expansion_request_type: ExpansionRequestType,
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
    #[brw(assert(init.bool))]
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct Cells {
    version: Version!(3, "Ai CellSystem"),
    #[brw(assert(init.bool))]
    init: Bool,
    cell_depot_creation: CellDepotCreation,
    cell_expansion: CellExpansion,
    cells: Array<Cell>,
    cell_validator: Option<CellValidator>,
    cell_clearing: Option<CellClearing>,
    harbor_creation: Option<HarborCreation>,
}

#[binrw]
#[derive(Debug)]
struct CellDepotCreation {
    version: Version!(0, "Ai CellDepotCreation"),
    #[brw(assert(init.bool))]
    init: Bool,
    order: ConstructionOrder,
}

#[binrw]
#[derive(Debug)]
struct CellExpansion {
    version: Version!(0, "Ai CellExpansion"),
    #[brw(assert(init.bool))]
    init: Bool,
    target: ExpansionTarget,
}

#[binrw]
#[derive(Debug)]
struct CellValidator {
    version: Version!(0, "AI CellValidator"),
    #[brw(assert(init.bool))]
    init: Bool,
    start: MapIdxPos,
    end: MapIdxPos,
}

#[binrw]
#[derive(Debug)]
struct CellClearing {
    version: Version!(0, "AI CellClearing"),
    #[brw(assert(init.bool))]
    init: Bool,
    start: MapIdxPos,
    end: MapIdxPos,
    order: ConstructionOrder,
}

#[binrw]
#[derive(Debug)]
struct HarborCreation {
    version: Version!(1, "Ai HarborCreation"),
    #[brw(assert(init.bool))]
    init: Bool,
    order: ConstructionOrder,
    active: Bool,
}

#[binrw]
#[derive(Debug)]
struct Cell {
    version: Version!(5, "Ai Cell"),
    #[brw(assert(init.bool))]
    init: Bool,
    id: Uuid,
    cell_idx_pos: MapIdxPos,
    pos: PatternCursor,
    depots: AiBuildingRefs,
    civil_buildings: AiBuildingRefs,
    military_buildings: AiBuildingRefs,
    productions: ProductionRefs,
    needs: NeedRefs,
    expansion_maybe: CellConstruction,
    depot_constructon: CellConstruction,
    cell_full: CellFull,
    destruction_site: CellConstruction,
    destruction_size: u32,
    aibuilding_ref: Ref<AiBuilding>,
    harbors: AiBuildingRefs,
    continent_id: u32,
    idk: CellConstruction,
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
    #[brw(assert(init.bool))]
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
    time: Time,
    prio: f32,
}

#[binrw]
#[derive(Debug)]
struct TerritoryMap {
    version: Version!(0, "Ai TerritoryMap"),
    #[brw(assert(init.bool))]
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
    catapults_constructioned: i32,
    military_ic: i32,
}

#[binrw]
#[derive(Debug)]
struct Production {
    version: Version!(0, "Ai Production"),
    #[brw(assert(init.bool))]
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
    #[brw(assert(init.bool))]
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
    #[brw(assert(init.bool))]
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
    #[brw(assert(init.bool))]
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
    #[brw(assert(init.bool))]
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
    #[brw(assert(init.bool))]
    init: Bool,
    order: ConstructionOrder,
    need: Ref<Need>,
}

#[binrw]
#[derive(Debug)]
struct NeedsArrangement {
    version: Version!(0, "AI NeedsArrangement"),
    #[brw(assert(init.bool))]
    init: Bool,
    target: ExpansionTarget,
}

#[binrw]
#[derive(Debug)]
struct ExpansionTarget {
    version: Version!(0, "Ai ExpansionTarget"),
    target_type: ExpansionTargetType,
    target: OptionalPatternCursor,
    time: Time,
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
    #[brw(assert(init.bool))]
    init: Bool,
    order: ConstructionOrder,
    order_type: ConstructionOrderType,
    building_construction: BuildingConstruction,
    street_construction: StreetConstruction,
    forester_construction: ForesterConstruction,
    street_optimizer: StreetOptimizer,
    street_route_optimizer: StreetRouteOptimizer,
    ship_construction: ShipConstruction,
    hunter_construction: HunterConstruction,
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
    #[brw(assert(init.bool))]
    init: Bool,
    curr_flag_idx: i32,
    target_flag: u64, //Flag ref
    idk: u32,
    tried_flag_poses: Array<PatternCursor>,
    idk2: Array<(u32, Uuid)>,
    idk3: OptionalPatternCursor,
    idk4: u32,
    idk5: u32,
    idk6: u32,
    idk7: u32,
    idk8: OptionalPatternCursor,
}

#[binrw]
#[derive(Debug)]
struct ForesterConstruction {
    version: Version!(1, "AI ForesterConstruction"),
    #[brw(assert(init.bool))]
    init: Bool,
    order: ConstructionOrder,
    curr_cell_idx: u32,
}

#[binrw]
#[derive(Debug)]
struct StreetOptimizer {
    version: Version!(1, "AI StreetOptimizer"),
    #[brw(assert(init.bool))]
    init: Bool,
    idx: u32,
}

#[binrw]
#[derive(Debug)]
struct StreetRouteOptimizer {
    version: Version!(1, "AI StreetRouteOptimizer"),
    #[brw(assert(init.bool))]
    init: Bool,
    aibuilding_ref: u64,
    poses: Array<PatternCursor>,
    idk: i32,
    idk2: u32,
    idk3: i32,
    idk4: u32,
}

#[binrw]
#[derive(Debug)]
struct ShipConstruction {
    version: Version!(0, "AI ShipConstruction"),
    #[brw(assert(init.bool))]
    init: Bool,
    order: ConstructionOrder,
    aibuilding_ref: Ref<AiBuilding>, //TODO
}

#[binrw]
#[derive(Debug)]
struct HunterConstruction {
    version: Version!(0, "AI ConstructionHunter"),
    #[brw(assert(init.bool))]
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
    building_pos: OptionalPatternCursor,
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
    curr_tick_pos: MapIdxPos,
    pos: OptionalPatternCursor,
    idk2: u32,
    building_type: BuildingType,
    priority: u32,
    ticked_seconds: Time,
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
    final_pos: OptionalPatternCursor,
    final_pos2: OptionalPatternCursor,
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
    #[brw(assert(init.bool))]
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
    #[brw(assert(init.bool))]
    init: Bool,
    width: u32,
    height: u32,
    #[br(count = width * height)]
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
    #[brw(assert(init.bool))]
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
