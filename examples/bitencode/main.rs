use std::{
    fs::File,
    io::{self, Write},
};

use bitcode::{Decode, Encode};

const NUM_RESOURCES: usize = 2048;
const CLIENT_SIZE: usize = 5_000_000;

#[derive(Encode, Decode, PartialEq, Debug)]
struct CustomerServers {
    servers: Vec<String>,
    customers: Vec<String>,
}
fn main() -> io::Result<()> {
    let cs = CustomerServers {
        servers: (1..=NUM_RESOURCES)
            .map(|i| format!("server{}.theykk.net", i))
            .collect(),
        customers: (1..=CLIENT_SIZE)
            .map(|i| format!("user-{}-or-tenat", i))
            .collect(),
    };

    let encodedd: Vec<u8> = bitcode::encode(&cs); // No error
    File::create("cs.bin")
        .unwrap()
        .write_all(&encodedd)
        .unwrap();

    Ok(())
}
