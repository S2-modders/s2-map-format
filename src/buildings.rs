use crate::helper_structs::BuildingType::*;
use crate::helper_structs::*;
use crate::player::Stock;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Villages {
    #[brw(args(0, "VillageSystem"))]
    version: Version,
    init: Bool,
    buildings: Array<Building>,
    remains: Array<Remains>,
    orders: Orders,
}

#[binrw]
#[derive(Debug)]
struct Building {
    building_type: BuildingType,
    owner: PlayerId,
    #[brw(args(7, "VillageBuilding"))]
    version: Version,
    id: Uuid,
    pos: PatternCursor,
    ticker: Ticker,
    depot: Depot,
    workers: Workers,
    deposit_ref: Uuid,
    #[brw(if(version.version > 3))]
    animal_ref: Option<Uuid>,
    construction: Construction,
    production: Production,
    blocking: Blocking,
    idk3: u32,
    tribe: u32,
    flag_ref: Uuid,
    settler_spawn: SettlerSpawn,
    mining_pos: PatternCursor,
    territory_updater: TerritoryUpdater,
    bulldozing: Bulldozing,
    order: OrderContainer,
    idk5: Bool,
    #[brw(if(version.version < 6 || matches!(building_type, Castle | Barracks | GuardHouse | Tower | WatchTower | Fortress)))]
    //military buildings
    military: Option<VillageMilitary>,
    carrier_refresh: CarrierRefresh,
    good_flags: GoodFlags,
    idk6: u32,
    tick: u32,
    #[brw(if(version.version < 6 || matches!(building_type, Catapult)))]
    catapult: Option<Catapult>,
    #[brw(if(version.version > 1 && matches!(building_type, Harbor)))]
    harbor: Option<Harbor>,
    #[brw(if(version.version > 4))]
    upgrade: Option<Upgrade>,
}

#[binrw]
#[derive(Debug)]
struct Ticker {
    #[brw(args(0, "ticker"))]
    version: Version,
    curr_tick: u32,
    target_tick: u32,
}

#[binrw]
#[derive(Debug)]
struct Depot {
    #[brw(args(0, "VillageDepot"))]
    version: Version,
    stock1: Stock,
    stock2: Stock,
    needed_goods: NeededGoods,
    returning_goods: ReturningGoods,
}

#[binrw]
#[derive(Debug)]
struct NeededGoods {
    #[brw(args(0, "Need Good System"))]
    version: Version,
    needed_goods: Array<Package>,
}

#[binrw]
#[derive(Debug)]
struct Package {
    idk: u32,
    package_ref: Uuid,
}

#[binrw]
#[derive(Debug)]
struct ReturningGoods {
    #[brw(args(0, "Returning Good System"))]
    version: Version,
    returning_goods: Array<Uuid>, //package refs
}

#[binrw]
#[derive(Debug)]
struct Workers {
    #[brw(args(0, "VillageWorkers"))]
    version: Version,
    workers: Array<Uuid>, //worker refs
}

#[binrw]
#[derive(Debug)]
struct Construction {
    #[brw(args(2, "VillageConstruction"))]
    version: Version,
    progress: f32,
    progress_start_idk: f32,
    settler_ref: Uuid,
    #[br(if(version.version > 0))]
    stock: Option<Stock>,
    #[br(if(version.version > 1))]
    building_type: Option<BuildingType>,
}

#[binrw]
#[derive(Debug)]
struct Production {
    #[brw(args(1, "VillageProduction"))]
    version: Version,
    init: Bool,
    idk: u32,
    #[brw(if(version.version > 0))]
    make_ships: Option<Bool>,
}

