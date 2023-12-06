# Nito

Sandbox cellular automata simulation

## Building and running

The simulation server is written in rust and therefore is cross-platform. The graphical frontend is written in C++ and
uses SFML, so it is also cross-platform.

Requires Rust and Cargo.

```console
cargo build --release
cargo run
```

The client is written in C++ and uses SFML. It is unix-only because of utilization of Unix sockets.

Requires SFML and CMake.

```console
cmake -S . -B build
cmake --build build
./build/nito
```

## Features:

### Technical

- Simulation is implemented on a Rust backend
- Graphical frontend is implemented in C++ using SFML
- Multithreaded frontend and backend
- Flexible frontend window dimensions
- Simulation can vary in size and speed

### Elements

- Water: falls and spreads
- Sand: falls and piles up
- Wood: stays in place, burns into embers
- Fire: flickers and decay
- Smoke: rises and decays, generated from burning embers
- Embers: generates fire and smoke, decays, turns into charcoal upon contact with water
- Charcoal: stays in place, burns into embers

## Authors

- [Erwan Gonzales](https://github.com/EstusSipper)
- [Clovis Rabot](https://github.com/rclovis)