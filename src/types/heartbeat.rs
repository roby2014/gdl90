//! GDL90 Heartbeat custom types. 560-1058-00 Rev A - ref 3.1.x

use binrw::BinRead;
use modular_bitfield::{bitfield, prelude::B4};

/// Heartbeat Status Byte 1. 560-1058-00 Rev A - ref 3.1.1
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

/// Heartbeat Status Byte 2. 560-1058-00 Rev A - ref 3.1.2
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
