mod utils;

use std::collections::HashMap;

use binrw::BinRead;
use decryptor_s2::*;
use simple_eyre::eyre::{OptionExt, Result};
mod helper_structs;
mod logic;
mod map;
use crate::{
    logic::MapFile,
    utils::{ObjRef, get_all_uuids},
};
mod ai;
mod buildings;
mod doodads;
mod military;
mod movement;
mod navy;
mod net;
mod player;
mod resources;
mod settlers;
mod transport;

fn main() -> Result<()> {
    simple_eyre::install()?;
    std::env::args().collect::<Vec<String>>()[1..]
        .iter()
        .try_for_each(|s| -> Result<()> {
            let decompressed = decryptor_s2::decrypt(s)?.ok_or_eyre("Not a map file: {s}")?;
            assert!(
                matches!(decompressed.game, Game::Dng),
                "Map is from wrong game"
            );
            let reader = &mut std::io::Cursor::new(decompressed.data);
            let map = MapFile::read_le(reader)?;
            let get_all_uuids = get_all_uuids(&map);
            let idmap: HashMap<u64, &ObjRef<'_>> =
                get_all_uuids.iter().map(|(a, b)| (a.id.get(), b)).collect();

            for pack in map
                .save_file_info
                .as_ref()
                .unwrap()
                .netsys
                .net_graph
                .iter()
                .flat_map(|a| a.iter())
                .flat_map(|a| a.array.iter())
                .flat_map(|a| a.1.array.iter())
                .map(|pack| pack.0.id)
            {
                dbg!(matches!(idmap.get(&pack.id.get()), Some(ObjRef::Flag(_))));
            }
            Ok(())
        })?;
    Ok(())
}
