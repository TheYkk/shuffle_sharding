use std::time::Duration;

use tokio::time::{self, sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!(" First {:?}", std::time::Instant::now());
    let b = alo();
    // println!(" second {:?}", std::time::Instant::now());
    sleep(Duration::from_millis(1000)).await;
    // println!(" Third {:?}", std::time::Instant::now());

    println!("Hello world");
    b.await;
    // println!(" Last {:?}", std::time::Instant::now());
    Ok(())
}

async fn alo() {
    println!("Hello abc");
    // println!("{:?}", std::time::Instant::now());
    sleep(Duration::from_millis(100)).await;
    println!("Hello dbc");
}
