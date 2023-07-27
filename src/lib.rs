#[path = "./abi/space.rs"]
mod space;

mod helpers;
mod pb;

use pb::schema::{EntriesAdded, EntryAdded};
use space::events::EntryAdded as EntryAddedEvent;
use substreams::pb::substreams::Clock;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::{pb::eth, Event};

use helpers::*;

pub const ADDRESS: &str = "0x170b749413328ac9a94762031a7A05b00c1D2e34";
const START_BLOCK: u64 = 12129118;

#[substreams::handlers::map]
fn map_entries_added(block: eth::v2::Block) -> Result<EntriesAdded, substreams::errors::Error> {
    let entries = block
        .logs()
        .filter_map(|log| {
            let tx_hash = format_hex(&log.receipt.transaction.hash);
            let log_index = log.index();
            let id = format!("{tx_hash}-{log_index}");

            if let Some(entry) = EntryAddedEvent::match_and_decode(log) {
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
        })
        .collect::<Vec<EntryAdded>>();

    Ok(EntriesAdded { entries })
}

// #[substreams::handlers::map]
// fn map_transfers(block: eth::v2::Block) -> Result<Transfers, substreams::errors::Error> {
//     let transfers = block
//         .logs()
//         .filter_map(|log| {
//             if format_hex(log.address()) == ADDRESS.to_lowercase() {
//                 let tx_hash = format_hex(&log.receipt.transaction.hash);
//                 let log_index = log.index();
//                 let id = format!("{tx_hash}-{log_index}");

//                 if let Some(transfer) = TransferBatchEvent::match_and_decode(log) {
//                     Some(TransferType::Batch(transfer, id))
//                 } else if let Some(transfer) = TransferSingleEvent::match_and_decode(log) {
//                     Some(TransferType::Single(transfer, id))
//                 } else {
//                     None
//                 }
//             } else {
//                 None
//             }
//         })
//         .map(|transfer_type| match transfer_type {
//             TransferType::Single(transfer, id) => Transfer {
//                 id,
//                 operator: format_hex(&transfer.operator),
//                 from: format_hex(&transfer.from),
//                 to: format_hex(&transfer.to),
//                 token_ids: vec![transfer.id.to_string()],
//             },
//             TransferType::Batch(transfer, id) => Transfer {
//                 id,
//                 operator: format_hex(&transfer.operator),
//                 from: format_hex(&transfer.from),
//                 to: format_hex(&transfer.to),
//                 token_ids: transfer.ids.iter().map(|id| id.to_string()).collect(),
//             },
//         })
//         .collect::<Vec<Transfer>>();

//     Ok(Transfers { transfers })
// }

// #[substreams::handlers::map]
// fn map_approvals(block: eth::v2::Block) -> Result<Approvals, substreams::errors::Error> {
//     let approvals = block
//         .logs()
//         .filter_map(|log| {
//             if format_hex(log.address()) == ADDRESS.to_lowercase() {
//                 let tx_hash = format_hex(&log.receipt.transaction.hash);
//                 let log_index = log.index();
//                 let id = format!("{tx_hash}-{log_index}");

//                 if let Some(approval) = ApprovalForAllEvent::match_and_decode(log) {
//                     Some((approval, id))
//                 } else {
//                     None
//                 }
//             } else {
//                 None
//             }
//         })
//         .map(|(approval, id)| Approval {
//             id,
//             owner: format_hex(&approval.owner),
//             operator: format_hex(&approval.operator),
//             approved: approval.approved,
//         })
//         .collect::<Vec<Approval>>();

//     Ok(Approvals { approvals })
// }

// #[substreams::handlers::map]
// fn map_uris(block: eth::v2::Block) -> Result<Uris, substreams::errors::Error> {
//     let uris = block
//         .logs()
//         .filter_map(|log| {
//             if format_hex(log.address()) == ADDRESS.to_lowercase() {
//                 let tx_hash = format_hex(&log.receipt.transaction.hash);
//                 let log_index = log.index();
//                 let id = format!("{tx_hash}-{log_index}");

//                 if let Some(uri) = UriEvent::match_and_decode(log) {
//                     Some((uri, id))
//                 } else {
//                     None
//                 }
//             } else {
//                 None
//             }
//         })
//         .map(|(uri, id)| Uri {
//             id,
//             value: uri.value,
//             token_id: uri.id.to_string(),
//         })
//         .collect::<Vec<Uri>>();

//     Ok(Uris { uris })
// }

// #[substreams::handlers::map]
// pub fn graph_out(
//     clock: Clock,
//     transfers: Transfers,
//     approvals: Approvals,
//     uris: Uris,
// ) -> Result<EntityChanges, substreams::errors::Error> {
//     let mut tables = Tables::new();

//     if clock.number == START_BLOCK {
//         // Create the collection, we only need to do this once
//         tables.create_row("Collection", ADDRESS.to_string());
//     }

//     transfers_to_table_changes(&mut tables, &transfers);

//     approvals_to_table_changes(&mut tables, &approvals);

//     uris_to_table_changes(&mut tables, &uris);

//     Ok(tables.to_entity_changes())
// }
