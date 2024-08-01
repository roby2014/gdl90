//! Vertical Metrics type.

use binrw::BinRead;
use modular_bitfield::{bitfield, Specifier};

#[derive(PartialEq, Debug)]
pub enum Vfom {
    Available(u16),
    Unavailable,
}

#[bitfield]
#[derive(BinRead, Debug)]
#[br(little)]
pub struct VerticalMetricsType {
    pub vertical_figure_of_merit: Vfom,
    pub vertical_warning_indicator: bool,
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
        println!("aq {}", input);
        let masked = input & 0x7FFF;

        let res = match masked {
            0x7FFF => Vfom::Unavailable,
            // 0x7FFE => ??? is it always hardcoded to 40000?
            _ => Vfom::Available(masked),
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn vfom_works() {
        assert_eq!(Vfom::from_bytes(0xFFFF).unwrap(), Vfom::Unavailable);
        //assert_eq!(Vfom::from_bytes(0x7FFE).unwrap(), Vfom::Available(40000)); // FIXME?
        assert_eq!(Vfom::from_bytes(0x000A).unwrap(), Vfom::Available(10));
        assert_eq!(Vfom::from_bytes(0x8032).unwrap(), Vfom::Available(50));
    }

    #[test]
    fn vertical_metrics_work() {
        let parsed = VerticalMetricsType::read(&mut Cursor::new(b"\xFF\xFF")).unwrap();
        assert_eq!(parsed.vertical_warning_indicator(), true);
        assert_eq!(parsed.vertical_figure_of_merit(), Vfom::Unavailable);

        let parsed = VerticalMetricsType::read(&mut Cursor::new(b"\xFE\x7F")).unwrap();
        assert_eq!(parsed.vertical_warning_indicator(), false);
        // assert_eq!(parsed.vertical_figure_of_merit(), Vfom::Available(40000)); // FIXME?

        let parsed = VerticalMetricsType::read(&mut Cursor::new(b"\x0A\x00")).unwrap();
        assert_eq!(parsed.vertical_warning_indicator(), false);
        assert_eq!(parsed.vertical_figure_of_merit(), Vfom::Available(10));

        let parsed = VerticalMetricsType::read(&mut Cursor::new(b"\x32\x80")).unwrap();
        assert_eq!(parsed.vertical_warning_indicator(), true);
        assert_eq!(parsed.vertical_figure_of_merit(), Vfom::Available(50));
    }
}
