//! Latitude and longitude types.

use modular_bitfield::Specifier;

#[derive(PartialEq, Debug)]
pub enum LatitudeDirection {
    North(f64),
    South(f64),
}

#[derive(PartialEq, Debug)]
pub enum LongitudeDirection {
    East(f64),
    West(f64),
}

pub struct LatitudeType {}

pub struct LongitudeType {}

// FIXME: duplicate code...

impl Specifier for LatitudeType {
    const BITS: usize = 24;
    type Bytes = u32;
    type InOut = LatitudeDirection;

    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, modular_bitfield::error::OutOfBounds> {
        unimplemented!()
    }

    fn from_bytes(
        input: Self::Bytes,
    ) -> Result<Self::InOut, modular_bitfield::error::InvalidBitPattern<Self::Bytes>> {
        // input has to be big-endian!

        let combined = u32::swap_bytes(input & 0x00FFFFFF) >> 8;

        let value = if combined & 0x800000 != 0 {
            (combined as i32) | !0xFFFFFF // sign-extend to 32 bits
        } else {
            combined as i32
        };

        // convert from semicircle to degrees
        let degrees = (value as f64) * (180.0 / (1 << 23) as f64);

        if degrees.is_sign_negative() {
            Ok(LatitudeDirection::South(degrees))
        } else {
            Ok(LatitudeDirection::North(degrees))
        }
    }
}

impl Specifier for LongitudeType {
    const BITS: usize = 24;
    type Bytes = u32;
    type InOut = LongitudeDirection;

    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, modular_bitfield::error::OutOfBounds> {
        unimplemented!()
    }

    fn from_bytes(
        input: Self::Bytes,
    ) -> Result<Self::InOut, modular_bitfield::error::InvalidBitPattern<Self::Bytes>> {
        // input has to be big-endian!

        let combined = u32::swap_bytes(input & 0x00FFFFFF) >> 8;

        let value = if combined & 0x800000 != 0 {
            (combined as i32) | !0xFFFFFF // sign-extend to 32 bits
        } else {
            combined as i32
        };

        // convert from semicircle to degrees
        let degrees = (value as f64) * (180.0 / (1 << 23) as f64);

        if degrees.is_sign_negative() {
            Ok(LongitudeDirection::West(degrees))
        } else {
            Ok(LongitudeDirection::East(degrees))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latitude_works() {
        //assert_eq!(LatitudeType::from_bytes(0x010000).unwrap(), 0.0); FIXME MAX RANGE ??
        //assert_eq!(LatitudeType::from_bytes(0xFFFFFF).unwrap(), 0.0); FIXME MAX RANGE ??
        assert_eq!(
            LatitudeType::from_bytes(0x000000).unwrap(),
            LatitudeDirection::North(0.0)
        );
        assert_eq!(
            LatitudeType::from_bytes(0x000020).unwrap(),
            LatitudeDirection::North(45.0)
        );
        assert_eq!(
            LatitudeType::from_bytes(0x0000E0).unwrap(),
            LatitudeDirection::South(-45.0)
        );
        assert_eq!(
            LatitudeType::from_bytes(0x000040).unwrap(),
            LatitudeDirection::North(90.0)
        );
        assert_eq!(
            LatitudeType::from_bytes(0x000080).unwrap(),
            LatitudeDirection::South(-180.0)
        );
        match LatitudeType::from_bytes(0xFFFF7F).unwrap() {
            LatitudeDirection::North(val) => assert!(val > 179.0), // almost 180..
            LatitudeDirection::South(val) => assert!(val > 179.0), // almost 180..
        }
    }
}
