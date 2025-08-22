mod utils;

use std::{io::Cursor, iter::repeat};

use binrw::{BinRead, BinWrite};
use decryptor_s2::*;
use simple_eyre::eyre::{OptionExt, Result};
use strum::IntoEnumIterator;
mod helper_structs;
mod logic;
mod map;
use crate::{
    doodads::DoodadType,
    helper_structs::PlayerId,
    logic::{MapFile, PlayerType, Trigger, TriggerType},
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
            let mut map = MapFile::read_le(reader)?;
            let players: Vec<PlayerId> = map
                .mapinfo
                .player_types
                .iter()
                .zip(PlayerId::iter())
                .filter_map(|(t, p)| matches!(t, PlayerType::Player).then_some(p))
                .collect();
            let uuid_generator = &mut map.logic.uuid_generator;
            map.logic.trigger_sys.data.array.append(
                &mut map
                    .doodads
                    .blocking
                    .array
                    .iter()
                    .filter(|d| matches!(d.doodad_type, DoodadType::Gate0))
                    .map(|d| d.pos)
                    .flat_map(|pos| players.iter().zip(repeat(pos)))
                    .zip(uuid_generator)
                    .map(|((p, pos), id)| Trigger::new(id, TriggerType::Win, pos.into(), 0, "", *p))
                    .collect(),
            );
            let mut data = Cursor::new(Vec::new());
            map.write_le(&mut data)?;
            decryptor_s2::write_encrypted(s.to_string() + ".mod", Game::Dng, data.into_inner())?;
            Ok(())
        })?;
    Ok(())
}
