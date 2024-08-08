use binrw::{binwrite, BinWrite};

pub struct Gdl90ControlMessage {}

impl Gdl90ControlMessage {}

pub enum Gdl90ControlMessageType {
    CallSign,
    OperationMode,
    VFR,
}

#[binwrite]
#[bw(little, magic = b"^CS ")]
struct CallSignMessage {
    pub call_sign: [u8; 8],

    pub checksum: [u8; 2],

    #[bw(calc(b'\r'))]
    carriage: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn control() {
        let object = CallSignMessage {
            call_sign: "Maverick".as_bytes().try_into().unwrap(),
            checksum: [0, 0],
        };
        let mut output = Cursor::new(vec![]);
        object.write(&mut output);
        println!("{output:?}");
        let s = std::str::from_utf8(output.get_ref());
        println!("{s:?}");
    }
}
