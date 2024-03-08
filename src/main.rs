use dashmap::{DashMap, DashSet};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader}; // 1. Import ahash

/// A simple shuffle sharding library
pub struct FastShards {
    customer_assignments: Vec<Vec<String>>,
}

impl FastShards {
    /// Creates a new `FastShards` instance and assigns customers to servers `n` times.
    pub fn new(servers: Vec<String>, customers: &[String], n: usize) -> Self {
        let mut rng = thread_rng();
        let mut customer_assignments = Vec::with_capacity(customers.len());

        // Pre-shuffle servers for faster assignment
        let mut shuffled_servers = servers.clone();
        shuffled_servers.shuffle(&mut rng);

        for _ in 0..customers.len() {
            customer_assignments.push(Vec::with_capacity(n));
        }
        let mut shard_counter = 0;
        for (i, _) in customers.iter().enumerate() {
            for _ in 0..n {
                let server_index = shard_counter;

                customer_assignments[i].push(shuffled_servers.get(server_index).unwrap().clone());
                shard_counter += 1;
                if shard_counter >= shuffled_servers.len() {
                    shard_counter = 0;
                    shuffled_servers.shuffle(&mut rng);
                }
            }
        }

        FastShards {
            customer_assignments,
        }
    }

    /// Returns the servers a given customer is assigned to.
    pub fn get_servers_for_customer(&self, customer_index: usize) -> Option<&Vec<String>> {
        self.customer_assignments.get(customer_index)
    }
}

fn main() -> io::Result<()> {
    let servers_file = File::open("fqdns.txt")?;
    let customers_file = File::open("users.txt")?;

    let servers_reader = BufReader::new(servers_file);
    let customers_reader = BufReader::new(customers_file);

    // Read servers and customers
    let servers: Vec<String> = servers_reader.lines().flatten().collect();
    let customers: Vec<String> = customers_reader.lines().flatten().collect();

    let n = 4; // Assign each customer to 3 servers
    let shards = FastShards::new(servers, &customers, n);

    let s_stat: DashMap<String, i32> = DashMap::new();
    // Access pre-assigned servers for each customer
    for (i, customer) in customers.iter().enumerate() {
        if let Some(assigned_servers) = shards.get_servers_for_customer(i) {
            for s in assigned_servers {
                let mut val = s_stat.entry(s.to_string()).or_insert(0);
                *val += 1;
            }
            if has_duplicates(assigned_servers) {
                println!("{:?}", assigned_servers);
                // panic!()
            }
            // println!(
            //     "Customer '{}' is assigned to servers: {:?}",
            //     customer, assigned_servers
            // );
        }
    }
    // let sorted_map = sort_hashmap_by_value(&s_stat);

    // for (key, value) in sorted_map {
    //     println!("{}: {}", key, value);
    // }
    Ok(())
}

// fn sort_hashmap_by_value(hashmap: &DashMap<String, i32>) -> Vec<(&String, &i32)> {
//     let mut sorted_pairs: Vec<_> = hashmap.iter().collect();
//     sorted_pairs.sort_by(|a, b| a.cmp(b)); // Sort by value

//     sorted_pairs.
// }
fn has_duplicates<T: Eq + std::hash::Hash>(vec: &[T]) -> bool {
    let set = DashSet::new();
    for item in vec {
        if !set.insert(item) {
            return true; // Found a duplicate
        }
    }
    false // No duplicates found
}
