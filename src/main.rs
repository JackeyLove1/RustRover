use std::collections::HashMap;
use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use serde_json::{json, to_string};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;


type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8088").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));

    let (tx, mut rx) = mpsc::channel(32);
    let tx2= tx.clone();
    tokio::spawn(async move {
        tx.send("sending from first handle").await.expect("failed to send");
    });
    tokio::spawn(async move {
        tx2.send("sending from second handle").await.expect("failed to send");
    });
    while let Some(message) = rx.recv().await {
        println!("Got: {}", message);
    }

    loop {
        println!("Server Running .. . ");
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        println!("Accepted connection from: {}", socket.peer_addr().unwrap());
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}


async fn process(socket: TcpStream, db: Db){
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
        
    }

}

#[derive(Debug)]
enum Command {
    Get{
        key: String,
    },
    Set {
        key: String,
        value: Bytes,
    }
}