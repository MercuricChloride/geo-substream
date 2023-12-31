pub mod helpers;
mod pb;

use pb::schema::{
    EntriesAdded, EntryAdded, GeoOutput, RoleChange, RoleChanges, RoleRevoked, RolesGranted,
    RolesRevoked,
};
use substreams::store::*;
use substreams_ethereum::{pb::eth, use_contract, Event};

use helpers::*;

use_contract!(space, "abis/space.json");

use space::events::{
    EntryAdded as EntryAddedEvent, RoleGranted as RoleGrantedEvent, RoleRevoked as RoleRevokedEvent,
};

const ROOT_SPACE_ADDRESS: &'static str = "0x170b749413328ac9a94762031a7a05b00c1d2e34";

#[substreams::handlers::map]
fn map_entries_added(block: eth::v2::Block) -> Result<EntriesAdded, substreams::errors::Error> {
    let entries = block
        .logs()
        .filter_map(|log| {
            if let Some(entry) = EntryAddedEvent::match_and_decode(log) {
                let tx_hash = format_hex(&log.receipt.transaction.hash);
                let log_index = log.index();
                let block_number = block.number;
                let id = format!("{block_number}-{tx_hash}-{log_index}");
                let address = format_hex(&log.address());
                Some((entry, id, address))
            } else {
                None
            }
        })
        .map(|(entry, id, address)| EntryAdded {
            id,
            index: entry.index.to_string(),
            uri: entry.uri,
            author: format_hex(&entry.author),
            space: address,
        })
        .collect::<Vec<EntryAdded>>();

    Ok(EntriesAdded { entries })
}

#[substreams::handlers::store]
fn store_addresses(entries: EntriesAdded, output: StoreSetIfNotExistsString) {
    let addresses = entries
        .entries
        .iter()
        .map(|entry| &entry.space)
        .collect::<Vec<&String>>();

    for address in addresses.iter() {
        output.set_if_not_exists(0, &address, address);
    }
}

#[substreams::handlers::map]
fn map_roles(
    block: eth::v2::Block,
    s: StoreGetString,
) -> Result<RoleChanges, substreams::errors::Error> {
    let changes: Vec<RoleChange> = block
        .logs()
        .filter_map(|log| {
            let tx_hash = format_hex(&log.receipt.transaction.hash);
            let log_index = log.index();
            let block_number = block.number;
            let id = format!("{block_number}-{tx_hash}-{log_index}");
            let address = format_hex(&log.address());

            if let None = s.get_last(&address) {
                if address != ROOT_SPACE_ADDRESS {
                    return None;
                }
            }

            if let Some(role_granted) = RoleGrantedEvent::match_and_decode(log) {
                let change = ChangeKind::Granted(role_granted);
                return Some((change, id, address));
            }
            if let Some(role_revoked) = RoleRevokedEvent::match_and_decode(log) {
                let change = ChangeKind::Revoked(role_revoked);
                return Some((change, id, address));
            }
            return None;
        })
        .map(|(role_change, id, address)| role_change.as_change(id, address))
        .collect();

    Ok(RoleChanges { changes })
}

#[substreams::handlers::map]
fn geo_out(
    entries: EntriesAdded,
    role_changes: RoleChanges,
) -> Result<GeoOutput, substreams::errors::Error> {
    let entries = entries.entries;
    let role_changes = role_changes.changes;

    Ok(GeoOutput {
        entries,
        role_changes,
    })
}
