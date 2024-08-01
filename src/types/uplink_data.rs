//! Uplink Data Payload. 560-1058-00 Rev A - ref 3.3.x

use binrw::BinRead;

#[derive(BinRead, Debug)]
#[br(little)]
pub struct UplinkPayload {
    pub uat_specific_header: u8,
    pub payload: [u8; 424],
}
