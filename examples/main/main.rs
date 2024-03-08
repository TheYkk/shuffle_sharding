use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// A simple shuffle sharding library
pub struct ShuffleShards {
    customer_to_servers: HashMap<String, HashSet<String>>,
}

impl ShuffleShards {
    /// Creates a new `ShuffleShards` instance and assigns customers to servers `n` times.
    pub fn new(servers: Vec<String>, customers: Vec<String>, n: usize) -> Self {
        let mut customer_to_servers = HashMap::new();
        let mut rng = thread_rng();

        for customer in customers {
            let mut assigned_servers = HashSet::with_capacity(n);
            let mut servers_copy = servers.clone();
            servers_copy.shuffle(&mut rng);

            for server in servers_copy.iter().take(n) {
                assigned_servers.insert(server.clone());
            }

            customer_to_servers.insert(customer, assigned_servers);
        }

        ShuffleShards {
            customer_to_servers,
        }
    }

    /// Returns the servers a given customer is assigned to.
    pub fn get_servers_for_customer(&self, customer: &str) -> Option<HashSet<String>> {
        self.customer_to_servers.get(customer).cloned()
    }
}

fn main() -> io::Result<()> {
    let servers_file = File::open("fqdns.txt")?;
    let customers_file = File::open("users.txt")?;

    let servers_reader = BufReader::new(servers_file);
    let customers_reader = BufReader::new(customers_file);

    let servers: Vec<String> = servers_reader.lines().flatten().collect();

    let customers: Vec<String> = customers_reader.lines().flatten().collect();

    let n = 2; // Assign each customer to 3 servers
    let shards = ShuffleShards::new(servers, customers.clone(), n);

    for customer in customers {
        if let Some(servers) = shards.get_servers_for_customer(customer.as_str()) {
            println!(
                "Customer '{}' is assigned to servers: {:?}",
                customer, servers
            );
        }
    }

    Ok(())
}
