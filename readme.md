Note: Work in progress, feel free to contribute.

# gdl90

[![WTE-MT-RX Parser on crates.io][cratesio-image]][cratesio]
[![WTE-MT-RX Parser on docs.rs][docsrs-image]][docsrs]
[![GitHub last commit][ghcommit-image]][ghcommit]

[cratesio-image]: https://img.shields.io/crates/v/gdl90.svg
[cratesio]: https://crates.io/crates/gdl90
[docsrs-image]: https://docs.rs/gdl90/badge.svg
[docsrs]: https://docs.rs/gdl90
[ghcommit-image]: https://img.shields.io/github/last-commit/roby2014/gdl90
[ghcommit]: https://github.com/roby2014/gdl90/

This crate provides types and structures for handling [GDL90 messages](https://www.faa.gov/sites/faa.gov/files/air_traffic/technology/adsb/archival/GDL90_Public_ICD_RevA.PDF).

It uses Rust crates such as [binrw](https://github.com/jam1garner/binrw) and [modular-bitfield](https://github.com/Robbepop/modular-bitfield) to efficiently
represent and manipulate these message types.

## What is GDL90?

> GDL 90 is designed to transmit, receive and decode Automatic Dependent Surveillance-Broadcast (ADS-B)
messages via onboard datalink. It broadcasts your aircraft's position, velocity, projected track, altitude
and flight identification to other equipped aircraft in your vicinity, as well as to ground-based transceivers maintained by the FAA.

Many ADS-B transponders (such as [uAvionix ping20Si](https://uavionix.com/products/ping20si/)) use this protocol for their I/O interfaces. With this crate, you can easily integrate one of those in your *rusty* system.

You can find the full specification [here](https://www.faa.gov/sites/faa.gov/files/air_traffic/technology/adsb/archival/GDL90_Public_ICD_RevA.PDF).

## Usage

```toml
[dependencies]
gdl90 = "0.1.0"
```

```rs
use gdl90::Gdl90Message;
use gdl90::message::Gdl90MessageType;
use binrw::BinRead;
use std::io::Cursor;

// example with a hardcoded heartbeat message
let mut gdl90_heartbeat = Cursor::new(b"\x7E\x00\x81\x41\xDB\xD0\x08\x02\xB3\x8B\x7E");
let parsed = Gdl90Message::read(&mut gdl90_heartbeat).unwrap();
dbg!(&parsed);

// handle message type as you want...
match parsed.message_data {
    Gdl90MessageType::Heartbeat(ref hb) => {}
    _ => assert!(false),
}
```

Result:
```
&parsed = Gdl90Message {
    message_data: Heartbeat(
        HeartbeatMessage {
            status_byte_1: HeartbeatStatusByte1 {
                uat_initialized: true,
                ratcs: false,
                gps_batt_low: false,
                addr_type: false,
                ident: false
                maint_reqd: false,
                gps_pos_valid: true,
            },
            status_byte_2: HeartbeatStatusByte2 {
                utc_ok: true,
                csa_not_available: false,
                csa_requested: true,
                timestamp_msb: false,
            },
            uat_timestamp: 53467,
            message_counts: 520,
        },
    ),
    frame_check_seq: 35763,
    flag_byte_end: 126,
}
```

## What is this...

I am not sure if this is considered a parser, decoder or deserializer.

## TODO:

- Add more strong typying structures, no raw bits like `B4`, ...
- Add remaining messages.
- Try removing `bitfield` as maximum as possible, replacing it with custom types, using `binrw` directives and magic macros.

## References

- [binrw](https://binrw.rs/)
- [modular-bitfield](https://github.com/Robbepop/modular-bitfield)
- [GDL90 spec](https://www.faa.gov/sites/faa.gov/files/air_traffic/technology/adsb/archival/GDL90_Public_ICD_RevA.PDF).
