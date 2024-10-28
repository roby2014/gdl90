//! A Rust crate for GDL90 message handling.
//!
//! Uses crates like [`binrw`] and [`modular_bitfield`] for reading/writing data in binary format.
//!
//! Read GDL90 especification [here](https://www.faa.gov/sites/faa.gov/files/air_traffic/technology/adsb/archival/GDL90_Public_ICD_RevA.PDF).
//!
//! | 1 byte (0x7E)    | 1 byte       | N bytes      | 2 bytes              | 1 byte (0x7E) |
//! |------------------|--------------|--------------|----------------------|---------------|
//! | Start Flag byte  | Message ID   | Message Data | Frame Check Sequence | End Flag byte |
//!
//!
//! # Usage
//! ```
//! use std::io::Cursor;
//! use gdl90::datalink::Gdl90DatalinkMessage;
//! use gdl90::Gdl90Message;
//!
//! // hardcoded, but you should read this from transponder
//! let parsed = gdl90::read_raw(&[0x7E, 0x00, 0x81, 0x41, 0xDB, 0xD0, 0x08, 0x02, 0xB3, 0x8B, 0x7E]).unwrap();
//! assert_eq!(parsed.frame_check_seq, 0x8bb3);
//! match parsed.message_data {
//!     Gdl90DatalinkMessage::Heartbeat { status_byte_1, status_byte_2, uat_timestamp, message_counts } => { /*...*/} ,
//!     Gdl90DatalinkMessage::Initialization { configuration_byte_1, configuration_byte_2 } => { /*...*/},
//!     Gdl90DatalinkMessage::UplinkData { time_of_reception, payload } => { /*...*/},
//!     Gdl90DatalinkMessage::HeightAboveTerrain { hat } => { /*...*/},
//!     Gdl90DatalinkMessage::OwnshipReport { report } => { /*...*/},
//!     Gdl90DatalinkMessage::TrafficReport { report } => { /*...*/},
//!     Gdl90DatalinkMessage::OwnshipGeoometricAltitude { ownship_geo_altitude, vertical_metrics } => { /*...*/},
//!     Gdl90DatalinkMessage::BasicReport() => { /*...*/},
//!     Gdl90DatalinkMessage::LongReport() => { /*...*/},
//!     Gdl90DatalinkMessage::Unknown => { /*...*/},
//! }
//! ```
//!
//! See [`Gdl90Message`] for more usage details.
//!
//! Note: Work in progress, feel free to contribute.

pub mod control;
pub mod crc;
pub mod datalink;
pub mod types;

use std::io::Cursor;

use binrw::{binread, BinRead};
use crc::gdl90_crc;
use datalink::Gdl90DatalinkMessage;

pub const GDL90_ESCAPEBYTE: u8 = 0x7D;
pub const GDL90_MAGIC: u8 = 0x7E;

/// Represents a full GDL90 message.
///
/// As it implements [`BinRead`] trait, you can try to decode any
/// binary message using it, e.g:
/// ```
/// use gdl90::Gdl90Message;
/// use binrw::BinRead;
/// use std::io::Cursor;
///
/// let mut data = Cursor::new(b"\x7E\x00\x81\x41\xDB\xD0\x08\x02\xB3\x8B\x7E");
/// let parsed = Gdl90Message::read(&mut data).unwrap();
/// // unwrap fails if the specified data is not 100% correct
/// ```
///
/// If you dont want to use the trait, you can use the aux function [`read_raw`]
/// ```
/// let parsed = gdl90::read_raw(&[0x7E, 0x00, 0x81, 0x41, 0xDB, 0xD0, 0x08, 0x02, 0xB3, 0x8B, 0x7E]);
/// ```
#[binread]
#[derive(Debug)]
#[br(little, magic = b"\x7E")]
pub struct Gdl90Message {
    #[br(temp, parse_with = parse_message_bytes)]
    data: Vec<u8>,

    /// Message payload depending on message id.
    #[br(map_stream = |_| Cursor::new(&data))]
    pub message_data: Gdl90DatalinkMessage,

    /// Frame Check Sequence. If not valid, assertion fails.
    #[br(assert(frame_check_seq == gdl90_crc(&data), "bad checksum of {:02X?}: {:#x?} != {:#x?}", data, frame_check_seq, gdl90_crc(&data)))]
    pub frame_check_seq: u16,
}

/// Reads from a raw buffer. Internally, it creates a `Cursor` and uses `BinRead` trait.
pub fn read_raw(buffer: &[u8]) -> Result<Gdl90Message, String> {
    Gdl90Message::read(&mut Cursor::new(buffer)).map_err(|err| format!("{err:?}").to_string())
}

/// 2.2.1. - Look for all Control-Escape characters in the saved string. Discard each one found, and XOR the
/// following character with 0x20.
fn remove_escapes(data: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let byte = data[i];
        if byte == GDL90_ESCAPEBYTE {
            if i + 1 < data.len() {
                i += 1;
                let escaped_byte = data[i] ^ 0x20;
                // FIXME ??? if escaped_byte == 0x7D || escaped_byte == 0x7E
                result.push(escaped_byte);
            }
        } else {
            result.push(byte);
        }
        i += 1;
    }
    result
}

/// Used to "pre-parse" a possible GDL90 message with binrw.
/// It reads until a [`GDL90_MAGIC`] byte is found, removing the CRC so its parsed and calculated after.
/// It returns the escaped result using [`remove_escapes`].
#[binrw::parser(reader, endian)]
fn parse_message_bytes() -> binrw::BinResult<Vec<u8>> {
    // TODO: confirm that this 0x7e is not present at next message
    let mut bytes: Vec<u8> =
        binrw::helpers::until_exclusive(|&b| b == GDL90_MAGIC)(reader, endian, ())?;
    // remove check seq
    bytes.truncate(bytes.len().saturating_sub(2));
    reader.seek(std::io::SeekFrom::Current(-3))?; // -3 because exclusive byte counts?
    Ok(remove_escapes(bytes))
}

