use ahash::AHashMap;

use std::{
    hash::{Hash, Hasher},
    io,
};

// use rand::seq::SliceRandom;

use wyhash::WyHash;

const NUM_RESOURCES: usize = 2048;
const CLIENT_SIZE: usize = 100_000_000;

fn main() -> io::Result<()> {
    let servers: Vec<String> = (1..=NUM_RESOURCES)
        .map(|i| format!("server{}.theykk.net", i))
        .collect();
    dbg!(servers.len());
    let customers: Vec<String> = (1..=CLIENT_SIZE)
        .map(|i| format!("user-or-tenat-{}", i))
        .collect();

    let mut s: WyHash = WyHash::default();

    // servers.shuffle(&mut );

    // let mut selected_indices: AHashMap<String, i32> = AHashMap::new();

    // let mut s_stat: AHashMap<String, i32> = AHashMap::new();
    let scount: usize = servers.len();

    // Access pre-assigned servers for each customer
    for (_, customer) in customers.iter().enumerate() {
        // s.write_str(customer.as_str())

        customer.hash(&mut s);
        let hascik = s.finish();
        let part1 = (hascik & 0xFFFF) as usize;
        let part2 = ((hascik >> 16) & 0xFFFF) as usize;
        let part3 = ((hascik >> 32) & 0xFFFF) as usize;
        let part4 = ((hascik >> 48) & 0xFFFF) as usize;

        let sel_server = vec![
            servers[part4 % scount].clone(),
            servers[part2 % scount].clone(),
            servers[part3 % scount].clone(),
            servers[part1 % scount].clone(),
        ];

        if part4 % scount == part2 % scount
            && part2 % scount == part3 % scount
            && part3 % scount == part1 % scount
        {
            println!("BINGOOOO");
            println!("{}", sel_server.clone().concat());
        }
        // let val = s_stat.entry(sel_server.concat()).or_insert(0);
        // *val += 1;

        // for ssx in sel_server {
        //     let val3 = selected_indices.entry(ssx).or_insert(0);
        //     *val3 += 1;
        // }
    }

    // dbg!(selected_indices.clone());
    // let mut bb = 0;
    // for sx in selected_indices {
    //     bb += sx.1
    // }
    // dbg!(bb);
    // // dbg!(s_stat.clone());
    // for x in s_stat {
    //     if x.1 > 2 {
    //         println!("{} => {}", x.1, x.0)
    //     }
    // }
    Ok(())
}
