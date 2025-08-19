use crate::Version;
use crate::VersionI;
use crate::Versioned;
use crate::VersionedI;
use crate::buildings::Building;
use crate::helper_structs::*;
use crate::net::Flag;
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
    pub ai_players: Vec<AiPlayer>,
    tick: u32, //TODO
    resource_map: ResourceMap,
    small_resource_map: SmallResourceMap,
    should_tick: Bool,
}

#[binrw]
#[derive(Debug)]
#[brw(import_raw(player: Option<&Player>))]
pub struct AiPlayer {
    version: Version!(6, "AI Player"),
    init: Bool,
    pub ai_type: Optional<AiType>,
    #[brw(if(init.bool))]
    pub initialized_ai_player: Option<InitAiPlayer>,
    #[brw(if(player.is_some_and(|p|p.init.bool)))]
    pub military_map: Option<MilitaryMap>,
}

#[binrw]
#[derive(Debug)]
pub struct InitAiPlayer {
    headquarters: PatternCursor,
    construction: Construction,
    needs_arrangement: VersionedI!("AI NeedsArrangement", ExpansionTarget),
    needs_construction: NeedsConstruction,
    resource_map_adds: VersionedI!("Ai ResourceMapAdds", Array<ResourceMapAddsElement>),
    small_resource_map_adds: VersionedI!(
        0,
        "Ai LowResResourceMapAdds",
        Array<SmallResourceMapAddsElement>
    ),

    pub buildings: Array<AiBuilding>,
    pub general_needs: Array<Need>,
    pub food_needs: Array<Need>,
    pub productions: Array<Production>,
    territory_map: VersionedI!("Ai TerritoryMap", Map<TerritoryMapElement>),
    pub cells: Cells,
    expansion: Expansion,
    military: AiMilitary,
    resources: AiResources,
    destruction: Destruction,
    need_creation: NeedCreation,
    lock_smith: LockSmith,
    food_arrangement: VersionI!("AI FoodArrangement"),
    goods_arrangement: VersionedI!("AI GoodsArrangement", GoodsArrangementStage),
    production_control: VersionedI!("AI ProductionControl", GoodOrSettler),
    conquered_continents: VersionedI!("Ai ConqueredContinents", Array<u32>),
}

impl Positioned<TerritoryMapElement> for Map<TerritoryMapElement> {
    fn at(&self, x: usize, y: usize) -> &TerritoryMapElement {
        &self.grid[(x, y)]
    }
}

