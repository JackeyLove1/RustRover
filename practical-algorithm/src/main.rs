use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use ndarray::Array;
use ndarray::arr2;
use log::{info, trace, warn};

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
    println!("Random char: {}", rng.gen::<char>());
    println!("Integer: {}", rng.gen_range(0..10));

    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);

    let mut vec_i32 = vec![1, 5, 10, 2, 15];
    vec_i32.sort();
    println!("vec_i32: {:?}", vec_i32);

    let mut vec_f64 = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec_f64.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("vec_f64: {:?}", vec_f64);

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    people.sort();
    println!("people: {:?}", people);

    people.sort_by(|a, b| a.age.cmp(&b.age));
    println!("people: {:?}", people);

    let a = arr2(&[[1,2,3],[4,5,6]]);
    let b = arr2(&[[6, 5, 4], [3, 2, 1]]);
    let sum = &a + &b;
    println!("sum: {:?}", sum);

    let angle: f64 = 2.0;
    let hypotenuse = angle.sin();
    println!("hypotenuse: {}", hypotenuse);

    let query = "Hello, World!";
    log::error!("Executing query: {}", query);
}

#[derive(Debug, Eq, Ord, PartialEq,PartialOrd)]
struct Person {
    name : String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person{
            name,
            age,
        }
    }
}
