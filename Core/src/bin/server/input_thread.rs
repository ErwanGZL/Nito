use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use nito::{Cell, Element, Simulation, Vector2D};

struct CellRead {
    x: u16,
    y: u16,
    value: u8,
}

fn read_exact_bytes(stream: &mut TcpStream, size: usize) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = vec![0; size];
    match stream.read_exact(buffer.as_mut_slice()) {
        Ok(_) => Ok(buffer),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                Ok(buffer)
            } else {
                Err(e)
            }
        }
    }
}

pub fn handle_connection(
    sim: Arc<Mutex<Simulation>>,
    mut stream: TcpStream,
) -> Result<(), std::io::Error> {
    loop {
        let header = read_exact_bytes(&mut stream, 2)?;
        if header.len() == 0 {
            break;
        }
        let cell_no = u16::from_le_bytes([header[0], header[1]]);
        let body = read_exact_bytes(&mut stream, cell_no as usize * 5)?;
        if body.len() == 0 {
            break;
        }

        {
            let mut sim = sim.lock().unwrap();
            for i in (0..(cell_no as usize * 5)).step_by(5) {
                let cell = CellRead {
                    x: u16::from_le_bytes([body[i], body[i + 1]]),
                    y: u16::from_le_bytes([body[i + 2], body[i + 3]]),
                    value: body[i + 4],
                };
                // Todo: Optimize this match
                if cell.value != 0
                    && sim.world[cell.y as usize][cell.x as usize].element() != Element::Air
                {
                    continue;
                }
                match cell.value {
                    0 => {
                        sim.world[cell.y as usize][cell.x as usize] = Cell::new(Element::Air);
                    }
                    1 => {
                        sim.world[cell.y as usize][cell.x as usize] = Cell::new(Element::Water);
                    }
                    2 => {
                        sim.world[cell.y as usize][cell.x as usize] = Cell::new(Element::Sand);
                    }
                    3 => {
                        sim.world[cell.y as usize][cell.x as usize] = Cell::new(Element::Wood);
                    }
                    _ => {}
                }
                sim.world[cell.y as usize][cell.x as usize].set_update();
            }
        }
    }
    println!("Client disconnected");
    Ok(())
}
