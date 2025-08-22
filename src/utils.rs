use crate::{
    ai::{AiBuilding, Cell, Need, Production},
    buildings::{Building, Order},
    doodads::Doodad,
    helper_structs::*,
    logic::{MapFile, Trigger},
    net::{Flag, Street},
    resources::{Animal, Deposit},
    settlers::{Bulldozer, Carrier, Constructor, Soldier, Specialist, Worker},
    transport::Package,
};

pub fn get_all_uuids(mapfile: &MapFile) -> Vec<(Uuid, ObjRef)> {
    use ObjRef::*;
    fn addall<'a, I: Ided>(res: &mut Vec<(Uuid, ObjRef<'a>)>, vec: &'a [I], map: fn(&I) -> ObjRef) {
        res.extend(vec.iter().map(|i| (i.id(), map(i))));
    }
    let mut res = Vec::new();
    let r = &mut res;
    addall(r, &mapfile.logic.trigger_sys.data.array, |i| Trigger(i));
    addall(r, &mapfile.resources.deposits.array, |i| Deposit(i));
    addall(r, &mapfile.resources.animals.array, |i| Animal(i));
    addall(r, &mapfile.doodads.map1.array, |i| Doodad(i));
    addall(r, &mapfile.doodads.map2.array, |i| Doodad(i));
    addall(r, &mapfile.doodads.map3.array, |i| Doodad(i));
    let Some(save) = mapfile.save_file_info.as_ref() else {
        return res;
    };

    addall(r, &save.villages.buildings.array, |i| Building(i));
    addall(r, &save.villages.orders.orders.array, |i| Order(i));
    addall(r, &save.settlers.workers.array, |i| Worker(i));
    addall(r, &save.settlers.constructor.array, |i| Constructor(i));
    addall(r, &save.settlers.carrier.array, |i| Carrier(i));
    addall(r, &save.settlers.bulldoser.array, |i| Bulldozer(i));
    addall(r, &save.settlers.soldier.array, |i| Soldier(i));
    addall(r, &save.settlers.specialist.array, |i| Specialist(i));
    addall(r, &save.transport.packages.data.array, |i| Package(i));
    addall(r, &save.netsys.flags.array, |i| Flag(i));
    addall(r, &save.netsys.streets.array, |i| Street(i));
    for ai_player in save
        .ai
        .ai_players
        .iter()
        .filter_map(|p| p.initialized_ai_player.as_ref())
    {
        addall(r, &ai_player.buildings.array, |i| AiBuilding(i));
        addall(r, &ai_player.productions.array, |i| Production(i));
        addall(r, &ai_player.cells.cells.array, |i| Cell(i));
        addall(r, &ai_player.general_needs.array, |i| Need(i));
        addall(r, &ai_player.food_needs.array, |i| Need(i));
    }
    res
}

#[derive(Debug)]
pub enum ObjRef<'a> {
    Trigger(&'a Trigger),
    Deposit(&'a Deposit),
    Animal(&'a Animal),
    Doodad(&'a Doodad),
    Building(&'a Building),
    Order(&'a Order),
    Worker(&'a Worker),
    Constructor(&'a Constructor),
    Carrier(&'a Carrier),
    Bulldozer(&'a Bulldozer),
    Soldier(&'a Soldier),
    Specialist(&'a Specialist),
    Package(&'a Package),
    Flag(&'a Flag),
    Street(&'a Street),
    AiBuilding(&'a AiBuilding),
    Production(&'a Production),
    Cell(&'a Cell),
    Need(&'a Need),
}
