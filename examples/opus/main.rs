use rand::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const NUM_SERVERS: usize = 5000;
const NUM_CUSTOMERS: usize = 5000;

fn main() -> io::Result<()> {
    let servers_file = File::open("fqdns.txt")?;
    let customers_file = File::open("users.txt")?;

    let servers_reader = BufReader::new(servers_file);
    let customers_reader = BufReader::new(customers_file);

    let mut servers = Vec::with_capacity(NUM_SERVERS);
    let mut customers = Vec::with_capacity(NUM_CUSTOMERS);

    for line in servers_reader.lines().flatten() {
        servers.push(line);
    }

    for line in customers_reader.lines().flatten() {
        customers.push(line);
    }

    let n = 2; // Assign each customer to 2 servers
    let mut rng = thread_rng();

    let mut customer_to_servers = Vec::with_capacity(NUM_CUSTOMERS);

    for customer in &customers {
        let mut assigned_servers = Vec::with_capacity(n);
        let mut indices = (0..NUM_SERVERS).collect::<Vec<_>>();
        indices.shuffle(&mut rng);

        for &index in indices.iter().take(n) {
            assigned_servers.push(servers[index].as_str());
        }

        customer_to_servers.push((customer.as_str(), assigned_servers));
    }

    for (customer, assigned_servers) in &customer_to_servers {
        println!(
            "Customer '{}' is assigned to servers: {:?}",
            customer, assigned_servers
        );
    }

    Ok(())
}
