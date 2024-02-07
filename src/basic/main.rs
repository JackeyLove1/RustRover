use std::collections::HashMap;
use std::fmt::Alignment::Center;
use std::fs::File;

fn main() {
    println!("Hello, world!");

    // tuple
    let tuple = (1, 0.2, "Hello");
    let (a, b, c) = tuple;
    println!("a:{a}, b:{b}, c:{c}");
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let x0 = x.0;
    let x1 = x.1;
    println!("x0:{x0}, x1:{x1}");

    // array
    let _a = [1,2,3,4,5];
    let a : [i32;5] = [1,2,3,4,5];
    for element in a{
        println!("element:{element}")
    }
    for num in (1..4).rev(){
        println!("{num}")
    }

    // String
    let s = String::from("Hello, World!");
    let s_copy = s.clone();
    let s1 = &s;
    println!("copy:{s_copy}");
    let mut s2 = String::from("Hell0");
    s2.push_str(", World!");
    println!("s2:{s2}");
    {
        let s3 = &mut s2;
        s3.push_str(" ... ");
        println!("s3:{s3}");
    }
    let slice = &s2[..3];
    println!("slice:{slice}");

    for (i, &item) in s2.as_bytes().iter().enumerate(){
        if item == b' '{
            let sub = &s2[0..i];
            println!("i:{i},  sub:{sub}");
        }
    }

    struct User {
        active: bool,
        username : String,
        email : String,
        sign_in_count : u64,
    };

    let _user1 = User{
        active:true,
        username:String::from("name"),
        email : String::from("email"),
        sign_in_count : 1,
    };

    struct Color(u32, u32, u32);
    let _color = Color(0, 0, 0);

    let rec1 = Rec{
        width : 30,
        height : 50,
    };
    println!("rec1:{:?}", rec1);
    dbg!(&rec1);

    println!("area:{}", rec1.area());

    let v1 = vec![1,2,3];
    let value1 = &v1.get(0);
    match value1 {
        Some(num) => println!("num:{num}"),
        None => println!("None"),
    };
    for (idx, num) in v1.iter().enumerate(){
        println!("idx:{idx}, value:{num}");
    }

    // hashmap
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Hello"), 1);
    scores.insert(String::from("World!"), 2);
    let k1 = String::from("123");
    let v1 = scores.get(&k1).copied().unwrap_or(0);
    println!("k1:{k1}, v1:{v1}");
    for (k, v) in &scores{
        println!("key:{k}, value:{v}");
    }
    scores.entry(String::from("asdasd")).or_insert(123);

    use std::fs::File;
    // let greeting_file = File::open("hello.txt").expect("no such file!");

    let nums1 = vec![2,3,4,1,66,2,100];
    println!("num1 largest:{}", find_largest_number(&nums1));
}

#[derive(Debug)]
struct Rec{
    width : u32,
    height : u32,
}


impl Rec {
    fn square (size : u32) -> Self {
        return Self {
            width: size,
            height : size,
        }
    }

    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other : &Rec)  -> bool {
        return self.height >= other.height && self.width > other.width;
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn plus_one(x : Option<u32>) -> Option<u32> {
    match x {
        Some(i) => Some(i+1),
        None => None,
    }
}


// generic
fn find_largest_number<T : std::cmp::PartialOrd>(nums : &[T]) -> &T{
    debug_assert!(nums.len() > 0);
    let mut largest = &nums[0];
    for num in nums{
        if num > largest{
            largest = num;
        }
    }
    return largest;
}

