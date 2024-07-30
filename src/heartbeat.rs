use bitfield_struct::bitfield;

/// The Heartbeat message provides real-time indications
/// to the Display of the status and operation of the GDL 90.
#[bitfield(u64)]
#[derive(PartialEq)]
pub struct HeartbeatMessage {
    #[bits(8)]
    pub message_id: u8,

    #[bits(8)]
    pub status_byte_1: HeartbeatStatusByte1,

    #[bits(8)]
    pub status_byte_2: HeartbeatStatusByte2,

    /// Current time-of-day in whole seconds elapsed since UTC midnight (`0000Z`).
    /// This requires a 17-bit data field. The most significant bit (bit 16) is in [`HeartbeatStatusByte2::timestamp_msb`].
    #[bits(16)]
    pub uat_timestamp: u16,

    /// Number of UAT messages received by the GDL 90 during the previous second.
    #[bits(16)]
    pub message_counts: u16, // FIXME:

    #[bits(8)]
    __: u8,
}

impl HeartbeatMessage {
    pub fn parse(data: u64) -> HeartbeatMessage {
        HeartbeatMessage::from_bits(data)
    }
}

#[bitfield(u8)]
pub struct HeartbeatStatusByte1 {
    /// This bit is set to `true` in all Heartbeat messages.
    pub uat_initialized: bool,

    /// Set to `false` in equipment that complies with this version of the specification.
    pub reserved: bool,

    /// Set to the present state of the Receiving ATC Services indication in the transmitted ADS-B messages.
    pub ratcs: bool,

    /// Whether the GDL 90 needs maintenance to replace its internal GPS battery.
    pub gps_batt_low: bool,

    /// Whether the GDL 90 is transmitting ADS-B messages using a temporary self-assigned (“anonymous”) address.
    pub addr_type: bool,

    /// Whether the GDL 90 has set the `IDENT` indication in its transmitted ADS-B messages.
    pub ident: bool,

    /// Whether the GDL 90 has detected a problem and requires maintainence.
    pub maint_reqd: bool,

    /// Whether the GDL 90 has a valid position fix for ADS-B messages.
    pub gps_pos_valid: bool,
}

#[bitfield(u8)]
pub struct HeartbeatStatusByte2 {
    /// Whether the GDL 90 is using a valid UTC timing reference.
    pub utc_ok: bool,

    /// Set to `false` in equipment that complies with this version of the specification.
    #[bits(4)]
    pub reserved: u8, // auto-padded!

    /// When set to `true`, this bit indicates to the Display that the CSA
    // algorithm has been requested but is not available.
    pub csa_not_available: bool,

    /// When set to `true`, this bit acknowledges to the Display that the GDL 90
    /// Conflict Situational Awareness (CSA) algorithm has been requested.
    pub csa_requested: bool,

    /// Most Significant Bit of the [`HeartbeatMessage::uat_timestamp`], in seconds elapsed since UTC midnight (0000Z)
    pub timestamp_msb: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_messages() {
        let yeah = HeartbeatMessage::parse(123456);
        println!("{:?}", yeah);
    }
}
