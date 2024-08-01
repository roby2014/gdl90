//! GDL90 Heartbeat message.
//!
//! | Byte # | Name             | Size  | Value                             |
//! |--------|------------------|-------|-----------------------------------|
//! | 1      |Message ID        | 1     | 0                                 |
//! | 2      |Status Byte 1     | 1     | see [`HeartbeatStatusByte1`]      |
//! | 3      |Status Byte 2     | 1     | see [`HeartbeatStatusByte2`]      |
//! | 4-5    |Timestamp         | 2     | Seconds since 0000Z, bits 15-0 (LSB byte first) |
//! | 6-7    |Message Counts    | 2     | see [`MessageCounts`]             |
//! |        |Total length      | 7     |                                   |
//!

use binrw::BinRead;
use modular_bitfield::{bitfield, prelude::B4};

/// The Heartbeat message provides real-time indications of the status and operation of the GDL 90.
#[derive(BinRead, Debug)]
#[br(little)]
pub struct HeartbeatMessage {
    pub status_byte_1: HeartbeatStatusByte1,
    pub status_byte_2: HeartbeatStatusByte2,
    pub uat_timestamp: u16,
    pub message_counts: u16, // FIXME: type
}

/// Heartbeat Status Byte 1.
///
/// | Bit | Description        | Value | Meaning                                     |
/// |-----|--------------------|-------|---------------------------------------------|
/// | 7   | GPS Pos Valid      | 1     | Position is available for ADS-B Tx          |
/// | 6   | Maint Req'd        | 1     | GDL 90 Maintenance Req'd                    |
/// | 5   | IDENT              | 1     | IDENT talkback                              |
/// | 4   | Addr Type          | 1     | Address Type talkback                       |
/// | 3   | GPS Batt Low       | 1     | GPS Battery low voltage                     |
/// | 2   | RATCS              | 1     | ATC Services talkback                       |
/// | 1   | Reserved           | -     | -                                           |
/// | 0   | UAT Initialized    | 1     | GDL 90 is initialized                       |
#[bitfield]
#[derive(BinRead, Debug)]
#[br(little)]
pub struct HeartbeatStatusByte1 {
    /// This bit is set to `true` in all Heartbeat messages.
    pub uat_initialized: bool,

    /// Set to `false` in equipment that complies with this version of the specification.
    #[skip]
    reserved: bool,

    /// Set to the present state of the Receiving ATC Services indication in the transmitted ADS-B messages.
    pub ratcs: bool,

    /// Whether the GDL 90 needs maintenance to replace its internal GPS battery.
    pub gps_batt_low: bool,

    /// Whether the GDL 90 is transmitting ADS-B messages using a temporary self-assigned (“anonymous”) address.
    pub addr_type: bool,

    /// Whether the GDL 90 has set the `IDENT` indication in its transmitted ADS-B messages.
    pub ident: bool,

    /// Whether the GDL 90 has detected a problem and requires maintainence.
    pub maint_reqd: bool,

    /// Whether the GDL 90 has a valid position fix for ADS-B messages.
    pub gps_pos_valid: bool,
}

/// Heartbeat Status Byte 2.
///
/// | Bit | Description        | Value | Meaning                                     |
/// |-----|--------------------|-------|---------------------------------------------|
/// | 7   | Time Stamp (MS bit)| 1     | Seconds since 0000Z, bit 16                 |
/// | 6   | CSA Requested      | 1     | CSA has been requested                      |
/// | 5   | CSA Not Available  | 1     | CSA is not available at this time           |
/// | 4   | Reserved           | -     | -                                           |
/// | 3   | Reserved           | -     | -                                           |
/// | 2   | Reserved           | -     | -                                           |
/// | 1   | Reserved           | -     | -                                           |
/// | 0   | UTC OK             | 1     | UTC timing is valid                         |
#[bitfield]
#[derive(BinRead, Debug)]
#[br(little)]
pub struct HeartbeatStatusByte2 {
    pub utc_ok: bool,

    #[skip]
    reserved: B4,

    pub csa_not_available: bool,

    pub csa_requested: bool,

    pub timestamp_msb: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_works() {
        let mut data = Cursor::new(b"\x81\x41\xDB\xD0\x08\x02");
        let parsed = HeartbeatMessage::read(&mut data).unwrap();

        assert_eq!(parsed.status_byte_1.gps_pos_valid(), true);
        assert_eq!(parsed.status_byte_1.maint_reqd(), false);
        assert_eq!(parsed.status_byte_1.ident(), false);
        assert_eq!(parsed.status_byte_1.addr_type(), false);
        assert_eq!(parsed.status_byte_1.gps_batt_low(), false);
        assert_eq!(parsed.status_byte_1.ratcs(), false);
        assert_eq!(parsed.status_byte_1.uat_initialized(), true);

        assert_eq!(parsed.status_byte_2.timestamp_msb(), false);
        assert_eq!(parsed.status_byte_2.csa_requested(), true);
        assert_eq!(parsed.status_byte_2.csa_not_available(), false);
        assert_eq!(parsed.status_byte_2.utc_ok(), true);

        assert_eq!(parsed.uat_timestamp, 0xD0DB);
        assert_eq!(parsed.message_counts, 0x0208);
    }
}
