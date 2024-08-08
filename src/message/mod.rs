//! GDL90 IN/OUT messages and types.
//!
//! | ID | Name                         | I/O   |
//! |----|------------------------------|-------|
//! | 0  | Heartbeat                    | OUT   |
//! | 2  | Initialization               | IN    |
//! | 7  | Uplink Data                  | OUT   |
//! | 9  | Height Above Terrain         | IN    |
//! | 10 | Ownship Report               | OUT   |
//! | 11 | Ownship Geometric Altitude   | OUT   |
//! | 20 | Traffic Report               | OUT   |
//! | 30 | Basic Report                 | OUT   |
//! | 31 | Long Report                  | OUT   |

use binrw::BinRead;

pub mod heartbeat;
pub mod height_above_terrain;
pub mod initialization;
pub mod ownship_geometric_altitude;
pub mod ownship_report;
pub mod traffic_report;
pub mod types;
pub mod uplink_data;

/// GDL90 IN/OUT message types.
/// TODO: binread for IN messages
/// TODO: binread for OUT messages
#[derive(BinRead, Debug)]
pub enum Gdl90MessageType {
    #[br(magic = b"\x00")]
    Heartbeat(heartbeat::HeartbeatMessage),

    #[br(magic = b"\x02")]
    Initialization(initialization::InitializationMessage),

    #[br(magic = b"\x07")]
    UplinkData(uplink_data::UplinkDataMessage),

    #[br(magic = b"\x0A")]
    OwnshipReport(ownship_report::OwnshipReportMessage),

    #[br(magic = b"\x0B")]
    OwnshipGeoometricAltitude(ownship_geometric_altitude::OwnshipGeometricAltitude),

    #[br(magic = b"\x14")]
    TrafficReport(traffic_report::TrafficReportMessage),

    Unknown,
}
