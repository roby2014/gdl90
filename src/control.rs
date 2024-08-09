//! Control Panel Interface
//!
//! The GDL 90 receives control messages over the Control Panel interface (on the DB15 - P1 connector),
//! using pin 12 (input to GDL 90) and pin 5 (ground). The interface uses an ASCII-text
//! basis, with an ASCII-encoded hexadecimal checksum. The checksum is the algebraic sum of the
//! message byte values. Messages are delimited with a carriage return character.
//!
//! ## Example
//! ```
//! use gdl90::control::VfrCodeMessage;
//! use gdl90::control::ToStringMessage;
//! use std::io::Cursor;
//!
//! let object = VfrCodeMessage {
//!     vfr_code: b"1200".clone(),
//!     checksum: b"DA".clone(),
//! };
//! assert_eq!(object.to_string_message(), "^VC 1200DA\r");
//! ```
//!

use std::io::Cursor;

use binrw::{binwrite, BinWrite};

pub trait ToStringMessage {
    /// Converts a Control message to string, ready to be sent.
    fn to_string_message(&self) -> String;
}

/// The call sign message provides for a user selectable call sign.
/// - Rate: Every 1 minute or when a change occurs
/// - Message Length: 15 bytes
#[binwrite]
#[bw(little, magic = b"^CS ")]
pub struct CallSignMessage {
    // TODO: fill unfillable bytes with spaces
    pub call_sign: [u8; 8],

    // TODO: calc checksum
    pub checksum: [u8; 2],

    #[bw(calc(b'\r'))]
    carriage: u8,
}

impl ToStringMessage for CallSignMessage {
    fn to_string_message(&self) -> String {
        let mut output = Cursor::new(vec![]);
        self.write(&mut output).unwrap();
        String::from_utf8(output.into_inner()).unwrap()
    }
}

/// The mode message indicates the current operating mode.
/// - Rate: 1 sec (nominal)
/// - Message Length: 17 bytes
#[binwrite]
#[bw(little, magic = b"^MD ")]
pub struct OperationModeMessage {
    pub mode: ModeField,

    #[bw(calc(b','))]
    comma: u8,

    pub ident: IdentField,

    #[bw(calc(b','))]
    comma: u8,

    pub squawk: [u8; 4],
    pub emergency: EmergencyField,
    pub healthy: HealthyField,

    // TODO: calc checksum
    pub checksum: [u8; 2],

    #[bw(calc(b'\r'))]
    carriage: u8,
}

impl ToStringMessage for OperationModeMessage {
    fn to_string_message(&self) -> String {
        let mut output = Cursor::new(vec![]);
        self.write(&mut output).unwrap();
        String::from_utf8(output.into_inner()).unwrap()
    }
}

/// The VFR Code message informs the GDL 90 of the squawk
/// code that is used to indicate the VFR operating condition.
/// - Rate: 1 minute
/// - Message Length 11 bytes
#[binwrite]
#[bw(little, magic = b"^VC ")]
pub struct VfrCodeMessage {
    pub vfr_code: [u8; 4],

    // TODO: calc checksum
    pub checksum: [u8; 2],

    #[bw(calc(b'\r'))]
    carriage: u8,
}

impl ToStringMessage for VfrCodeMessage {
    fn to_string_message(&self) -> String {
        let mut output = Cursor::new(vec![]);
        self.write(&mut output).unwrap();
        String::from_utf8(output.into_inner()).unwrap()
    }
}

#[derive(BinWrite)]
#[bw(little, repr = u8)]
pub enum ModeField {
    /// Standby Mode turns the GDL 90 transmitter off, so that no ADS-B messages are transmitted.
    StandBy = 0x4F,

    /// Mode A suppresses the transmission of pressure altitude in the ADS-B messages.
    ModeA = 0x41,

    /// Mode C is the normal operating mode, which includes pressure altitude.
    ModeC = 0x43,
}

/// When enabled, this causes the GDL 90 to include the IDENT
/// indication in transmitted ADS-B messages for the next 20 seconds.
#[derive(BinWrite)]
#[bw(little, repr = u8)]
pub enum IdentField {
    Enabled = 0x49,  // 'I'
    Inactive = 0x2D, // '-'
}

/// The Health indication is set to ‘1’ by the control panel to indicate that it is operating normally.
#[derive(BinWrite)]
#[bw(little, repr = u8)]
pub enum HealthyField {
    NotHealthy = 48, // '0'
    Healthy = 49,    // '1'
}

/// Any active emergency code is included in the GDL 90’s transmitted ADS-B messages.
#[derive(BinWrite)]
#[bw(little, repr = u8)]
pub enum EmergencyField {
    None = 48,    // 0x0
    General = 49, // 0x1
    Medical = 50, // 0x2
    Fuel = 51,    // 0x3
    Com = 52,     // 0x4
    Hijack = 53,  // 0x5
    Downed = 54,  // 0x6
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn call_sign() {
        let object = CallSignMessage {
            call_sign: "GARMIN  ".as_bytes().try_into().unwrap(),
            checksum: b"FA".clone(),
        };
        assert_eq!(object.to_string_message(), "^CS GARMIN  FE\r");
    }

    #[test]
    fn operation_mode() {
        let object = OperationModeMessage {
            mode: ModeField::ModeA,
            ident: IdentField::Enabled,
            squawk: b"2354".clone(),
            emergency: EmergencyField::None,
            healthy: HealthyField::Healthy,
            checksum: b"FA".clone(),
        };
        assert_eq!(object.to_string_message(), "^MD A,I,235401FA\r");
    }

    #[test]
    fn vfr_code() {
        let object = VfrCodeMessage {
            vfr_code: b"1200".clone(),
            checksum: b"DA".clone(),
        };
        assert_eq!(object.to_string_message(), "^VC 1200DA\r");
    }
}
