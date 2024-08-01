//! GDL90 Datalink message data types. 560-1058-00 Rev A - ref 3.
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

use crate::types::heartbeat::HeartbeatStatusByte1;
use crate::types::heartbeat::HeartbeatStatusByte2;
use crate::types::initialization::ConfigurationByte1;
use crate::types::initialization::ConfigurationByte2;
use crate::types::ownship_geometric_altitude::VerticalMetrics;
use crate::types::report::Report;
use crate::types::uplink_data::UplinkPayload;

use binrw::binread;

const GDL90_GEO_ALTITUDE_FACTOR: i16 = 5;

/// GDL90 IN/OUT message types.
/// TODO: binread for IN messages
/// TODO: binread for OUT messages
#[binread]
#[br(little)]
#[derive(Debug)]
pub enum Gdl90DatalinkMessage {
    /// (OUT) - GDL90 Heartbeat message. 560-1058-00 Rev A - ref 3.1.
    ///
    /// | Byte # | Name             | Size  | Value                                             |
    /// |--------|------------------|-------|---------------------------------------------------|
    /// | 1      |Message ID        | 1     | 0                                                 |
    /// | 2      |Status Byte 1     | 1     | see [`HeartbeatStatusByte1`]                      |
    /// | 3      |Status Byte 2     | 1     | see [`HeartbeatStatusByte2`]                      |
    /// | 4-5    |Timestamp         | 2     | Seconds since 0000Z, bits 15-0 (LSB byte first)   |
    /// | 6-7    |Message Counts    | 2     | see [`MessageCounts`]                             |
    /// |        |Total length      | 7     |                                                   |
    ///
    #[br(little, magic = b"\x00")]
    Heartbeat {
        status_byte_1: HeartbeatStatusByte1,
        status_byte_2: HeartbeatStatusByte2,
        uat_timestamp: u16,
        message_counts: u16,
    },

    /// (IN TODO) - GDL90 Initialization message. 560-1058-00 Rev A - ref 3.2.
    ///
    /// | Byte # | Name                 | Size  | Value                         |
    /// |--------|----------------------|-------|-------------------------------|
    /// | 1      |Message ID            | 1     | 2                             |
    /// | 2      |Configuration Byte 1  | 1     | see [`ConfigurationByte1`]    |
    /// | 3      |Configuration Byte 2  | 1     | see [`ConfigurationByte2`]    |
    /// |        |Total length          | 3     |                               |
    ///
    #[br(magic = b"\x02")]
    Initialization {
        configuration_byte_1: ConfigurationByte1,
        configuration_byte_2: ConfigurationByte2,
    },

    /// (OUT) - GDL90 Uplink Data message. 560-1058-00 Rev A - ref 3.3.
    ///
    /// Uplink messages received from UAT Ground Broadcast Transceivers are reported to the Display.
    ///
    /// | Byte # | Name             | Size  | Value                                         |
    /// |--------|------------------|-------|-----------------------------------------------|
    /// | 1      |Message ID        | 1     | 7                                             |
    /// | 2-4    |Time of reception | 3     | 24-bit binary fraction Resolution = 80 nsec   |
    /// | 5-436  |Uplink payload    | 432   | see [`UplinkPayload`]                         |
    /// |        |Total length      | 436   |                                               |
    ///
    #[br(magic = b"\x07")]
    UplinkData {
        #[br(parse_with = binrw::helpers::read_u24)]
        time_of_reception: u32,
        payload: UplinkPayload,
    },

    /// (OUT) - GDL90 Height Above Terrain Message. 560-1058-00 Rev A - ref 3.7.
    ///
    /// The GDL 90 can use the Height Above Terrain information from other on-board equipment that
    /// supports terrain awareness, in order to provide reduced CSA sensitivity at low altitudes.
    ///
    /// | Byte # | Name                 | Size  | Value                                         |
    /// |--------|----------------------|-------|-----------------------------------------------|
    /// | 1      |Message ID            | 1     | 9                                             |
    /// | 2-3    |Height Above Terrain  | 2     | Height above terrain. Resolution: 1 foot      |
    /// |        |Total length          | 3     |                                               |
    ///
    #[br(magic = b"\x09")]
    HeightAboveTerrain {
        hat: u16, // TODO: custom type
    },

    /// (OUT) - GDL90 Ownship Report message. 560-1058-00 Rev A - ref 3.4.
    ///
    /// The Ownship message contains information on the GNSS position.
    ///
    /// | Byte # | Name         | Size  | Value                         |
    /// |--------|--------------|-------|-------------------------------|
    /// | 1      |Message ID    | 1     | 10                            |
    /// | 2-28   |Ownship Report| 27    | see [`OwnshipReportMessage`]  |
    /// |        |Total length  | 28    |                               |
    ///
    #[br(little, magic = b"\x0A")]
    OwnshipReport {
        report: Report,
    },

    /// (OUT) - GDL90 Traffic Report message. 560-1058-00 Rev A - ref 3.5.
    ///
    /// When the Traffic Alert interface is in use, a Traffic Report message
    /// is output from the GDL 90 in each second for each alerted or proximate target.
    ///
    /// | Byte # | Name         | Size  | Value                         |
    /// |--------|--------------|-------|-------------------------------|
    /// | 1      |Message ID    | 1     | 10                            |
    /// | 2-28   |Traffic Report| 27    | see [`TrafficReportMessage`]  |
    /// |        |Total length  | 28    |                               |
    ///
    #[br(magic = b"\x14")]
    TrafficReport {
        report: Report,
    },

    /// (OUT) - GDL90 Ownship Geometric Altitude message. 560-1058-00 Rev A - ref 3.8.
    ///
    /// An Ownship Geometric Altitude message will be transmitted
    /// with a period of one second when the GNSS fix is valid.
    ///
    /// | Byte # | Name                 | Size  | Value                                                             |
    /// |--------|----------------------|-------|-------------------------------------------------------------------|
    /// | 1      |Message ID            | 1     | 11                                                                |
    /// | 2-3    |Ownship Geo Altitude  | 2     | Signed altitude in 5 ft. resolution                               |
    /// | 4-5    |Vertical Metrics      | 2     | Vertical Warning indicator and Vertical Figure of Merit in meters |
    /// |        |Total length          | 5     |                                                                   |
    ///
    #[br(magic = b"\x0B")]
    OwnshipGeoometricAltitude {
        #[br(map = |x: i16| x * GDL90_GEO_ALTITUDE_FACTOR)]
        ownship_geo_altitude: i16,
        vertical_metrics: VerticalMetrics,
    },

    #[br(magic = b"\x1E")]
    BasicReport(), // TODO ?

    #[br(magic = b"\x1F")]
    LongReport(), // TODO ?

    Unknown,
}
