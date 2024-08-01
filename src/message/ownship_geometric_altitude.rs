//! GDL90 Ownship Geometric Altitude message.
//!
//! | Byte # | Name                 | Size  | Value                                                             |
//! |--------|----------------------|-------|-------------------------------------------------------------------|
//! | 1      |Message ID            | 1     | 11                                                                |
//! | 2-3    |Ownship Geo Altitude  | 2     | Signed altitude in 5 ft. resolution                               |
//! | 4-5    |Vertical Metrics      | 2     | Vertical Warning indicator and Vertical Figure of Merit in meters |
//! |        |Total length          | 5     |                                                                   |
//!

use binrw::BinRead;

const GDL90_GEO_ALTITUDE_FACTOR: i16 = 5;

use super::types::vertical_metrics::VerticalMetricsType;

#[derive(BinRead, Debug)]
#[br(little)]
pub struct OwnshipGeometricAltitude {
    #[br(map = |x: i16| x*GDL90_GEO_ALTITUDE_FACTOR)]
    pub ownship_geo_altitude: i16,

    pub vertical_metrics: VerticalMetricsType,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::message::types::vertical_metrics::Vfom;

    use super::*;

    #[test]
    fn it_works() {
        let mut data = Cursor::new(b"\x38\xFF\x0A\x00");
        let parsed = OwnshipGeometricAltitude::read(&mut data).unwrap();
        assert_eq!(parsed.ownship_geo_altitude, -1000);
        assert_eq!(parsed.vertical_metrics.vertical_warning_indicator(), false);
        assert_eq!(
            parsed.vertical_metrics.vertical_figure_of_merit(),
            Vfom::Available(10)
        );
        // 0xFF38 0x000A
    }
}
