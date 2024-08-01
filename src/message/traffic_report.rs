//! GDL90 Traffic Report message.
//!
//! | Byte # | Name         | Size  | Value                         |
//! |--------|--------------|-------|-------------------------------|
//! | 1      |Message ID    | 1     | 10                            |
//! | 2-28   |Traffic Report| 27    | see [`TrafficReportMessage`]  |
//! |        |Total length  | 28    |                               |
//!

use binrw::BinRead;

use super::types::report::Report;

/// When the Traffic Alert interface is in use, a Traffic Report message
/// is output from the GDL 90 in each second for each alerted or proximate target.
#[derive(BinRead, Debug)]
#[br(little)]
pub struct TrafficReportMessage {
    pub traffic_report: Report,
}
