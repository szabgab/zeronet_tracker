use clap::crate_version;
use lazy_static::lazy_static;
use prometheus::{labels, opts, register_int_counter, register_int_gauge, IntCounter, IntGauge};

use crate::peer_db::get_peer_db_type;

lazy_static! {
  pub static ref PEER_GAUGE: IntGauge =
    register_int_gauge!("zn_tracker_peers", "Peers in database").unwrap();
  pub static ref HASH_GAUGE: IntGauge =
    register_int_gauge!("zn_tracker_hashes", "Hashes in database").unwrap();
  pub static ref REQUEST_COUNTER: IntCounter =
    register_int_counter!("zn_tracker_requests_total", "Requests received").unwrap();
  pub static ref OPENED_CONNECTIONS: IntCounter = register_int_counter!(
    "zn_tracker_opened_connections_total",
    "Connections opened since start"
  )
  .unwrap();
  pub static ref CLOSED_CONNECTIONS: IntCounter = register_int_counter!(
    "zn_tracker_closed_connections_total",
    "Connections closed since start"
  )
  .unwrap();
  pub static ref VERSION_GAUGE: IntGauge = register_int_gauge!(opts!(
    "zn_tracker_build_info",
    "Build information",
    labels! {
      "version" => crate_version!(),
      "revision" => env!("CARGO_PKG_REVISION"),
      "peerdb_type" => get_peer_db_type(),
      "rustc" => env!("CARGO_PKG_RUSTC"),
    }
  ))
  .unwrap();
}
