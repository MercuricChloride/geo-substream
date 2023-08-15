#[path = "./abi/space.rs"]
mod space;

pub mod helpers;
mod pb;
pub mod triples;

use pb::schema::{EntriesAdded, EntryAdded};
use space::events::EntryAdded as EntryAddedEvent;
use substreams::pb::substreams::Clock;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::{pb::eth, Event};

use helpers::*;

pub const ADDRESS: &str = "0x170b749413328ac9a94762031a7A05b00c1D2e34";

#[substreams::handlers::map]
fn map_entries_added(block: eth::v2::Block) -> Result<EntriesAdded, substreams::errors::Error> {
    let entries = block
        .logs()
        .filter_map(|log| {
            if format_hex(log.address()) != ADDRESS.to_lowercase() {
                return None;
            }

            if let Some(entry) = EntryAddedEvent::match_and_decode(log) {
                let tx_hash = format_hex(&log.receipt.transaction.hash);
                let log_index = log.index();
                let id = format!("{tx_hash}-{log_index}");
                Some((entry, id))
            } else {
                None
            }
        })
        .map(|(entry, id)| EntryAdded {
            id,
            index: entry.index.to_string(),
            uri: entry.uri,
            author: format_hex(&entry.author),
            space: ADDRESS.to_string(),
        })
        .collect::<Vec<EntryAdded>>();

    Ok(EntriesAdded { entries })
}

#[substreams::handlers::map]
pub fn graph_out(entries: EntriesAdded) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for entry in entries.entries {
        tables
            .create_row("EntryAdded", entry.id)
            .set("index", entry.index)
            .set("uri", entry.uri)
            .set("author", entry.author)
            .set("space", entry.space);
    }

    Ok(tables.to_entity_changes())
}
