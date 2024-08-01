//! GDL90 Report message and types (for Ownship and Traffic). 560-1058-00 Rev A - ref 3.5.1.x

use binrw::BinRead;
use modular_bitfield::{
    bitfield,
    prelude::{B24, B4, B8},
    BitfieldSpecifier, Specifier,
};

/// Common Report data structure.
#[bitfield]
#[derive(BinRead, Debug)]
pub struct Report {
    /// Traffic Alert Status.
    pub traffic_alert_status: TrafficAlert,

    /// Address Type.
    pub address_type: AddressType,

    /// Participant Address.
    pub participant_address: B24,

    /// Latitude.
    pub latitude: Cord,

    /// Longitude.
    pub longitude: Cord,

    /// Miscellaneous indicator.
    //pub misc_indicators: B4, // FIXME: MiscIndicator, // altitude gets all the bytes?

    /// Altitude.
    pub altitude: Altitude,

    /// Navigation Accuracy Category for Position. TODO: better type?
    pub nacp: B4,

    /// Navigation Integrity Category. TODO: better type?
    pub nic: B4,

    // Velocity.
    pub velocity: Velocity,

    /// Track/Heading.
    pub track_heading: B8,

    /// Emitter Category.
    pub emmiter_cattegory: EmmiterCategory,

    /// Call Sign.
    pub call_sign: CallSignType,

    /// Emergency/Priority Code.
    pub emergency_priority_code: EmergencyPriorityCodeCategory,

    /// Spare (reserved for future use).
    pub reserved: B4,
}

/// 4-bit field which indicates whether CSA has identified this target with an alert.
#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[bits = 4]
pub enum TrafficAlert {
    NoTraffic,
    TrafficAlert,
    Reserved0,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
    Reserved9,
    Reserved10,
    Reserved11,
    Reserved12,
    Reserved13,
}

/// 4-bit field which describes the type of address conveyed in the [`Report::participant_address`] field.
#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[bits = 4]
pub enum AddressType {
    ADSBWithICAOAddress,
    ADSBWithSelfAssignedAddress,
    TISBWithICAOAddress,
    TISBWithTrackFileID,
    SurfaceVehicle,
    GroundStationBeacon,
    Reserved0,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
    Reserved9,
}

/// 8-bit field which describes the Emmiter Category.
#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[bits = 8]
pub enum EmmiterCategory {
    NoAircraftTypeInformation,
    Light,
    Small,
    Large,
    HighVortexLarge,
    Heavy,
    HighlyManeuverable,
    Rotorcraft,
    Unassigned0,
    GliderSailplane,
    LighterThanAir,
    ParachutistSkyDiver,
    UltraLightHangGliderParaglider,
    Unassigned1,
    UnmannedAerialVehicle,
    SpaceTransatmosphericVehicle,
    Unassigned2,
    SurfaceVehicleEmergency,
    SurfaceVehicleService,
    PointObstacle,
    ClusterObstacle,
    LineObstacle,
    Reserved0,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
    Reserved9,
    Reserved10,
    Reserved11,
    Reserved12,
    Reserved13,
    Reserved14,
    Reserved15,
    Reserved16,
    Reserved17,
}

/// 4-bit field which provides status information about the traffic.
#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[bits = 4]
pub enum EmergencyPriorityCodeCategory {
    NoEmergency,
    GeneralEmergency,
    MedicalEmergency,
    MinimumFuel,
    NoCommunication,
    UnlawfulInterference,
    DownedAircraft,
    Reserved0,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
}

/// 4-bit field which describes the miscellaneous indicator bits that apply to the Traffic Report field.
#[derive(Debug, PartialEq)]
pub enum MiscIndicator {
    // FIXME: how to get non set bits? bit masking?
    TrackHeadingNotValid,
    TrackHeadingTrueTrackAngle,
    TrackHeadingMagnetic,
    TrackHeadingTrue,
    ReportUpdated,
    ReportExtrapolated,
    OnGround,
    Airborne,
}

impl Specifier for MiscIndicator {
    const BITS: usize = 4;
    type Bytes = u8;
    type InOut = MiscIndicator;

    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, modular_bitfield::error::OutOfBounds> {
        unimplemented!()
    }

