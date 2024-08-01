//! Custom definitions and types for GDL90 report messages (Ownship and Traffic).

use binrw::BinRead;
use modular_bitfield::{
    bitfield,
    prelude::{B24, B4, B8},
    BitfieldSpecifier,
};

use crate::message::types::{altitude::Altitude, call_sign::CallSignType, velocity::Velocity};

use super::cords::{LatitudeDirection, LongitudeDirection};

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
    pub latitude: LatitudeDirection,

    /// Longitude.
    pub longitude: LongitudeDirection,

    /// Altitude.
    pub altitude: Altitude,

    /// Miscellaneous indicator.
    pub misc_indicators: MiscIndicator,

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

/// 4-bit field which describes the miscellaneous indicator bits that apply to the Traffic Report field.
#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[bits = 4]
pub enum MiscIndicator {
    // FIXME: how to get non set bits? bit masking?
    TTNotValid,
    TTTrueTrackAngle,
    TTHeadingMagnetic,
    TTHeadingTrue,
    ReportUpdated,
    ReportExtrapolated,
    OnGround,
    Airborne,
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

#[cfg(test)]
mod tests {
    use crate::message::{
        ownship_report::OwnshipReportMessage,
        types::{
            altitude::Altitude,
            cords::{LatitudeDirection, LongitudeDirection},
            velocity::VelocityType,
        },
    };

    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_works_ownship_traffic() {
        // no message id and crc here!
        let data = vec![
            0x00, // st
            0xAB, 0x45, 0x49, // aa aa aa
            0x1F, 0xEF, 0x15, // ll ll ll
            0xA8, 0x89, 0x78, // nn nn nn
            0x0F, 0x09, // dd dm
            0xA9, // ia
            0x07, // hh
            0xB0, // hv
            0x01, // vv
            0x20, // tt
            0x01, // ee
            0x4E, 0x38, 0x32, 0x35, 0x56, 0x20, 0x20, 0x20, // cc ...
            0x00,
        ];
        let mut cursor = Cursor::new(data);
        let parsed = OwnshipReportMessage::read(&mut cursor).unwrap();

        assert_eq!(
            parsed.ownship_report.traffic_alert_status(),
            TrafficAlert::NoTraffic
        );
        assert_eq!(
            parsed.ownship_report.address_type(),
            AddressType::ADSBWithICAOAddress
        );
        assert_eq!(
            parsed.ownship_report.latitude(),
            LatitudeDirection::North(44.907066822052)
        );
        assert_eq!(
            parsed.ownship_report.longitude(),
            LongitudeDirection::West(-122.9948616027832)
        );
        assert_eq!(parsed.ownship_report.altitude(), Altitude::Valid(5000));
        assert_eq!(parsed.ownship_report.nic(), 10);
        assert_eq!(parsed.ownship_report.nacp(), 9);
        assert_eq!(
            parsed.ownship_report.velocity(),
            Velocity {
                h_vel: VelocityType::Horizontal(123),
                v_vel: VelocityType::Vertical(64)
            }
        );
        assert_eq!(
            parsed.ownship_report.emergency_priority_code(),
            EmergencyPriorityCodeCategory::NoEmergency
        );
        assert_eq!(
            parsed.ownship_report.emmiter_cattegory(),
            EmmiterCategory::Light
        );
        assert_eq!(parsed.ownship_report.call_sign().tail_number, "N825V");
    }
}
