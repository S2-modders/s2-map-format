use crate::Version;
use crate::VersionI;
use crate::Versioned;
use crate::helper_structs::BuildingType::*;
use crate::helper_structs::*;
use crate::navy::Ship;
use crate::net::{Flag, Street};
use crate::player::Stock;
use crate::resources::{Animal, Deposit};
use crate::settlers::{Settler, Worker};
use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct Villages {
    version: VersionI!(0, "VillageSystem"),
    buildings: Array<Building>,
    remains: Array<Remains>,
    orders: Orders,
}

#[binrw]
#[derive(Debug)]
pub struct Building {
    building_type: BuildingType,
    owner: PlayerId,
    version: Version!(6, "VillageBuilding"),
    id: Uuid,
    pos: PatternCursor,
    ticker: Ticker,
    depot: Depot,
    workers: Versioned!(0, "VillageWorkers", Array<Ref<Worker>>),
    deposit_ref: Ref<Deposit>,
    animal_ref: Ref<Animal>,
    construction: Construction,
    production: Production,
    blocking: Blocking,
    idk3: u32,
    tribe: u32,
    flag_ref: Ref<Flag>,
    settler_spawn: SettlerSpawn,
    mining_pos: OptionalPatternCursor,
    territory_updater: TerritoryUpdater,
    bulldozing: Bulldozing,
    order: Versioned!(0, "Order Container", Array<Ref<Order>>),
    idk5: Bool,
    #[brw(if(matches!(building_type, Castle | Barracks | GuardHouse | Tower | WatchTower | Fortress)))]
    military: Option<VillageMilitary>,
    carrier_refresh: CarrierRefresh,
    good_flags: Versioned!(0, "VillageGoodFlags", Array<GoodFlags>),
    idk6: u32,
    tick: u32,
    #[brw(if(matches!(building_type, Catapult)))]
    catapult: Option<Catapult>,
    #[brw(if(matches!(building_type, Harbor)))]
    harbor: Option<Harbor>,
    upgrade: Upgrade,
}

impl Ided for Building {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[binrw]
#[derive(Debug)]
struct Ticker {
    version: Version!(0, "ticker"),
    curr_tick: u32,
    target_tick: u32,
}

#[binrw]
#[derive(Debug)]
struct Depot {
    version: Version!(0, "VillageDepot"),
    stock1: Stock,
    stock2: Stock,
    needed_goods: Versioned!(0, "Need Good System", Array<Package>),
    returning_goods: Versioned!(
        0,
        "Returning Good System",
        Array<Ref<crate::transport::Package>>
    ),
}

#[binrw]
#[derive(Debug)]
struct Package {
    idk: u32, //TODO
    package_ref: Ref<crate::transport::Package>,
}

#[binrw]
#[derive(Debug)]
struct Construction {
    version: Version!(2, "VillageConstruction"),
    progress: f32,
    progress_start_idk: f32,
    settler_ref: Ref<Settler>,
    stock: Stock,
    building_type: BuildingType,
}

#[binrw]
#[derive(Debug)]
struct Production {
    version: Version!(1, "VillageProduction"),
    init: Bool,
    idk: Option<u32>,
    make_ships: Option<Bool>,
}

#[binrw]
#[derive(Debug)]
struct SettlerSpawn {
    version: Version!(0, "Village SettlerSpawn"),
    idk: u32, //TODO
}

#[binrw]
#[derive(Debug)]
struct TerritoryUpdater {
    version: Version!(0, "Village Territory Updater"),
    init: Bool,
}

#[binrw]
#[derive(Debug)]
struct Bulldozing {
    version: Version!(1, "VillageBulldozing"),
    progress: f32,
    settler_ref: Ref<Settler>,
    idk: u32,
    building_type: BuildingType,
}

#[binrw]
#[derive(Debug)]
struct VillageMilitary {
    version: Version!(2, "Village Military"),
    soldiers: Versioned!(0, "VillageSoldiers", SettlersContainer),
    attackers: Versioned!(0, "Village Attackers", SettlersContainer),
    enemy_distance: u32,
    idk: f32,
    enable_coin_supply: Bool,
    idk2: u32,
    soldier_rserve: [u32; 5],
    interceptors: Versioned!(0, "Village Interceptors", SettlersContainer),
    coin_supply2: Bool,
}

#[binrw]
#[derive(Debug)]
pub struct SettlersContainer {
    version: Version!(0, "Settlers Container"),
    settlers: Array<Ref<Settler>>,
}

#[binrw]
#[derive(Debug)]
struct CarrierRefresh {
    version: Version!(0, "Village CarrierRefresh"),
    idk: u32, //TODO
}

#[binrw]
#[derive(Debug)]
struct GoodFlags {
    good: Good,
    lock: Bool,
    evict: Bool,
}

#[binrw]
#[derive(Debug)]
struct Catapult {
    version: Version!(0, "Village Catapult"),
    target: OptionalPatternCursor,
    target_radomized: OptionalPatternCursor,
    time_next_direction_set: f32,
    time_stone_ordered: f32,
    direction: Direction,
    next_direction: u32,
}

#[binrw]
#[derive(Debug)]
struct Harbor {
    version: VersionI!(6, "Village Harbor"),
    landing_positions: Array<LandingPosition>,
    idk: u32,
    expedition: Expedition,
    orders: Versioned!(0, "Order Container", Array<Ref<Order>>),
    harbor_receivers: Array<HarborReceiver>,
    ship_ref: Ref<Ship>,
    settlers: SettlersContainer,
    idk2: u32,
    needs_transfer0: Version!(0, "Village HarboarNeedsTransfer"),
    needs_transfer1: Version!(0, "Village HarboarNeedsTransfer"),
    needs_transfer2: Version!(0, "Village HarboarNeedsTransfer"),
}

#[binrw]
#[derive(Debug)]
struct LandingPosition {
    version: Version!(0, "Village Landing Position"),
    ship_ref0: Ref<Ship>,
    ship_ref1: Ref<Ship>,
    pos: PatternCursor,
}

#[binrw]
#[derive(Debug)]
struct Expedition {
    version: Version!(0, "Village Expedition"),
    expedition_state: u32,
    stock: Stock,
    ship_ref: Ref<Ship>,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct HarborReceiver {
    version: Version!(0, "Village HarborReceiver"),
    idk: u32, //TODO
    building_ref: Ref<Building>,
}

#[binrw]
#[derive(Debug)]
struct Upgrade {
    version: Version!(0, "VillageUpgrade"),
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
    version: VersionI!(1, "VillageRemains"),
    pos: PatternCursor,
    someproperty: u32,
    blocking: Blocking,
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct Blocking {
    version: VersionI!(0, "Blocking"),
    pos: PatternCursor,
    size: u32, //TODO
}

#[binrw]
#[derive(Debug)]
struct Orders {
    version: VersionI!(2, "Order System"),
    orders: Array<Order>,
    idk: i32, //TODO
}

#[binrw]
#[derive(Debug)]
pub struct Order {
    version: Version!(3, "Village Order"),
    id: Uuid,
    ordered: Good,
    used: Bool,
    building_ref: Ref<Building>,
    flag_ref: Ref<Flag>,
    street_ref: Ref<Street>,
    building_ref2: Ref<Building>,
}

impl Ided for Order {
    fn id(&self) -> Uuid {
        self.id
    }
}
