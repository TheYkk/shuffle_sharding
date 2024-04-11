use ahash::{AHashMap, AHashSet};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// A simple shuffle sharding library
pub struct FastShards<'a> {
    customer_assignments: Vec<Vec<&'a String>>,
}

impl<'a> FastShards<'a> {
    /// Creates a new `FastShards` instance and assigns customers to servers `n` times.
    pub fn new(servers: &'a [String], customers: &'a [String], n: usize) -> Self {
        let mut rng = thread_rng();
        let mut customer_assignments = Vec::with_capacity(customers.len());

        for _ in 0..customers.len() {
            customer_assignments.push(Vec::with_capacity(n));
        }

        for (i, _) in customers.iter().enumerate() {
            // for _ in 0..n {
            customer_assignments[i].extend(
                servers
                    .choose_multiple(&mut rng, n)
                    .collect::<Vec<&String>>(),
            );
            // }
        }

        FastShards {
            customer_assignments,
        }
    }

    /// Returns the servers a given customer is assigned to.
    pub fn get_servers_for_customer(&self, customer_index: usize) -> Option<&Vec<&String>> {
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

    let n = 4; // Assign each customer to 4 servers
    let shards = FastShards::new(&servers, &customers, n);

    let mut s_stat: AHashMap<String, i32> = AHashMap::new();
    // Access pre-assigned servers for each customer
    for (i, customer) in customers.iter().enumerate() {
        if let Some(assigned_servers) = shards.get_servers_for_customer(i) {
            for s in assigned_servers {
                let val = s_stat.entry(s.to_string()).or_insert(0);
                *val += 1;
            }
            if has_duplicates(assigned_servers) {
                println!("{:?}", assigned_servers);
            }
        }
    }

    Ok(())
}

fn has_duplicates<T: Eq + std::hash::Hash>(vec: &[T]) -> bool {
    let mut set = AHashSet::new();
    for item in vec {
        if !set.insert(item) {
            return true; // Found a duplicate
        }
    }
    false // No duplicates found
}