#[binrw]
#[brw(repr = u32)]
#[derive(Debug)]
pub enum AiType {
    Weak = 1,
    Normal = 2,
    Strong = 3,
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
struct LockSmith {
    version: VersionI!(1, "AI LockSmith"),
    tool_need_ref: OptRef<Need>,
    last_tool_produced: Optional<Good>,
}

#[binrw]
#[derive(Debug)]
struct NeedCreation {
    version: VersionI!("AI NeedCreation"),
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
    version: VersionI!(2, "Ai DestructionSystem"),
    curr_flag_idx: Idx<Flag>,
    some_ref: u64,
}

#[binrw]
#[derive(Debug)]
struct AiResources {
    version: VersionI!("Ai ResourceSystem"),
    needs: Versioned!("Ai ReferncesNeed", Array<Ref<Need>>),
    expansion_target: ExpansionTarget,
    need: Optional<Good>,
}

#[binrw]
#[derive(Debug)]
struct AiMilitary {
    version: VersionI!(8, "Ai MilitarySystem"),
    military_building_creation: MilitaryBuildingCreation,
    military_settings: MilitarySettings,
    attack_system: AttackSystem,
    soldier_creation: VersionI!("AI SoldierCreation"),
    catapult_construction: CatapultConstruction,
    castle_settings: VersionI!("AI CastleSettings"),
    weapon_production: WeaponProduction,
    coin_production: CoinProduction,
    coin_arrangement: VersionI!("AI CoinArrangement"),
    military_upgrade: MilitaryUpgrade,
}

#[binrw]
#[derive(Debug)]
struct MilitaryUpgrade {
    version: VersionI!(1, "AI MilitaryUpgrade"),
    curr_military_building_idx: Idx<Building>,
}

#[binrw]
#[derive(Debug)]
struct CoinProduction {
    version: VersionI!("AI CoinProduction"),
    idk: u32,
    idk2: f32, //same as in WeaponProduction
}

#[binrw]
#[derive(Debug)]
struct WeaponProduction {
    version: VersionI!("AI WeaponProduction"),
    idk: u32,
    idk2: f32,
}

#[binrw]
#[derive(Debug)]
struct CatapultConstruction {
    version: VersionI!("AI CatapultConstruction"),
    curr_pos: MapIdxPos<TerritoryMapElement, Map<TerritoryMapElement>>,
    best_score: f32,
    best_pos: MapIdxPos<TerritoryMapElement, Map<TerritoryMapElement>>,
    max_iterations: i32,
    order: ConstructionOrder,
}

#[binrw]
#[derive(Debug)]
struct AttackSystem {
    version: VersionI!(1, "Ai AttackSystem"),
    target_selection: VersionedI!("Ai AttackTargetSelection", AttackTarget),
    attack_execution: AttackExecution,
    allowed_attack_count: Optional<u32>,
}

#[binrw]
#[derive(Debug)]
struct AttackTarget {
    version: Version!("Ai AttackTarget"),
    target_pos: MapIdxPos<TerritoryMapElement, Map<TerritoryMapElement>>, //also useable for militarymap
    distance: u32,
    score: u32,
    target_owner: Optional<PlayerId>,
}

#[binrw]
#[derive(Debug)]
struct AttackExecution {
    version: VersionI!(1, "Ai AttackExecution"),
    pos: OptionalPatternCursor,
    expansion_target: ExpansionTarget,
}

#[binrw]
#[derive(Debug)]
struct MilitarySettings {
    version: VersionI!("AI MilitarySettings"),
    tick: CapedU32<50>,
}

#[binrw]
#[derive(Debug)]
struct MilitaryBuildingCreation {
    version: VersionI!("Ai MilitaryBuildingCreation"),
    order: ConstructionOrder,
    target: ExpansionTarget,
    tick: CapedU32<3>,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct Expansion {
    version: VersionI!(1, "Ai ExpansionSystem"),
    target: ExpansionTarget,
    target2: ExpansionTarget,
    expansion_request_type: ExpansionRequestType,
    expedition: VersionI!("AI Expedition"),
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
pub struct Cells {
    version: VersionI!(3, "Ai CellSystem"),
    cell_depot_creation: VersionedI!("Ai CellDepotCreation", ConstructionOrder),
    cell_expansion: VersionedI!("Ai CellExpansion", ExpansionTarget),
    pub cells: Array<Cell>,
    cell_validator: Option<CellValidator>,
    cell_clearing: Option<CellClearing>,
    harbor_creation: Option<HarborCreation>,
}

impl Positioned<Cell> for Cells {
    fn at(&self, x: usize, y: usize) -> &Cell {
        self.cells
            .array
            .iter()
            .find(|c| x as u32 & !0xf == c.pos.x && y as u32 & !0xf == c.pos.y)
            .unwrap() //TODO
    }
}

#[binrw]
#[derive(Debug)]
struct CellValidator {
    version: VersionI!("AI CellValidator"),
    start: MapIdxPos<Cell, Cells>,
    end: MapIdxPos<Cell, Cells>,
}

#[binrw]
#[derive(Debug)]
struct CellClearing {
    version: VersionI!("AI CellClearing"),
    start: MapIdxPos<Cell, Cells>,
    end: MapIdxPos<Cell, Cells>,
    order: ConstructionOrder,
}

#[binrw]
#[derive(Debug)]
struct HarborCreation {
    version: VersionI!(1, "Ai HarborCreation"),
    order: ConstructionOrder,
    active: Bool,
}

#[binrw]
#[derive(Debug)]
pub struct Cell {
    version: VersionI!(5, "Ai Cell"),
    id: Uuid,
    cell_idx_pos: MapIdxPos<Cell, Cells>,
    pos: PatternCursor,
    depots: Versioned!("Ai ReferncesBuilding", Array<Ref<AiBuilding>>),
    civil_buildings: Versioned!("Ai ReferncesBuilding", Array<Ref<AiBuilding>>),
    military_buildings: Versioned!("Ai ReferncesBuilding", Array<Ref<AiBuilding>>),
    productions: Versioned!("Ai ReferncesProduction", Array<Ref<Production>>),
    needs: Versioned!("Ai ReferncesNeed", Array<Ref<Need>>),
    expansion_maybe: CellConstruction,
    depot_constructon: CellConstruction,
    cell_full: VersionedI!("AI CellFull", Array<CellFullElement>),
    destruction_site: CellConstruction,
    destruction_size: u32,
    aibuilding_ref: OptRef<AiBuilding>,
    harbors: Versioned!("Ai ReferncesBuilding", Array<Ref<AiBuilding>>),
    continent_id: u32,
    idk: CellConstruction,
    aibuilding_ref2: OptRef<AiBuilding>,
}

impl Ided for Cell {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct CellFullElement {
    good: Good,
    version: Version!("AI CellFullElement"),
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
struct TerritoryMapElement {
    version: Version!(2, "AI TerritoryMap Element"),
    cell_ref: OptRef<Cell>,
    idk: i32,
    attack_fc: i32,
    catapults_constructioned: i32,
    military_ic: i32,
}

#[binrw]
#[derive(Debug)]
pub struct Production {
    version: VersionI!("Ai Production"),
    id: Uuid,
    product: Good,
    cell_ref: Ref<Cell>,
    need_ref: Ref<Need>,
    aibuilding_ref: OptRef<AiBuilding>,
}

impl Ided for Production {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
pub struct Need {
    version: VersionI!("Ai Need"),
    id: Uuid,
    need_type: Good,
    cell_ref: OptRef<Cell>,
    production_ref: OptRef<Production>,
    aibuilding_ref: OptRef<AiBuilding>,
    pos_resource_element_adds: MapIdxPos<Vec<ResourceMapElement>, ResourceMap>,
    is_in_resource_system: Bool,
    child: OptRef<Need>,
    parent: OptRef<Need>,
}

impl Ided for Need {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
pub struct AiBuilding {
    version: VersionI!(1, "Ai Building"), //TODO: why?
    id: Uuid,                             //TODO same as Building Uuid
    building_ref: Ref<Building>,
    cell_ref: Ref<Cell>,
    need_refs: Versioned!("Ai ReferncesNeed", Array<Ref<Need>>),
    production_refs: Versioned!("Ai ReferncesProduction", Array<Ref<Production>>),
}

impl Ided for AiBuilding {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct SmallResourceMapAddsElement {
    version: Version!("Ai LowResResourceMapAddsElement"),
    idk: i32, //TODO
}

#[binrw]
#[derive(Debug)]
struct ResourceMapAddsElement {
    version: Version!("Ai ResourceMapAddsElement"),
    need: OptRef<Need>,
    idk: Bool,
}

#[binrw]
#[derive(Debug)]
struct NeedsConstruction {
    version: VersionI!("AI NeedsConstruction"),
    order: ConstructionOrder,
    need: OptRef<Need>,
}

#[binrw]
#[derive(Debug)]
struct ExpansionTarget {
    version: Version!("Ai ExpansionTarget"),
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
    version: VersionI!(3, "Ai ConstructionSystem"),
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
    version: VersionI!(2, "AI StreetConstruction"),
    curr_flag_idx: i32,
    target_flag: u64, //Flag ref
    idk: u32,
    tried_flag_poses: Array<PatternCursor>,
    idk2: Array<(u32, Ref<Flag>)>, //TODO idk
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
    version: VersionI!(1, "AI ForesterConstruction"),
    order: ConstructionOrder,
    curr_cell_idx: u32,
}

#[binrw]
#[derive(Debug)]
struct StreetOptimizer {
    version: VersionI!(1, "AI StreetOptimizer"),
    idx: u32,
}

#[binrw]
#[derive(Debug)]
struct StreetRouteOptimizer {
    version: VersionI!(1, "AI StreetRouteOptimizer"),
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
    version: VersionI!("AI ShipConstruction"),
    order: ConstructionOrder,
    aibuilding_ref: OptRef<AiBuilding>, //TODO
}

#[binrw]
#[derive(Debug)]
struct HunterConstruction {
    version: VersionI!("AI ConstructionHunter"),
    order: ConstructionOrder,
    cell_idk: u32, //TODO
}

#[binrw]
#[derive(Debug)]
struct BuildingConstruction {
    version: Version!("AI BuildingConstruction"),
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
    version: Version!("Ai ConstructionOrder"),
    idk: u32,
    curr_tick_pos: MapIdxPos<Cell, Cells>,
    pos: OptionalPatternCursor,
    idk2: u32,
    building_type: Optional<BuildingType>,
    priority: u32,
    ticked_seconds: Time,
}

#[binrw]
#[derive(Debug)]
struct SearchConstructionPlace {
    version: Version!("AI SearchConstructionPlace"),
    building_type: Optional<BuildingType>,
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
    version: Version!("AI CellPositionItertator"),
    curr_tick_pos: (u32, u32),
    pos: PatternCursor,
    counter1: u32,
    counter2: u32,
}

#[binrw]
#[derive(Debug)]
pub struct MilitaryMap {
    version: VersionI!("Ai Military Map"),
    map: Map<MilitaryMapElement>,
    curr_tick_pos: MapIdxPos<MilitaryMapElement, MilitaryMap>,
}

impl Positioned<MilitaryMapElement> for MilitaryMap {
    fn at(&self, x: usize, y: usize) -> &MilitaryMapElement {
        &self.map.grid[(x, y)]
    }
}

#[binrw]
#[derive(Debug)]
struct ResourceMap {
    version: VersionI!(1, "Ai ResourceMap"), //TODO why not 0?
    map: Map<Versioned!("AI ResourceMapElements", Array<ResourceMapElement>)>,
    tick_pos: MapIdxPos<Vec<ResourceMapElement>, ResourceMap>,
    tick_pos2: MapIdxPos<Vec<ResourceMapElement>, ResourceMap>,
    len: u32,
}

impl Positioned<Vec<ResourceMapElement>> for ResourceMap {
    fn at(&self, x: usize, y: usize) -> &Vec<ResourceMapElement> {
        &self.map.grid[(x, y)].data.array
    }
}

#[binrw]
#[derive(Debug)]
pub struct ResourceMapElement {
    version: Version!("AI ResourceMapElement"),
    good: Good,
    pattern_type: u32, //idk
    deposit_number: u32,
    continent_id: u32,
}

#[binrw]
#[derive(Debug)]
pub struct SmallResourceMap {
    version: VersionI!("Ai LowResResourceMap"),
    map: Map<Versioned!("AI ResourceMapElements", Array<ResourceMapElement>)>,
    curr_tick_pos: MapIdxPos<Vec<ResourceMapElement>, SmallResourceMap>,
}

impl Positioned<Vec<ResourceMapElement>> for SmallResourceMap {
    fn at(&self, x: usize, y: usize) -> &Vec<ResourceMapElement> {
        &self.map.grid[(x, y)].data.array
    }
}

#[binrw]
#[derive(Debug)]
struct MilitaryMapElement {
    version: Version!("AI Military Map Element"),
    treat: (u32, u32), // second one is with unfinised buildings
    ai_score: u32,
}
