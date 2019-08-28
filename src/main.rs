#![feature(test)]

mod constants;
mod handshake;
mod router;
mod types;

use hjul::*;

use std::error::Error;
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use sodiumoxide;
use types::{Bind, KeyPair};

struct Test {}

impl Bind for Test {
    type Error = BindError;
    type Endpoint = SocketAddr;

    fn new() -> Test {
        Test {}
    }

    fn set_port(&self, port: u16) -> Result<(), Self::Error> {
        Ok(())
    }

    fn get_port(&self) -> Option<u16> {
        None
    }

    fn recv(&self, buf: &mut [u8]) -> Result<(usize, Self::Endpoint), Self::Error> {
        Ok((0, "127.0.0.1:8080".parse().unwrap()))
    }

    fn send(&self, buf: &[u8], dst: &Self::Endpoint) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug)]
enum BindError {}

impl Error for BindError {
    fn description(&self) -> &str {
        "Generic Bind Error"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for BindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not Possible")
    }
}

#[derive(Debug, Clone)]
struct PeerTimer {
    a: Timer,
    b: Timer,
}

fn main() {
    let runner = Runner::new(Duration::from_millis(100), 1000, 1024);

    // choose optimal crypto implementations for platform
    sodiumoxide::init().unwrap();
    {
        let router = router::Device::new(
            4,
            |t: &PeerTimer, data: bool, sent: bool| t.a.reset(Duration::from_millis(1000)),
            |t: &PeerTimer, data: bool, sent: bool| t.b.reset(Duration::from_millis(1000)),
            |t: &PeerTimer| println!("new key requested"),
        );

        let pt = PeerTimer {
            a: runner.timer(|| println!("timer-a fired for peer")),
            b: runner.timer(|| println!("timer-b fired for peer")),
        };

        let peer = router.new_peer(pt.clone());

        println!("{:?}", pt);
    }

    println!("joined");
}
