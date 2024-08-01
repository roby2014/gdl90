//! GDL90 Uplink Data message.
//!
//! | Byte # | Name             | Size  | Value                                         |
//! |--------|------------------|-------|-----------------------------------------------|
//! | 1      |Message ID        | 1     | 7                                             |
//! | 2-4    |Time of reception | 3     | 24-bit binary fraction Resolution = 80 nsec   |
//! | 5-436  |Uplink payload    | 432   | see [`UplinkPayload`]                         |
//! |        |Total length      | 436   |                                               |
//!

use binrw::BinRead;

/// Uplink messages received from UAT Ground Broadcast Transceivers are reported to the Display.
#[derive(BinRead, Debug)]
#[br(little)]
pub struct UplinkDataMessage {
    #[br(parse_with = binrw::helpers::read_u24)]
    pub time_of_reception: u32,
    pub payload: UplinkPayload,
}

#[derive(BinRead, Debug)]
#[br(little)]
pub struct UplinkPayload {
    pub uat_specific_header: u8,
    pub payload: [u8; 424],
}
