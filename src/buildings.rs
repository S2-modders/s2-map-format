use crate::helper_structs::BuildingType::*;
use crate::helper_structs::*;
use crate::player::Stock;
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Villages {
    #[brw(args("VillageSystem"))]
    version: Version<0>,
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
    #[brw(args("VillageBuilding"))]
    version: Version<7>,
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
    #[brw(args("ticker"))]
    version: Version<0>,
    curr_tick: u32,
    target_tick: u32,
}

#[binrw]
#[derive(Debug)]
struct Depot {
    #[brw(args("VillageDepot"))]
    version: Version<0>,
    stock1: Stock,
    stock2: Stock,
    needed_goods: NeededGoods,
    returning_goods: ReturningGoods,
}

#[binrw]
#[derive(Debug)]
struct NeededGoods {
    #[brw(args("Need Good System"))]
    version: Version<0>,
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
    #[brw(args("Returning Good System"))]
    version: Version<0>,
    returning_goods: Array<Uuid>, //package refs
}

#[binrw]
#[derive(Debug)]
struct Workers {
    #[brw(args("VillageWorkers"))]
    version: Version<0>,
    workers: Array<Uuid>, //worker refs
}

#[binrw]
#[derive(Debug)]
struct Construction {
    #[brw(args("VillageConstruction"))]
    version: Version<2>,
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
    #[brw(args("VillageProduction"))]
    version: Version<1>,
    init: Bool,
    idk: u32,
    #[brw(if(version.version > 0))]
    make_ships: Option<Bool>,
}

#[binrw]
#[derive(Debug)]
struct SettlerSpawn {
    #[brw(args("Village SettlerSpawn"))]
    version: Version<0>,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct TerritoryUpdater {
    #[brw(args("Village Territory Updater"))]
    version: Version<0>,
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct Bulldozing {
    #[brw(args("VillageBulldozing"))]
    version: Version<1>,
    progress: f32,
    settler_ref: Uuid,
    idk: u32,
    #[br(if(version.version > 0))]
    building_type: Option<BuildingType>,
}

#[binrw]
#[derive(Debug)]
struct OrderContainer {
    #[brw(args("Order Container"))]
    version: Version<0>,
    orders: Array<Uuid>, // Order refs
}

#[binrw]
#[derive(Debug)]
struct VillageMilitary {
    #[brw(args("Village Military"))]
    version: Version<2>,
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
    #[brw(args("VillageSoldiers"))]
    version: Version<0>,
    settlers: SettlersContainer,
}

#[binrw]
#[derive(Debug)]
struct Attackers {
    #[brw(args("Village Attackers"))]
    version: Version<0>,
    settlers: SettlersContainer,
}

#[binrw]
#[derive(Debug)]
struct Interceptors {
    #[brw(args("Village Interceptors"))]
    version: Version<0>,
    settlers: SettlersContainer,
}

#[binrw]
#[derive(Debug)]
struct SettlersContainer {
    #[brw(args("Settlers Container"))]
    version: Version<0>,
    settlers: Array<Uuid>, // Settler refs
}

#[binrw]
#[derive(Debug)]
struct CarrierRefresh {
    #[brw(args("Village CarrierRefresh"))]
    version: Version<0>,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct GoodFlags {
    #[brw(args("VillageGoodFlags"))]
    version: Version<0>,
    flags: Array<(Good, u32, u32)>, // lock, evict
}

#[binrw]
#[derive(Debug)]
struct Catapult {
    #[brw(args("Village Catapult"))]
    version: Version<0>,
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
    #[brw(args("Village Harbor"))]
    version: Version<6>,
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
    #[brw(args("Village Landing Position"))]
    version: Version<0>,
    ship_ref0: Uuid,
    ship_ref1: Uuid,
    pos: PatternCursor,
}

#[binrw]
#[derive(Debug)]
struct Expedition {
    #[brw(args("Village Expedition"))]
    version: Version<0>,
    expedition_state: u32,
    stock: Stock,
    ship_ref: Uuid,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct HarborReceiver {
    #[brw(args("Village HarborReceiver"))]
    version: Version<0>,
    idk: u32,
    building_ref: Uuid,
}

#[binrw]
#[derive(Debug)]
struct NeedsTransfer {
    #[brw(args("Village HarboarNeedsTransfer"))]
    version: Version<0>,
}

#[binrw]
#[derive(Debug)]
struct Upgrade {
    #[brw(args("VillageUpgrade"))]
    version: Version<0>,
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
    #[brw(args("VillageRemains"))]
    version: Version<1>,
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
    #[brw(args("Blocking"))]
    version: Version<0>,
    init: Bool,
    pos: PatternCursor,
    size: u32,
}

#[binrw]
#[derive(Debug)]
struct Orders {
    #[brw(args("Order System"))]
    version: Version<2>,
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
    #[brw(args("Village Order"))]
    version: Version<3>,
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
