use ahash::RandomState;
use core::hash;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use dashmap::{DashMap, DashSet};
use std::sync::Arc;
fn main() -> io::Result<()> {
    let servers: Arc<DashSet<String>> = Arc::new(DashSet::new());
    let users: Arc<DashSet<String>> = Arc::new(DashSet::new());

    let file = File::open("fqdns_5.txt")?;
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        // Unwrap the Result to handle any errors
        servers.insert(line?);
    }

    // Read users
    let file = File::open("users.txt")?;
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        // Unwrap the Result to handle any errors
        users.insert(line?);
    }

    users.iter().for_each(|u| {
      hash
    })
    Ok(())
}
