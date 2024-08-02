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
use modular_bitfield::{bitfield, Specifier};

const GDL90_GEO_ALTITUDE_FACTOR: i16 = 5;

/// An Ownship Geometric Altitude message will be transmitted
/// with a period of one second when the GNSS fix is valid.
#[derive(BinRead, Debug)]
#[br(little)]
pub struct OwnshipGeometricAltitude {
    #[br(map = |x: i16| x*GDL90_GEO_ALTITUDE_FACTOR)]
    pub ownship_geo_altitude: i16,

    pub vertical_metrics: VerticalMetrics,
}

/// Vertical Metrics wrapper.
#[bitfield]
#[derive(BinRead, Debug)]
#[br(little)]
pub struct VerticalMetrics {
    pub vertical_figure_of_merit: Vfom,
    pub vertical_warning_indicator: bool,
}

/// Vertical Figure of Merit (VFOM).
#[derive(PartialEq, Debug)]
pub enum Vfom {
    Available(u16),
    Unavailable,
}

impl Specifier for Vfom {
    const BITS: usize = 15;
    type Bytes = u16;
    type InOut = Vfom;

    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, modular_bitfield::error::OutOfBounds> {
        unimplemented!()
    }

    fn from_bytes(
        input: Self::Bytes,
    ) -> Result<Self::InOut, modular_bitfield::error::InvalidBitPattern<Self::Bytes>> {
        let masked = input & 0x7FFF;
        let res = match masked {
            0x7FFF => Vfom::Unavailable,
            // 0x7FFE => ??? HANDLE SATURATION?
            _ => Vfom::Available(masked),
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn ownship_works() {
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

    #[test]
    fn vfom_bitfield_works() {
        assert_eq!(Vfom::from_bytes(0xFFFF).unwrap(), Vfom::Unavailable);
        //assert_eq!(Vfom::from_bytes(0x7FFE).unwrap(), Vfom::Available(40000)); // FIXME SATURATION?
        assert_eq!(Vfom::from_bytes(0x000A).unwrap(), Vfom::Available(10));
        assert_eq!(Vfom::from_bytes(0x8032).unwrap(), Vfom::Available(50));
    }

    #[test]
    fn vertical_metrics_works() {
        let parsed = VerticalMetrics::read(&mut Cursor::new(b"\xFF\xFF")).unwrap();
        assert_eq!(parsed.vertical_warning_indicator(), true);
        assert_eq!(parsed.vertical_figure_of_merit(), Vfom::Unavailable);

        let parsed = VerticalMetrics::read(&mut Cursor::new(b"\xFE\x7F")).unwrap();
        assert_eq!(parsed.vertical_warning_indicator(), false);
        // assert_eq!(parsed.vertical_figure_of_merit(), Vfom::Available(40000)); // FIXME SATURATION?

        let parsed = VerticalMetrics::read(&mut Cursor::new(b"\x0A\x00")).unwrap();
        assert_eq!(parsed.vertical_warning_indicator(), false);
        assert_eq!(parsed.vertical_figure_of_merit(), Vfom::Available(10));

        let parsed = VerticalMetrics::read(&mut Cursor::new(b"\x32\x80")).unwrap();
        assert_eq!(parsed.vertical_warning_indicator(), true);
        assert_eq!(parsed.vertical_figure_of_merit(), Vfom::Available(50));
    }
}
