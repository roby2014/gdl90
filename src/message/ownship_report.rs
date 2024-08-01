//! GDL90 Ownship Report message.
//!
//! | Byte # | Name         | Size  | Value                         |
//! |--------|--------------|-------|-------------------------------|
//! | 1      |Message ID    | 1     | 10                            |
//! | 2-28   |Ownship Report| 27    | see [`OwnshipReportMessage`]  |
//! |        |Total length  | 28    |                               |
//!

use binrw::BinRead;

use super::types::report::Report;

/// The Ownship Report is output by the GDL 90 regardless of
/// whether a valid GPS position fix is available.
#[derive(BinRead, Debug)]
#[br(little)]
pub struct OwnshipReportMessage {
    pub ownship_report: Report,
}
