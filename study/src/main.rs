#[derive(Debug)]
enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil,
}

fn box_list_print(){
    use crate::BoxList::{Cons, Nil};
    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    println!("box list: {:?}", list)
}

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::iter::Map;

#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn rc_list_print(){
    use crate::RcList::{Cons, Nil};
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("rc a: {:?}", a);
    let b = Rc::new(Cons(4, Rc::clone(&a)));
    println!("rc b: {:?}", b);
    println!("reference of a: {}", Rc::strong_count(&a));
}

fn mut_shared_pointer(){
    let value = Rc::new(RefCell::new(0));
    *value.borrow_mut() += 1;
    println!("value: {}", *value.borrow());
}


#[derive(Debug)]
struct Node {
    value : i32,
    children: RefCell<Weak<Node>>,
    parent: RefCell<Vec<Rc<Node>>>,
}

use std::thread;
use std::time::Duration;
fn thread_print(){
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for (index, &value) in v.iter().enumerate(){
            println!("hi number {} from the spawned thread!", value);
        }
    });
    handle.join().unwrap();
}

use std::sync::{mpsc, Mutex};
fn message_queue(){
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
       let val = String::from("Hello, World!");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

use std::sync::Arc;
fn mutex_counter(){
    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let thread = thread::spawn(move ||{
           let mut num = counter.lock().unwrap();
            *num += 1
        });
        threads.push(thread);
    }

    for thread in threads{
        thread.join().unwrap();
    }
    println!("mutext value: {}", *counter.lock().unwrap())
}

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let http_request : Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
    println!("Request: {:#?}", http_request);
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}

fn server() {
    let listen = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listen.incoming(){
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn test_iter_map(){
    let res1: i32 = (1..30).sum();
    println!("res1: {}", res1);

    let res2 : i32 = (1..1000).find(|n| {return n % 2 == 0}).unwrap();
    println!("res2: {}", res2);

    let res3  = (1..1000).all(|n| {return n < 1000;});
    println!("res3: {}", res3);

    let res4  = (1..1000).any(|n| {return n > 1000;});
    println!("res3: {}", res4);

    for (index, value) in (1..5).enumerate(){
        println!("index: {}, value: {}", index, value);
    }

    for (index, value) in (1..5).rev().enumerate(){
        println!("index: {}, value: {}", index, value);
    }

    let mut a = (0..10);
    let mut b = (0..5);
    for (x, y) in a.zip(b){
        println!("zip x:{x}, y:{y}");
    }

    let max_value : Option<i32> = (-5..100).max();
    println!("max value: {:?}", max_value);

}

fn dash_map(){
    use dashmap::DashMap;
    let reviews = DashMap::new();
    reviews.insert("Veloren", "What a fantastic game!");

    println!("reviews: {:?}", reviews);

    let mappings = DashMap::with_shard_amount(32);
    mappings.insert(1, 2);
    mappings.insert(3, 2);
    println!("mappings: {:?}", mappings);
    println!("iters: {}", mappings.iter().count())

}

use std::ops::Add;
fn double_it<T>(value : T) -> T
where T : Add<Output=T> + Copy + Clone
{
    return value + value;
}

fn main() {
    box_list_print();

    rc_list_print();

    mut_shared_pointer();

    thread_print();

    message_queue();

    // server();

    // test_thread_pool();

    test_iter_map();

    dash_map();

    println!("{} + {} = {}", 1, 1, double_it(1));


}
