//! Call sign type.

use modular_bitfield::Specifier;

pub struct CallSignType {}

impl Specifier for CallSignType {
    const BITS: usize = 64;
    type Bytes = u64;
    type InOut = String;

    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, modular_bitfield::error::OutOfBounds> {
        unimplemented!()
    }

    fn from_bytes(
        input: Self::Bytes,
    ) -> Result<Self::InOut, modular_bitfield::error::InvalidBitPattern<Self::Bytes>> {
        let str = std::str::from_utf8(&input.to_le_bytes())
            .unwrap_or("invalid_call_sign")
            .trim()
            .to_string();
        Ok(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = u64::to_be(0x4e38323556202020);
        assert_eq!(CallSignType::from_bytes(data).unwrap(), "N825V");
    }
}
