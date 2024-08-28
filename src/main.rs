use rand::rngs::StdRng;
use std::{
    fs::File,
    hash::{Hash, Hasher},
    io::{self, Write},
    sync::Arc,
};

// use rand::seq::SliceRandom;

use ahash::{AHashMap, AHasher};
// use ahash::AHashMap;
use rand::{seq::SliceRandom, RngCore, SeedableRng};
use wyhash::{WyHash, WyRng};

const NUM_RESOURCES: usize = 50;
const CLIENT_SIZE: usize = 100_000;
const SHARD_NODE_COUNT: usize = 4;
fn main() -> io::Result<()> {
    let mut servers: Vec<String> = (1..=NUM_RESOURCES)
        .map(|i| format!("server{}.theykk.net", i))
        .collect();

    dbg!(servers.len());

    let customers: Vec<String> = (1..=CLIENT_SIZE)
        .map(|i| format!("user-{}-or-tenat", i))
        .collect();

    // let mut customers: Vec<String> = Vec::with_capacity(CLIENT_SIZE);
    // customers.extend((1..=CLIENT_SIZE).map(|i| format!("user-{}-or-tenat", i)));

    let encodedd: Vec<u8> = bitcode::encode(&customers.clone()); // No error
    File::create("twoenc.bin")
        .unwrap()
        .write_all(&encodedd)
        .unwrap();
    let mut s: AHasher = AHasher::default();

    let scount: u64 = servers.len().try_into().unwrap();

    let mut selected_indices: AHashMap<String, u32> = AHashMap::with_capacity(scount as usize);

    // Access pre-assigned servers for each customer
    for (_, customer) in customers.iter().enumerate() {
        customer.hash(&mut s);
        let hascik: u64 = s.finish();
        // let mut chash: WyRng = R::seed_from_u64(hascik);
        let mut rng = StdRng::seed_from_u64(hascik);
        // let mut cservers = servers.clone();
        servers.shuffle(&mut rng);

        let mut sel_server: Vec<String> = Vec::with_capacity(SHARD_NODE_COUNT);

        &servers
            .iter()
            .take(SHARD_NODE_COUNT)
            .for_each(|s| sel_server.push(s.to_string()));

        // for _ in 0..SHARD_NODE_COUNT {
        //   sel_server.push(servers)

        let all_equal = sel_server.iter().all(|x| *x == sel_server[0]);

        if all_equal {
            println!("BINGOOOO");
            println!("{}", sel_server.clone().concat());
        }

        for ssx in sel_server {
            let val3 = selected_indices.entry(ssx).or_insert(0);
            *val3 += 1;
        }
    }

    // Convert the AHashMap to a Vec of tuples before encoding
    let selected_indices_vec: Vec<(String, u32)> = selected_indices.into_iter().collect();

    Ok(())
}
