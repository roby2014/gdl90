//! Velocity type.

use modular_bitfield::Specifier;

const GDL90_HORZ_VELOCITY_FACTOR: u16 = 1;
const GDL90_VERT_VELOCITY_FACTOR: i16 = 64;

#[derive(PartialEq, Debug)]
pub enum Velocity {
    Horizontal(u16),
    /// 12-bit signed value, in units of 64 feet per minute (FPM).
    /// Note: positive means climbing, negative means descending.
    Vertical(i16),
    Unavailable,
}

#[derive(PartialEq, Debug)]
pub struct VelocityType {
    h_vel: Velocity,
    v_vel: Velocity,
}

impl Specifier for VelocityType {
    const BITS: usize = 24;
    type Bytes = u32;
    type InOut = VelocityType;

    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, modular_bitfield::error::OutOfBounds> {
        unimplemented!()
    }

    fn from_bytes(
        input: Self::Bytes,
    ) -> Result<Self::InOut, modular_bitfield::error::InvalidBitPattern<Self::Bytes>> {
        // assuming input is big-endian, e.g 00 01 B0 07
        // we swap it so and make it 24-bit: 07 b0 01
        // vertical is 001
        // horizontal is 07b
        //
        let input = u32::swap_bytes(input) >> 8;

        // horizontal
        let combined_h: u16 = ((input & 0xFFF000) >> 12) as u16;
        let h_vel = if combined_h == 0xFFF {
            Velocity::Unavailable
        } else {
            Velocity::Horizontal(combined_h * GDL90_HORZ_VELOCITY_FACTOR)
        };

        // vertical
        let combined_v = (input & 0x000FFF) as i16;
        let v_vel = if combined_v == 0x800 {
            // no vertical velocity info available
            Velocity::Unavailable
        } else if (combined_v >= 0x1FF && combined_v <= 0x7FF)
            || (combined_v >= 0x801 && combined_v <= 0xE01)
        {
            // not used, invalid ranges
            Velocity::Unavailable
        } else if combined_v > 2047 {
            // convert 2s complement for negative values
            Velocity::Vertical((combined_v - 4096) * GDL90_VERT_VELOCITY_FACTOR)
        } else {
            Velocity::Vertical(combined_v * GDL90_VERT_VELOCITY_FACTOR)
        };

        Ok(VelocityType { h_vel, v_vel })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical() {
        // 01 b0 07 -> will get reversed to 07 b0 01 -> horizontal = 07b, vertical = 001
        assert_eq!(
            VelocityType::from_bytes(0x01b_007).unwrap(),
            VelocityType {
                h_vel: Velocity::Horizontal(123),
                v_vel: Velocity::Vertical(64)
            }
        );
        assert_eq!(
            VelocityType::from_bytes(0).unwrap(),
            VelocityType {
                h_vel: Velocity::Horizontal(0),
                v_vel: Velocity::Vertical(0)
            }
        );
        // 01 00 00 -> will get reversed to 00 00 01 -> horizontal = 010, vertical = 000
        assert_eq!(
            VelocityType::from_bytes(0x01_00_00).unwrap(),
            VelocityType {
                h_vel: Velocity::Horizontal(0),
                v_vel: Velocity::Vertical(64)
            }
        );
        // FF 0F 00 -> will get reversed to 00 0F FF -> horizontal = 000, vertical = FFF
        assert_eq!(
            VelocityType::from_bytes(0xFF_0F_00).unwrap(),
            VelocityType {
                h_vel: Velocity::Horizontal(0),
                v_vel: Velocity::Vertical(-64)
            }
        );
        // FE 01 00 -> will get reversed to 00 01 FE -> horizontal = 000, vertical = 1FE
        assert_eq!(
            VelocityType::from_bytes(0xFE_01_00).unwrap(),
            VelocityType {
                h_vel: Velocity::Horizontal(0),
                v_vel: Velocity::Vertical(32640)
            }
        );
        // 03 0E 00 -> will get reversed to 00 0E 03 -> horizontal = 000, vertical = E03
        assert_eq!(
            VelocityType::from_bytes(0x03_0E_00).unwrap(),
            VelocityType {
                h_vel: Velocity::Horizontal(0),
                v_vel: Velocity::Vertical(-32576)
            }
        );
        // 02 0E 00 -> will get reversed to 00 0E 02 -> horizontal = 000, vertical = E02
        assert_eq!(
            VelocityType::from_bytes(0x02_0E_00).unwrap(),
            VelocityType {
                h_vel: Velocity::Horizontal(0),
                v_vel: Velocity::Vertical(-32640)
            }
        );
        // 00 08 00 -> will get reversed to 00 08 00 -> horizontal = 000, vertical = 800
        assert_eq!(
            VelocityType::from_bytes(0x00_08_00).unwrap(),
            VelocityType {
                h_vel: Velocity::Horizontal(0),
                v_vel: Velocity::Unavailable
            }
        );

        #[test]
        fn horizontal() {
            //todo!();
        }
    }
}