#[binrw]
#[derive(Debug)]
struct SettlerSpawn {
    #[brw(args(0, "Village SettlerSpawn"))]
    version: Version,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct TerritoryUpdater {
    #[brw(args(0, "Village Territory Updater"))]
    version: Version,
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct Bulldozing {
    #[brw(args(1, "VillageBulldozing"))]
    version: Version,
    progress: f32,
    settler_ref: Uuid,
    idk: u32,
    #[br(if(version.version > 0))]
    building_type: Option<BuildingType>,
}

#[binrw]
#[derive(Debug)]
struct OrderContainer {
    #[brw(args(0, "Order Container"))]
    version: Version,
    orders: Array<Uuid>, // Order refs
}

#[binrw]
#[derive(Debug)]
struct VillageMilitary {
    #[brw(args(2, "Village Military"))]
    version: Version,
    soldiers: Soldiers,
    attackers: Attackers,
    enemy_distance: u32,
    idk: f32,
    enable_coin_supply: Bool,
    idk2: u32,
    soldier_rserve: [u32; 5],
    #[brw(if(version.version > 0))]
    interceptors: Option<Interceptors>,
    #[brw(if(version.version > 1))]
    coin_supply2: Option<Bool>,
}

#[binrw]
#[derive(Debug)]
struct Soldiers {
    #[brw(args(0, "VillageSoldiers"))]
    version: Version,
    settlers: SettlersContainer,
}

#[binrw]
#[derive(Debug)]
struct Attackers {
    #[brw(args(0, "Village Attackers"))]
    version: Version,
    settlers: SettlersContainer,
}

#[binrw]
#[derive(Debug)]
struct Interceptors {
    #[brw(args(0, "Village Interceptors"))]
    version: Version,
    settlers: SettlersContainer,
}

#[binrw]
#[derive(Debug)]
struct SettlersContainer {
    #[brw(args(0, "Settlers Container"))]
    version: Version,
    settlers: Array<Uuid>, // Settler refs
}

#[binrw]
#[derive(Debug)]
struct CarrierRefresh {
    #[brw(args(0, "Village CarrierRefresh"))]
    version: Version,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct GoodFlags {
    #[brw(args(0, "VillageGoodFlags"))]
    version: Version,
    flags: Array<(Good, u32, u32)>, // lock, evict
}

#[binrw]
#[derive(Debug)]
struct Catapult {
    #[brw(args(0, "Village Catapult"))]
    version: Version,
    target: PatternCursor,
    target_radomized: PatternCursor,
    time_next_direction_set: f32,
    time_stone_ordered: f32,
    direction: u32, //TODO: enum
    next_direction: u32,
}

#[binrw]
#[derive(Debug)]
struct Harbor {
    #[brw(args(6, "Village Harbor"))]
    version: Version,
    #[brw(if(version.version < 6, Bool { bool: true }))]
    init: Bool,
    landing_positions: Array<LandingPosition>,
    #[brw(if(version.version > 0))]
    idk: u32,
    #[brw(if(version.version > 1))]
    expedition: Option<Expedition>,
    #[brw(if(version.version > 1))]
    orders: Option<OrderContainer>,
    #[brw(if(version.version > 2))]
    harbor_receivers: Array<HarborReceiver>,
    #[brw(if(version.version > 3))]
    ship_ref: Option<Uuid>,
    #[brw(if(version.version > 3))]
    settlers: Option<SettlersContainer>,
    #[brw(if(version.version > 4))]
    idk2: u32,
    needs_transfer0: NeedsTransfer,
    needs_transfer1: NeedsTransfer,
    needs_transfer2: NeedsTransfer,
}

#[binrw]
#[derive(Debug)]
struct LandingPosition {
    #[brw(args(0, "Village Landing Position"))]
    version: Version,
    ship_ref0: Uuid,
    ship_ref1: Uuid,
    pos: PatternCursor,
}

#[binrw]
#[derive(Debug)]
struct Expedition {
    #[brw(args(0, "Village Expedition"))]
    version: Version,
    expedition_state: u32,
    stock: Stock,
    ship_ref: Uuid,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct HarborReceiver {
    #[brw(args(0, "Village HarborReceiver"))]
    version: Version,
    idk: u32,
    building_ref: Uuid,
}

#[binrw]
#[derive(Debug)]
struct NeedsTransfer {
    #[brw(args(0, "Village HarboarNeedsTransfer"))]
    version: Version,
}

#[binrw]
#[derive(Debug)]
struct Upgrade {
    #[brw(args(0, "VillageUpgrade"))]
    version: Version,
    init: Bool,
    #[brw(if(init.bool))]
    idk: u32,
    #[brw(if(init.bool))]
    construction: Option<Construction>,
    #[brw(if(init.bool))]
    bulldozing: Option<Bulldozing>,
}

#[binrw]
#[derive(Debug)]
struct Remains {
    remains_type: RemainsType, //maybe in wrong order
    building_type: BuildingType,
    #[brw(args(1, "VillageRemains"))]
    version: Version,
    init: Bool,
    pos: PatternCursor,
    someproperty: u32,
    blocking: Blocking,
    #[brw(if(version.version > 0))]
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct Blocking {
    #[brw(args(0, "Blocking"))]
    version: Version,
    init: Bool,
    pos: PatternCursor,
    size: u32,
}

#[binrw]
#[derive(Debug)]
struct Orders {
    #[brw(args(2, "Order System"))]
    version: Version,
    init: Bool,
    orders: Array<Order>,
    #[brw(if(version.version == 0))]
    unused1: u32,
    #[brw(if(version.version == 1))]
    unused2: u64,
    #[brw(if(version.version > 1))]
    idk: i32,
}

#[binrw]
#[derive(Debug)]
struct Order {
    #[brw(args(3, "Village Order"))]
    version: Version,
    id: Uuid,
    #[brw(if(version.version < 3))]
    unused: u32,
    ordered: Good,
    #[brw(if(version.version > 0))]
    used: Bool,
    building_ref: Uuid,
    flag_ref: Uuid,
    street_ref: Uuid,
    #[brw(if(version.version > 1))]
    building_ref2: Option<Uuid>,
}
