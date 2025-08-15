use crate::Version;
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
    version: Version!(0, "VillageSystem"),
    init: Bool,
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
    workers: Workers,
    deposit_ref: Ref<Deposit>,
    #[brw(if(version.version > 3))]
    animal_ref: Option<Ref<Animal>>,
    construction: Construction,
    production: Production,
    blocking: Blocking,
    idk3: u32,
    tribe: u32,
    flag_ref: Ref<Flag>,
    settler_spawn: SettlerSpawn,
    mining_pos: PatternCursor,
    territory_updater: TerritoryUpdater,
    bulldozing: Bulldozing,
    order: OrderContainer,
    idk5: Bool,
    #[brw(if(version.version < 6 || matches!(building_type, Castle | Barracks | GuardHouse | Tower | WatchTower | Fortress)))]
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
    needed_goods: NeededGoods,
    returning_goods: ReturningGoods,
}

#[binrw]
#[derive(Debug)]
struct NeededGoods {
    version: Version!(0, "Need Good System"),
    needed_goods: Array<Package>,
}

#[binrw]
#[derive(Debug)]
struct Package {
    idk: u32,
    package_ref: Ref<crate::transport::Package>,
}

#[binrw]
#[derive(Debug)]
struct ReturningGoods {
    version: Version!(0, "Returning Good System"),
    returning_goods: Array<Ref<crate::transport::Package>>,
}

#[binrw]
#[derive(Debug)]
struct Workers {
    version: Version!(0, "VillageWorkers"),
    workers: Array<Ref<Worker>>,
}

#[binrw]
#[derive(Debug)]
struct Construction {
    version: Version!(2, "VillageConstruction"),
    progress: f32,
    progress_start_idk: f32,
    settler_ref: Ref<Settler>,
    #[br(if(version.version > 0))]
    stock: Option<Stock>,
    #[br(if(version.version > 1))]
    building_type: Option<BuildingType>,
}

#[binrw]
#[derive(Debug)]
struct Production {
    version: Version!(1, "VillageProduction"),
    init: Bool,
    idk: u32,
    #[brw(if(version.version > 0))]
    make_ships: Option<Bool>,
}

#[binrw]
#[derive(Debug)]
struct SettlerSpawn {
    version: Version!(0, "Village SettlerSpawn"),
    idk: u32,
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
    #[br(if(version.version > 0))]
    building_type: Option<BuildingType>,
}

#[binrw]
#[derive(Debug)]
pub struct OrderContainer {
    version: Version!(0, "Order Container"),
    orders: Array<Ref<Order>>,
}

#[binrw]
#[derive(Debug)]
struct VillageMilitary {
    version: Version!(2, "Village Military"),
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
    version: Version!(0, "VillageSoldiers"),
    settlers: SettlersContainer,
}

#[binrw]
#[derive(Debug)]
struct Attackers {
    version: Version!(0, "Village Attackers"),
    settlers: SettlersContainer,
}

#[binrw]
#[derive(Debug)]
struct Interceptors {
    version: Version!(0, "Village Interceptors"),
    settlers: SettlersContainer,
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
    idk: u32,
}

#[binrw]
#[derive(Debug)]
struct GoodFlags {
    version: Version!(0, "VillageGoodFlags"),
    flags: Array<(Good, u32, u32)>, // lock, evict
}

#[binrw]
#[derive(Debug)]
struct Catapult {
    version: Version!(0, "Village Catapult"),
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
    version: Version!(6, "Village Harbor"),
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
    ship_ref: Option<Ref<Ship>>,
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
    idk: u32,
    building_ref: Ref<Building>,
}

#[binrw]
#[derive(Debug)]
struct NeedsTransfer {
    version: Version!(0, "Village HarboarNeedsTransfer"),
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
    version: Version!(1, "VillageRemains"),
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
    version: Version!(0, "Blocking"),
    init: Bool,
    pos: PatternCursor,
    size: u32,
}

#[binrw]
#[derive(Debug)]
struct Orders {
    version: Version!(2, "Order System"),
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
    version: Version!(3, "Village Order"),
    id: Uuid,
    #[brw(if(version.version < 3))]
    unused: u32,
    ordered: Good,
    #[brw(if(version.version > 0))]
    used: Bool,
    building_ref: Ref<Building>,
    flag_ref: Ref<Flag>,
    street_ref: Ref<Street>,
    #[brw(if(version.version > 1))]
    building_ref2: Ref<Building>,
}

impl Ided for Order {
    fn id(&self) -> Uuid {
        self.id
    }
}