    fn from_bytes(
        input: Self::Bytes,
    ) -> Result<Self::InOut, modular_bitfield::error::InvalidBitPattern<Self::Bytes>> {
        dbg!(input);
        // FIXME!
        let res = match u8::swap_bytes(input & 0b0011) {
            // check bit 0 and 1
            0b00 => MiscIndicator::TrackHeadingNotValid,
            0b01 => MiscIndicator::TrackHeadingTrueTrackAngle,
            0b10 => MiscIndicator::TrackHeadingMagnetic,
            0b11 => MiscIndicator::TrackHeadingTrue,
            _ => {
                // TODO: handle bit 2 and 3?
                MiscIndicator::OnGround
            }
        };
        Ok(res)
    }
}

const GDL90_ALTITUDE_FACTOR: i32 = 25;
const GDL90_ALTITUDE_OFFSET: i32 = -1000;

#[derive(PartialEq, Debug)]
pub enum Altitude {
    Valid(i32),
    InvalidOrUnknown,
}

impl Specifier for Altitude {
    const BITS: usize = 16; // FIXME: should be 12 and 4 for misc, but cant get it to work
    type Bytes = u16;
    type InOut = Altitude;

    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, modular_bitfield::error::OutOfBounds> {
        unimplemented!()
    }

    fn from_bytes(
        input: Self::Bytes,
    ) -> Result<Self::InOut, modular_bitfield::error::InvalidBitPattern<Self::Bytes>> {
        let swapped = u16::swap_bytes(input) >> 4;
        let factored = swapped as i32 * GDL90_ALTITUDE_FACTOR;
        let value = factored + GDL90_ALTITUDE_OFFSET;
        if value == 0xFFF {
            Ok(Altitude::InvalidOrUnknown)
        } else {
            Ok(Altitude::Valid(value))
        }
    }
}

#[derive(Debug)]
pub struct CallSignType {
    pub tail_number: String,
}

impl Specifier for CallSignType {
    const BITS: usize = 64;
    type Bytes = u64;
    type InOut = CallSignType;

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
        Ok(CallSignType { tail_number: str })
    }
}

/// Geographic coordinate (latitude/longitude).
pub struct Cord;

impl Specifier for Cord {
    const BITS: usize = 24;
    type Bytes = u32;
    type InOut = f32;

    fn into_bytes(input: Self::InOut) -> Result<Self::Bytes, modular_bitfield::error::OutOfBounds> {
        unimplemented!()
    }

    fn from_bytes(
        input: Self::Bytes,
    ) -> Result<Self::InOut, modular_bitfield::error::InvalidBitPattern<Self::Bytes>> {
        let combined = u32::swap_bytes(input & 0x00FFFFFF) >> 8;

        let value = if combined & 0x800000 != 0 {
            (combined as i32) | !0xFFFFFF // sign-extend to 32 bits
        } else {
            combined as i32
        };

        // convert from semicircle to degrees
        let degrees = (value as f32) * (180.0 / (1 << 23) as f32);

        Ok(degrees)
    }
}

const GDL90_HORZ_VELOCITY_FACTOR: u16 = 1;
const GDL90_VERT_VELOCITY_FACTOR: i16 = 64;

#[derive(PartialEq, Debug)]
pub enum VelocityType {
    Horizontal(u16),
    /// 12-bit signed value, in units of 64 feet per minute (FPM).
    /// Note: positive means climbing, negative means descending.
    Vertical(i16),
    Unavailable,
}

#[derive(PartialEq, Debug)]
pub struct Velocity {
    pub h_vel: VelocityType,
    pub v_vel: VelocityType,
}

impl Specifier for Velocity {
    const BITS: usize = 24;
    type Bytes = u32;
    type InOut = Velocity;

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
        let input = u32::swap_bytes(input) >> 8;

        // horizontal
        let combined_h: u16 = ((input & 0xFFF000) >> 12) as u16;
        let h_vel = if combined_h == 0xFFF {
            VelocityType::Unavailable
        } else {
            VelocityType::Horizontal(combined_h * GDL90_HORZ_VELOCITY_FACTOR)
        };

