use tokio::sync::mpsc;
use std::time::{Duration, Instant};
use std::thread;
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 0..10 {
            if let Err(_) = tx.send(i).await {
                println!("receiver dropped");
                return;
            }
        }
    });

    while let Some(i) = rx.recv().await {
        println!("got = {}", i);
    }

    let op = say_hello();
    println!("hello");
    op.await;

    let handle = tokio::spawn(async{
        println!("run async work");
        std::thread::sleep(std::time::Duration::from_millis(100));
       return "sub thread done";
    });
    println!("main still running ... ");
    let out = handle.await.unwrap();
    println!("out: {}", out);

    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}

async fn say_hello() {
    println!("world!");
}

