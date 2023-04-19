use std::time::Duration;

use futures::{
    future::join_all,
    stream::{FuturesUnordered, StreamExt},
};
use tokio::time::sleep;

async fn process_item(item: i32) {
    println!("{item:0>2}: start");
    sleep(Duration::from_millis(item as u64 * 75)).await;
    println!("    {item:0>2}: done");
}

async fn process_queue(queue: Vec<i32>, concurrency: usize) {
    let mut futures = FuturesUnordered::new();
    for item in queue {
        futures.push(Box::pin(process_item(item)));
        if futures.len() >= concurrency {
            futures.next().await;
        }
    }
    join_all(futures.into_iter()).await;
}

#[tokio::main]
async fn main() {
    let queue = (1..=16).collect();
    let concurrency = 5;
    process_queue(queue, concurrency).await;
}