        // vertical
        let combined_v = (input & 0x000FFF) as i16;
        let v_vel = if combined_v == 0x800 {
            // no vertical velocity info available
            VelocityType::Unavailable
        } else if (combined_v >= 0x1FF && combined_v <= 0x7FF)
            || (combined_v >= 0x801 && combined_v <= 0xE01)
        {
            // not used, invalid ranges
            VelocityType::Unavailable
        } else if combined_v > 2047 {
            // convert 2s complement for negative values
            VelocityType::Vertical((combined_v - 4096) * GDL90_VERT_VELOCITY_FACTOR)
        } else {
            VelocityType::Vertical(combined_v * GDL90_VERT_VELOCITY_FACTOR)
        };

        Ok(Velocity { h_vel, v_vel })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn callsign_works() {
        let data = u64::to_be(0x4e38323556202020);
        assert_eq!(CallSignType::from_bytes(data).unwrap().tail_number, "N825V");
    }

    #[test]
    fn latitude_works() {
        //assert_eq!(LatitudeType::from_bytes(0x010000).unwrap(), 0.0); FIXME MAX RANGE ??
        //assert_eq!(LatitudeType::from_bytes(0xFFFFFF).unwrap(), 0.0); FIXME MAX RANGE ??
        assert_eq!(Cord::from_bytes(0x000000).unwrap(), 0.0);
        assert_eq!(Cord::from_bytes(0x000020).unwrap(), 45.0);
        assert_eq!(Cord::from_bytes(0x0000E0).unwrap(), -45.0);
        assert_eq!(Cord::from_bytes(0x000040).unwrap(), 90.0);
        assert_eq!(Cord::from_bytes(0x000080).unwrap(), -180.0);
    }

    #[test]
    fn vertical() {
        // 01 b0 07 -> will get reversed to 07 b0 01 -> horizontal = 07b, vertical = 001
        assert_eq!(
            Velocity::from_bytes(0x01b_007).unwrap(),
            Velocity {
                h_vel: VelocityType::Horizontal(123),
                v_vel: VelocityType::Vertical(64)
            }
        );
        assert_eq!(
            Velocity::from_bytes(0).unwrap(),
            Velocity {
                h_vel: VelocityType::Horizontal(0),
                v_vel: VelocityType::Vertical(0)
            }
        );
        // 01 00 00 -> will get reversed to 00 00 01 -> horizontal = 010, vertical = 000
        assert_eq!(
            Velocity::from_bytes(0x01_00_00).unwrap(),
            Velocity {
                h_vel: VelocityType::Horizontal(0),
                v_vel: VelocityType::Vertical(64)
            }
        );
        // FF 0F 00 -> will get reversed to 00 0F FF -> horizontal = 000, vertical = FFF
        assert_eq!(
            Velocity::from_bytes(0xFF_0F_00).unwrap(),
            Velocity {
                h_vel: VelocityType::Horizontal(0),
                v_vel: VelocityType::Vertical(-64)
            }
        );
        // FE 01 00 -> will get reversed to 00 01 FE -> horizontal = 000, vertical = 1FE
        assert_eq!(
            Velocity::from_bytes(0xFE_01_00).unwrap(),
            Velocity {
                h_vel: VelocityType::Horizontal(0),
                v_vel: VelocityType::Vertical(32640)
            }
        );
        // 03 0E 00 -> will get reversed to 00 0E 03 -> horizontal = 000, vertical = E03
        assert_eq!(
            Velocity::from_bytes(0x03_0E_00).unwrap(),
            Velocity {
                h_vel: VelocityType::Horizontal(0),
                v_vel: VelocityType::Vertical(-32576)
            }
        );
        // 02 0E 00 -> will get reversed to 00 0E 02 -> horizontal = 000, vertical = E02
        assert_eq!(
            Velocity::from_bytes(0x02_0E_00).unwrap(),
            Velocity {
                h_vel: VelocityType::Horizontal(0),
                v_vel: VelocityType::Vertical(-32640)
            }
        );
        // 00 08 00 -> will get reversed to 00 08 00 -> horizontal = 000, vertical = 800
        assert_eq!(
            Velocity::from_bytes(0x00_08_00).unwrap(),
            Velocity {
                h_vel: VelocityType::Horizontal(0),
                v_vel: VelocityType::Unavailable
            }
        )
    }

    #[test]
    fn horizontal() {
        //todo!();
    }
}
