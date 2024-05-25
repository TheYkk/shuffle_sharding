use std::hash::{Hash, Hasher};

use ahash::{AHashMap, AHasher};
use rand::{seq::SliceRandom, SeedableRng};
// use rand::thread_rng;
use rand_chacha::ChaCha8Rng;
use wyhash::WyHash;

const NUM_RESOURCES: usize = 50;
const SHARD_SIZE: usize = 4;
const CLIENT_SIZE: usize = 100_000;

fn main() {
    let resources: Vec<String> = (1..=NUM_RESOURCES)
        .map(|i| format!("server{}.theykk.net", i))
        .collect();
    dbg!(resources.len());
    let users: Vec<String> = (1..=CLIENT_SIZE)
        .map(|i| format!("user-or-tenat-{}", i))
        .collect();
    let mut s_stat: AHashMap<String, i32> = AHashMap::new();
    let mut selected_indices: AHashMap<String, i32> = AHashMap::new();

    // Simulate requests from different clients
    for client_id in users {
        let shard = get_shard_for_client(&resources, SHARD_SIZE, client_id);
        for ssx in shard.clone() {
            let val3 = selected_indices.entry(ssx).or_insert(0);
            *val3 += 1;
        }

        let val = s_stat.entry(shard.concat()).or_insert(0);
        *val += 1;
        // println!("Client {}'s shard: {:?}", client_id, shard);
    }

    dbg!(selected_indices.clone());
    let mut bb = 0;
    for sx in selected_indices {
        bb += sx.1
    }
    dbg!(bb);

    for x in s_stat {
        if x.1 > 2 {
            println!("{} => {}", x.1, x.0)
        }
    }
}

/// Returns a shard of resources for a given client ID.
fn get_shard_for_client(resources: &[String], shard_size: usize, client_id: String) -> Vec<String> {
    let mut s = WyHash::with_seed(25);
    client_id.hash(&mut s);
    let hascik = s.finish();

    // Create a thread-local RNG
    let mut rng = ChaCha8Rng::seed_from_u64(hascik);

    // Shuffle resources using a seed derived from the client_id
    let mut shuffled_resources = resources.to_vec();
    shuffled_resources.shuffle(&mut rng);

    // Select the first `shard_size` resources from the shuffled list
    shuffled_resources
        .into_iter()
        .skip(resources.len() / 2)
        .take(shard_size)
        .collect()
}
