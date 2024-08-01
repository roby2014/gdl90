//! GDL90 Control Panel Interface. 560-1058-00 Rev A - ref 6.x
//!
//! | ID    | Description               | Notes                         | Ref   |
//! |-------|---------------------------|-------------------------------|-------|
//! | ^CS   | Call Sign                 | 1 min interval or on change   | 6.2.1 |
//! | ^MD   | Operation Mode Message    | 1 second interval (nominal)   | 6.2.2 |
//! | ^VC   | VFR Code                  | 1 min interval                | 6.2.3 |
//!
//! The GDL 90 receives control messages over the Control Panel interface (on the DB15 - P1 connector),
//! using pin 12 (input to GDL 90) and pin 5 (ground). The interface uses an ASCII-text
//! basis, with an ASCII-encoded hexadecimal checksum. The checksum is the algebraic sum of the
//! message byte values. Messages are delimited with a carriage return character.
//!
//! This module contains utilities for creating control panel messages fast and easy.
//!
//! ## Example
//! ```
//! use gdl90::control::*;
//!
//! let object = OperationModeMessage {
//!      mode: ModeField::ModeA,
//!      ident: IdentField::Enabled,
//!      squawk: 2345,
//!      emergency: EmergencyField::None,
//!      healthy: HealthyField::Healthy,
//! };
//! assert_eq!(object.to_string_message(), "^MD A,I,23450120\r");
//! // write to transponder..
//! ```

use std::io::{Cursor, Seek, SeekFrom, Write};

use binrw::{binwrite, BinWrite};

/// Trait that each GDL90 Control Panel Interface type should implement in order to get data to be sent.
pub trait ToStringMessage {
    /// Converts a GDL90 Control message to string, ready to be sent.
    fn to_string_message(&self) -> String;
}

/// The call sign message provides for a user selectable call sign.
/// - Rate: Every 1 minute or when a change occurs
/// - Message Length: 15 bytes
#[binwrite]
#[bw(little, stream = w, map_stream = Checksum::new)]
pub struct CallSignMessage {
    #[bw(calc(b"^CS "))]
    id: &[u8; 4],

    // TODO: fill unfillable bytes with spaces
    #[bw(map = |x| str_to_eight_digit_ascii(&x))]
    pub call_sign: String,

    #[bw(calc(w.check()))]
    checksum: [u8; 2],

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
#[bw(little, stream = w, map_stream = Checksum::new)]
pub struct OperationModeMessage {
    #[bw(calc(b"^MD "))]
    id: &[u8; 4],

    pub mode: ModeField,

    #[bw(calc(b','))]
    comma: u8,

    pub ident: IdentField,

    #[bw(calc(b','))]
    comma: u8,

    #[bw(map = |x| u16_to_four_digit_ascii(*x))]
    pub squawk: u16,

    pub emergency: EmergencyField,

    pub healthy: HealthyField,

    #[bw(calc(w.check()))]
    checksum: [u8; 2],

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
#[bw(little, stream = w, map_stream = Checksum::new)]
pub struct VfrCodeMessage {
    #[bw(calc(b"^VC "))]
    id: &[u8; 4],

    #[bw(map = |x| u16_to_four_digit_ascii(*x))]
    pub vfr_code: u16,

    #[bw(calc(w.check()))]
    checksum: [u8; 2],

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

/// GDL90 Operating mode field.
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

struct Checksum<T> {
    inner: T,
    check: core::num::Wrapping<u8>,
}

impl<T> Checksum<T> {
    fn new(inner: T) -> Self {
        Self {
            inner,
            check: core::num::Wrapping(0),
        }
    }

    fn check(&self) -> [u8; 2] {
        let hex_str = format!("{:02X}", self.check.0);
        let s1 = hex_str.chars().nth(0).unwrap() as u8;
        let s2 = hex_str.chars().nth(1).unwrap() as u8;
        [s1, s2]
    }
}

impl<T: Write> Write for Checksum<T> {
    fn write(&mut self, buf: &[u8]) -> binrw::io::Result<usize> {
        for b in buf {
            self.check += b;
        }
        self.inner.write(buf)
    }

    fn flush(&mut self) -> binrw::io::Result<()> {
        self.inner.flush()
    }
}

impl<T: Seek> Seek for Checksum<T> {
    fn seek(&mut self, pos: SeekFrom) -> binrw::io::Result<u64> {
        self.inner.seek(pos)
    }
}

/// Converts `x` as a four digit ASCII-text.
fn u16_to_four_digit_ascii(x: u16) -> [u8; 4] {
    let mut s = format!("{:04}", x);
    s.truncate(4);
    s.as_bytes().try_into().unwrap_or([0, 0, 0, 0])
}

/// Converts `x` to a eight digit ASCII-text.
fn str_to_eight_digit_ascii(x: &str) -> [u8; 8] {
    let mut buffer = [b' '; 8];
    let bytes = x.as_bytes();

    let len = bytes.len().min(8);
    buffer[..len].copy_from_slice(&bytes[..len]);

    buffer.try_into().unwrap_or([0, 0, 0, 0, 0, 0, 0, 0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_sign() {
        let object = CallSignMessage {
            call_sign: "GARMIN".to_owned(),
        };
        assert_eq!(object.to_string_message(), "^CS GARMIN  12\r");
    }

    #[test]
    fn operation_mode() {
        let object = OperationModeMessage {
            mode: ModeField::ModeA,
            ident: IdentField::Enabled,
            squawk: 2345,
            emergency: EmergencyField::None,
            healthy: HealthyField::Healthy,
        };
        assert_eq!(object.to_string_message(), "^MD A,I,23450120\r");
    }

    #[test]
    fn vfr_code() {
        let object = VfrCodeMessage { vfr_code: 1200 };
        assert_eq!(object.to_string_message(), "^VC 1200DA\r");
    }
}
