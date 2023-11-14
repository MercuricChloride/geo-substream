pub mod helpers;
mod pb;
pub mod triples;

use pb::schema::{
    EntriesAdded, EntryAdded, GeoOutput, RoleGranted, RoleRevoked, RolesGranted, RolesRevoked,
};
use substreams::hex;
use substreams_ethereum::{pb::eth, use_contract, Event};

use helpers::*;

use_contract!(space, "abis/space.json");

use space::events::{
    EntryAdded as EntryAddedEvent, RoleAdminChanged as RoleAdminChangedEvent,
    RoleGranted as RoleGrantedEvent, RoleRevoked as RoleRevokedEvent,
};

const ROOT_SPACE_ADDRESS: [u8; 20] = hex!("170b749413328ac9a94762031a7a05b00c1d2e34");

const ADMIN_ROLE: [u8; 32] =
    hex!("a49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c21775");

const EDITOR_CONTROLLER_ROLE: [u8; 32] =
    hex!("bc2c04b16435c5f4eaa37fec9ad808fec563d665b1febf40775380f3f1b592b4");

const EDITOR_ROLE: [u8; 32] =
    hex!("21d1167972f621f75904fb065136bc8b53c7ba1c60ccd3a7758fbee465851e9c");

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

#[substreams::handlers::map]
fn map_roles_granted(block: eth::v2::Block) -> Result<RolesGranted, substreams::errors::Error> {
    let roles: Vec<RoleGranted> = block
        .logs()
        .filter_map(|log| {
            if let Some(role_granted) = RoleGrantedEvent::match_and_decode(log) {
                let tx_hash = format_hex(&log.receipt.transaction.hash);
                let log_index = log.index();
                let block_number = block.number;
                let id = format!("{block_number}-{tx_hash}-{log_index}");
                let address = format_hex(&log.address());
                Some((role_granted, id, address))
            } else {
                None
            }
        })
        .map(|(role_granted, id, address)| {
            let role = role_to_enum_value(role_granted.role);
            let sender = format_hex(&role_granted.sender);
            let account = format_hex(&role_granted.account);

            RoleGranted {
                id,
                role,
                sender,
                account,
                space: address,
            }
        })
        .collect();

    Ok(RolesGranted { roles })
}

#[substreams::handlers::map]
fn map_roles_revoked(block: eth::v2::Block) -> Result<RolesRevoked, substreams::errors::Error> {
    let roles: Vec<RoleRevoked> = block
        .logs()
        .filter_map(|log| {
            if let Some(role_revoked) = RoleRevokedEvent::match_and_decode(log) {
                let tx_hash = format_hex(&log.receipt.transaction.hash);
                let log_index = log.index();
                let block_number = block.number;
                let id = format!("{block_number}-{tx_hash}-{log_index}");
                let address = format_hex(&log.address());
                Some((role_revoked, id, address))
            } else {
                None
            }
        })
        .map(|(role_revoked, id, address)| {
            let role = role_to_enum_value(role_revoked.role);
            let sender = format_hex(&role_revoked.sender);
            let account = format_hex(&role_revoked.account);

            RoleRevoked {
                id,
                role,
                sender,
                account,
                space: address,
            }
        })
        .collect();

    Ok(RolesRevoked { roles })
}

#[substreams::handlers::map]
fn geo_out(
    entries: EntriesAdded,
    roles_granted: RolesGranted,
    roles_revoked: RolesRevoked,
) -> Result<GeoOutput, substreams::errors::Error> {
    let entries = entries.entries;
    let roles_granted = roles_granted.roles;
    let roles_revoked = roles_revoked.roles;

    Ok(GeoOutput {
        entries,
        roles_granted,
        roles_revoked,
    })
}
