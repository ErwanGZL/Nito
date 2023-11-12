use mpsc::channel;
use nito::simulation;
use serde::de::Expected;
use std::time::Instant;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

struct NewCell {
    x: u16,
    y: u16,
    value: u8,
}

use nito::{config::open_config, simulation::Simulation};

fn handle_connection(
    sim: Arc<Mutex<Simulation>>,
    mut stream: TcpStream,
) {
    let mut buffer: [u8; 2048] = [0; 2048];
    let mut size: [u8; 2] = [0; 2];
    let mut cell = NewCell {
        x: 0,
        y: 0,
        value: 0,
    };
    loop {
        let data = stream.read(&mut size);
        match data {
            Ok(size_read) => {
                if size_read == 0 {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        let body_size = size[0] as u16 + ((size[1] as u16) << 8);
        if body_size == 0 {
            break;
        }
        let data = stream.read(&mut buffer[0..(body_size * 5) as usize]);
        match data {
            Ok(size_read) => {
                if size_read == 0 {
                    break;
                }
                for i in (0..(body_size * 5) as usize).step_by(5) {
                    cell.x = buffer[i + 0] as u16 + ((buffer[i + 1] as u16) << 8);
                    cell.y = buffer[i + 2] as u16 + ((buffer[i + 3] as u16) << 8);
                    cell.value = buffer[i + 4];
                    match cell.value {
                        0 => {
                            sim.lock().unwrap().world[cell.y as usize][cell.x as usize] =
                                nito::simulation::Element::Air;
                        }
                        1 => {
                            sim.lock().unwrap().world[cell.y as usize][cell.x as usize] =
                                nito::simulation::Element::Water;
                        }
                        2 => {
                            sim.lock().unwrap().world[cell.y as usize][cell.x as usize] =
                                nito::simulation::Element::Sand;
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                eprintln!("Error_: {}", e);
            }
        }
    }
    println!("Client disconnected");
}

fn sim_logic(
    sim: Arc<Mutex<Simulation>>,
    dump: Arc<Mutex<Vec<u8>>>,
    sender: mpsc::Sender<bool>,
    frequency: f64,
) {
    println!("Simulation thread started!");
    let d = Duration::from_secs_f64(frequency);
    let mut last_frame_time = Instant::now();
    loop {
        {
            let mut dump = dump.lock().unwrap();
            *dump = sim.lock().unwrap().dump();
            sender.send(true).unwrap();
        }
        // Todo: read from inbox and update the sim
        // Todo: sim.update();

        let now = Instant::now();
        let elapsed_time = now.duration_since(last_frame_time);
        if elapsed_time < d {
            thread::sleep(d - elapsed_time);
        }
        last_frame_time = Instant::now();
    }
}

fn dispatcher_logic(
    receiver: mpsc::Receiver<bool>,
    dump: Arc<Mutex<Vec<u8>>>,
    clients: Arc<Mutex<Vec<TcpStream>>>,
) {
    println!("Dispatcher thread started!");
    let mut disconnects = vec![];
    loop {
        let msg = receiver.recv().unwrap();
        if msg {
            let mut clients = clients.lock().unwrap();
            let dump = dump.lock().unwrap();
            let mut i = 0;
            for con in clients.iter_mut() {
                match con.write(dump.as_slice()) {
                    Ok(_) => {
                    }
                    Err(_) => {
                        disconnects.push(i);
                    }
                }
                i += 1;
            }
            for i in disconnects.iter().rev() {
                clients.remove(*i);
            }
            disconnects.clear();
        }
    }
}

fn main() -> std::io::Result<()> {
    let cfg = open_config();
    // let mut sim: Simulation = Simulation::new(cfg.world.x as usize, cfg.world.y as usize);
    let sim: Arc<Mutex<Simulation>> = Arc::new(Mutex::new(Simulation::new(
        cfg.world.x as usize,
        cfg.world.y as usize,
    )));

    // sim.lock().unwrap().world[0][5] = nito::simulation::Element::Water;


    let addr = format!("{}:{}", cfg.endpoint.address, cfg.endpoint.port);
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");
    println!("Server running and listening on port {}", cfg.endpoint.port);

    let mut handles = vec![];
    let clients = Arc::new(Mutex::new(Vec::new()));
    let dump = Arc::new(Mutex::new(Vec::new()));
    let (sender, receiver) = channel();

    // Simulation thread
    let dump_clone = Arc::clone(&dump);
    let sim_clone = Arc::clone(&sim);
    let handle = thread::spawn(move || {
        sim_logic(sim_clone, dump_clone, sender, 1.0 / cfg.world.frequency as f64);
    });
    handles.push(handle);

    // Dispatcher thread
    let dump_clone = Arc::clone(&dump);
    let clients_clone = Arc::clone(&clients);
    let handle = thread::spawn(move || {
        dispatcher_logic(receiver, dump_clone, clients_clone);
    });
    handles.push(handle);

    // Listen for incoming connection
    for stream in listener.incoming() {
        let clients = Arc::clone(&clients);

        match stream {
            Ok(stream) => {
                println!("Received new connection");
                // Add client to clients vector
                let mut clients = clients.lock().unwrap();
                clients.push(stream.try_clone().expect("Failed to clone connection"));

                // On new connection spawn a thread
                let clone_stream = stream.try_clone().expect("Failed to clone connection");
                let sim_clone = Arc::clone(&sim);
                let handle = thread::spawn(move || {
                    handle_connection(sim_clone, clone_stream);
                });
                handles.push(handle);
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        };
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
