use std::net;

use nito::config::open_config;
use nito::simulation::{Element, Simulation};

fn main() {
    let cfg = open_config();
    let mut remote =
        net::TcpStream::connect(format!("{}:{}", cfg.remote.address, cfg.remote.port)).unwrap();
    let mut sim = Simulation::new(cfg.world.x as usize, cfg.world.y as usize);
    sim.world[0][5] = Element::Water;
    println!("{}", sim);
    println!("Header: {:02X?}", &sim.dump()[0..8]);
    println!("Body: {:02X?}", &sim.dump()[8..]);
}
