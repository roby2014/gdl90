mod heartbeat;

#[derive(Debug, PartialEq)]
pub enum Gdl90Message {
    Heartbeat(heartbeat::HeartbeatMessage),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
