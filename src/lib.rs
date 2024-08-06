//! A Rust crate for GDL90 message handling.
//!
//! Please read GDL90 especification [here](https://www.faa.gov/sites/faa.gov/files/air_traffic/technology/adsb/archival/GDL90_Public_ICD_RevA.PDF).
//! But in short:
//!
//! | 1 byte (0x7E)    | 1 byte (...) | N bytes      | 2 bytes              | 1 byte (0x7E) |
//! |------------------|--------------|--------------|----------------------|---------------|
//! | Start Flag byte  | Message ID   | Message Data | Frame Check Sequence | End Flag byte |
//!
//! # Usage
//!
//! ```
//! use gdl90::Gdl90Message;
//! use gdl90::message::Gdl90MessageType;
//! use binrw::BinRead;
//! use std::io::Cursor;
//!
//! let mut gdl90_heartbeat = Cursor::new(b"\x7E\x00\x81\x41\xDB\xD0\x08\x02\xB3\x8B\x7E");
//! let parsed = Gdl90Message::read(&mut gdl90_heartbeat).unwrap();
//! assert_eq!(parsed.frame_check_seq, 0x8bb3);
//! match parsed.message_data {
//!     Gdl90MessageType::Heartbeat(ref hb) => {}
//!     // ...
//!     _ => assert!(false),
//! }
//! ```
//!
//! Under the hood, this parser uses crates like [`modular-bitfield`] and [`binrw`] to parse message fields.
//!
//! Note: Work in progress, feel free to contribute.

pub mod message;

use std::io::Cursor;

use binrw::{binread, BinRead, BinResult};
use message::Gdl90MessageType;

fn remove_escapes(data: Vec<u8>) -> Vec<u8> {
    data
}

#[binrw::parser(reader, endian)]
fn parse_message_bytes() -> binrw::BinResult<Vec<u8>> {
    // TODO: confirm that this 0x7e is not present at next message
    let mut bytes: Vec<u8> = binrw::helpers::until_exclusive(|&b| b == 0x7e)(reader, endian, ())?;

    // remove check seq
    bytes.truncate(bytes.len().saturating_sub(2));
    reader.seek(std::io::SeekFrom::Current(-3))?; // -3 because exclusive byte counts?

    Ok(remove_escapes(bytes))
}

pub struct Gdl90ControlMessage {}

impl Gdl90ControlMessage {}

pub enum Gdl90ControlMessageType {
    CallSign,
    OperationMode,
    VFR,
}

#[binread]
#[derive(Debug)]
#[br(little, magic = b"\x7E")]
pub struct Gdl90Message {
    #[br(temp, parse_with = parse_message_bytes)]
    data: Vec<u8>,

    #[br(map_stream = |_| Cursor::new(&data))]
    pub message_data: Gdl90MessageType,

    /// assert here
    pub frame_check_seq: u16,
    //#[br(temp, assert(flag_byte_end == 0x7E))]
    //pub flag_byte_end: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn msg_heartbeat() {
        let mut data = Cursor::new(b"\x7E\x00\x81\x41\xDB\xD0\x08\x02\xB3\x8B\x7E");
        let parsed = Gdl90Message::read(&mut data).unwrap();
        //dbg!(&parsed);
        assert_eq!(parsed.frame_check_seq, 0x8bb3);
        match parsed.message_data {
            Gdl90MessageType::Heartbeat(ref hb) => {}
            _ => assert!(false),
        }
    }
}
