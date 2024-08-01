//! GDL90 Initialization message.
//!
//! | Byte # | Name                 | Size  | Value                         |
//! |--------|----------------------|-------|-------------------------------|
//! | 1      |Message ID            | 1     | 2                             |
//! | 2      |Configuration Byte 1  | 1     | see [`ConfigurationByte1`]    |
//! | 3      |Configuration Byte 2  | 1     | see [`ConfigurationByte2`]    |
//! |        |Total length          | 3     |                               |
//!

use binrw::BinRead;
use modular_bitfield::{
    bitfield,
    prelude::{B4, B6},
};

/// The Init message is sent by the Display once per second.
#[derive(BinRead, Debug)]
#[br(little)]
pub struct InitializationMessage {
    pub configuration_byte_1: ConfigurationByte1,
    pub configuration_byte_2: ConfigurationByte2,
}

/// Initialization Configuration Byte 1.
///
/// | Bit | Description                        | Value | Meaning                              |
/// |-----|------------------------------------|-------|--------------------------------------|
/// | 7   | Reserved                           | -     | -                                    |
/// | 6   | Audio Test                         | 1     | Initiate audio test                  |
/// | 5   | Reserved                           | -     | -                                    |
/// | 4   | Reserved                           | -     | -                                    |
/// | 3   | Reserved                           | -     | -                                    |
/// | 2   | Reserved                           | -     | -                                    |
/// | 1   | Audio Inhibit                      | 1     | Suppress GDL 90 audio output         |
/// | 0   | CDTI OK                            | 1     | CDTI capability is operating         |
#[bitfield]
#[derive(BinRead, Debug)]
#[br(little)]
pub struct ConfigurationByte1 {
    pub cdti_ok: bool,
    pub audio_inhibit: bool,
    #[skip]
    reserved_2345: B4,
    pub audio_test: bool,
    #[skip]
    reserved_7: bool,
}

/// Initialization Configuration Byte 2.
///
/// | Bit | Description                        | Value | Meaning                              |
/// |-----|------------------------------------|-------|--------------------------------------|
/// | 7   | Reserved                           | -     | -                                    |
/// | 6   | Reserved                           | -     | -                                    |
/// | 5   | Reserved                           | -     | -                                    |
/// | 4   | Reserved                           | -     | -                                    |
/// | 3   | Reserved                           | -     | -                                    |
/// | 2   | Reserved                           | -     | -                                    |
/// | 1   | CSA Audio Disable                  | 1     | Disable GDL 90 audible traffic alerts|
/// | 0   | CSA Disable                        | 1     | Disable CSA traffic alerting         |
#[bitfield]
#[derive(BinRead, Debug)]
#[br(little)]
pub struct ConfigurationByte2 {
    pub csa_audio_disable: bool,
    pub csa_disable: bool,
    #[skip]
    reserved_234567: B6,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