#[cfg(test)]
mod tests {
    use types::report::{Altitude, Cord};

    use super::*;
    use std::io::Cursor;

    /* HEARTBEAT */

    #[test]
    fn msg_heartbeat() {
        let data = b"\x7E\x00\x81\x41\xDB\xD0\x08\x02\xB3\x8B\x7E";
        let parsed = Gdl90Message::read(&mut Cursor::new(data)).unwrap();
        assert_eq!(parsed.frame_check_seq, 0x8bb3);
        match parsed.message_data {
            Gdl90DatalinkMessage::Heartbeat { .. } => assert!(true),
            _ => assert!(false),
        }
        dbg!(parsed);
    }

    #[test]
    fn msg_heartbeat_invalid_crc() {
        let data = b"\x7E\x00\x81\x41\xDB\xD0\x08\x02\xFF\xFF\x7E";
        assert!(Gdl90Message::read(&mut Cursor::new(data)).is_err());
    }

    /* OWNSHIP REPORT */

    /// Wrapper to test ownship report messages.
    fn assert_ownship_report(
        data: &[u8],
        expected_seq: u16,
        lat_range: (f32, f32),
        long_range: (f32, f32),
        expected_altitude: Altitude,
    ) {
        let parsed = Gdl90Message::read(&mut Cursor::new(data)).unwrap();
        assert_eq!(parsed.frame_check_seq, expected_seq);

        if let Gdl90DatalinkMessage::OwnshipReport { report } = parsed.message_data {
            assert!(report.latitude() >= lat_range.0 && report.latitude() <= lat_range.1);
            assert!(report.longitude() >= long_range.0 && report.longitude() <= long_range.1);
            assert_eq!(report.altitude(), expected_altitude);
        } else {
            panic!("Expected OwnshipReport message");
        }
    }

    #[test]
    fn ownship_1() {
        assert_ownship_report(
            b"\x7E\x0A\x00\x00\x00\x00\x15\xA7\xE5\xBA\x47\x99\x08\xC9\x88\xFF\xE0\x00\x80\x01\x4E\x31\x32\x33\x34\x35\x20\x20\x00\x7B\xE5\x7E",
            0xe57b,
            (30.0, 31.0),
            (-99.0, -98.0),
            Altitude::Valid(2500),
        );
    }

    #[test]
    fn ownship_2() {
        assert_ownship_report(
            b"\x7E\x0A\x00\x00\x00\x00\x18\x7D\x5D\xF5\xBD\x1F\xB4\x09\x49\x88\x27\x40\x00\x82\x01\x4E\x31\x32\x33\x34\x35\x20\x20\x00\x8C\xEB\x7E",
            0xeb8c,
            (34.0, 35.0),
            (-95.0, -93.0),
            Altitude::Valid(2700),
        );
    }

    #[test]
    fn ownship_3() {
        assert_ownship_report(
            b"\x7E\x0A\x01\xF0\x00\x00\x1C\x25\xE6\xB5\x0F\xF2\x16\x09\x8A\x00\x08\x00\x42\x01\x53\x74\x72\x61\x74\x75\x78\x00\x00\xDB\xF6\x7E",
            0xf6db,
            (39.0, 40.0),
            (-106.0, -105.0),
            Altitude::Valid(7800),
        );
    }

    /* TRAFIC REPORT */

    #[test]
    fn traffic_1() {
        let data: Vec<u8> = vec![
            0x7E, // start
            0x14, // message id
            0x00, // st
            0x00, // aa
            0x00, // aa
            0x00, // aa
            0x18, // ll
            0x7D, // ll
            0x5D, // ll
            0xF5, // nn
            0xBD, // nn
            0x1F, // nn
            0xB4, // dd
            0x09, // dm
            0x49, // ia
            0x88, // hh
            0x27, // hv
            0x40, // vv
            0x00, // tt
            0x82, // ee
            0x01, // cc
            0x4E, // cc
            0x31, // cc
            0x32, // cc
            0x33, // cc
            0x34, // cc
            0x35, // cc
            0x20, // cc
            0x20, // cc
            0x00, // px
            0x5E, 0x66, // crc
            0x7E,
        ];
        let parsed = Gdl90Message::read(&mut Cursor::new(data)).unwrap();
        assert_eq!(parsed.frame_check_seq, 0x665e);

        if let Gdl90DatalinkMessage::TrafficReport { report } = parsed.message_data {
            //assert_eq!(report.participant_address().to_string(), "0");
            assert!(report.latitude() >= 34.0 && report.latitude() <= 35.0);
            assert!(report.longitude() >= -95.0 && report.longitude() <= -93.0);
            assert_eq!(report.altitude(), Altitude::Valid(2700));
        } else {
            panic!("Expected OwnshipReport message");
        }
    }

    /* OwnshipGeoometricAltitude */

    #[test]
    fn ownship_geometric_altitude() {
        let parsed = read_raw(&[126, 11, 0, 202, 0, 12, 251, 136, 126]);
        assert!(parsed.is_ok());
    }

    #[test]
    fn aux() {
        let parsed = read_raw(&[
            0x7E, 0x00, 0x81, 0x41, 0xDB, 0xD0, 0x08, 0x02, 0xB3, 0x8B, 0x7E,
        ])
        .unwrap();
        assert_eq!(parsed.frame_check_seq, 0x8bb3);
        match parsed.message_data {
            Gdl90DatalinkMessage::Heartbeat { .. } => assert!(true),
            _ => assert!(false),
        }
    }
}
