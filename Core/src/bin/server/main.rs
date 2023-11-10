use mpsc::channel;
use serde::de::Expected;
use std::time::Instant;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use nito::{config::open_config, simulation::Simulation};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        let data = stream.read(&mut buffer);
        match data {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                println!("Received: {:?}", &buffer[..size]);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    println!("Client disconnected");
}

fn sim_logic(
    mut sim: Simulation,
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
            *dump = sim.dump();
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
    let mut sim = Simulation::new(cfg.world.x as usize, cfg.world.y as usize);
    sim.world[0][5] = nito::simulation::Element::Water;

    let addr = format!("{}:{}", cfg.endpoint.address, cfg.endpoint.port);
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");
    println!("Server running and listening on port {}", cfg.endpoint.port);

    let mut handles = vec![];
    let clients = Arc::new(Mutex::new(Vec::new()));
    let dump = Arc::new(Mutex::new(Vec::new()));
    let (sender, receiver) = channel();

    // Simulation thread
    let dump_clone = Arc::clone(&dump);
    let handle = thread::spawn(move || {
        sim_logic(sim, dump_clone, sender, 1.0 / cfg.world.frequency as f64);
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
                let handle = thread::spawn(|| {
                    handle_connection(stream);
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
