//! Altitude type.

use modular_bitfield::Specifier;

const GDL90_ALTITUDE_FACTOR: i32 = 25;
const GDL90_ALTITUDE_OFFSET: i32 = -1000;

#[derive(PartialEq, Debug)]
pub enum Altitude {
    Valid(i32),
    InvalidOrUnknown,
}

impl Specifier for Altitude {
    const BITS: usize = 12;
    type Bytes = u32;
    type InOut = Altitude;

    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, modular_bitfield::error::OutOfBounds> {
        unimplemented!()
    }

    fn from_bytes(
        input: Self::Bytes,
    ) -> Result<Self::InOut, modular_bitfield::error::InvalidBitPattern<Self::Bytes>> {
        // input has to be big-endian!
        let combined = u32::swap_bytes(input & 0x000FFFFF) >> 20;
        let factored = combined as i32 * GDL90_ALTITUDE_FACTOR;
        let value = factored + GDL90_ALTITUDE_OFFSET;
        if value == 0xFFF {
            Ok(Altitude::InvalidOrUnknown)
        } else {
            Ok(Altitude::Valid(value))
        }
    }
}
