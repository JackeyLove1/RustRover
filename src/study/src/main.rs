mod test;

fn main() {
    println!("Hello, world!");

    let value1 : u128 = "12345678910".parse().expect("Not a number!");
    println!("value1:{value1}");


    let values : [i32; 5] = [1,2,3,4,5];
    for v in values{
        println!("v:{v}");
    }

    let s1 = String::from("String1");
    take_string(s1);

    let i1 : i32 = 100;
    copy_integer(i1);

    let s2 = String::from("String2");
    take_string_length(&s2);

    let mut s3 = String::from("String3");
    mutable_string(&mut s3);
    println!("String3: {s3}");

    let s4 = String::from("Hello World!");
    let index = first_word(&s4);
    println!("index:{index}");
    let slice1 = &s4[0..index];
    println!("slice1: {slice1}");

    print_rectangle();

    let one = Number::One;
    print_number(one);

    optional_print();

}

fn take_string(string: String) {
    println!("string : {string}")
}

fn copy_integer(integer: i32) {
    println!("integer: {integer}")
}

fn take_string_length(string : &String) -> usize {
    println!("String: {}, length:{}", string, string.len());
    return string.len();
}

fn mutable_string(string : &mut String) {
    string.push_str(", hello world!");
}

fn first_word(string : &String) -> usize{
    let bytes = string.as_bytes();
    for (index, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return index;
        }
    }
    return string.len();
}

#[derive(Debug)]
struct Rectangle {
    width : u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.length * self.width;
    }

    fn square(size : u32) -> Self {
        return Self {
            width:size,
            length:size,
        }
    }
}

fn print_rectangle(){
    let rec = Rectangle{
        width: 0,
        length: 0,
    };
    println!("rec: {:?}", rec);
    println!("rec area: {}", rec.area());
    let square = Rectangle::square(10);
    println!("square: {:?}", square);

}

enum Number{
    One,
    Two,
    Three,
}

fn print_number(number : Number){
    match number {
        Number::One => println!("1"),
        Number::Two => println!("2"),
        Number::Three => println!("3"),
    }
}

fn optional_print(){
    fn plus_one(x : Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }
    let some = Some(0);
    let res1 = plus_one(None);
    let res2 = plus_one(some);
    if let Some(value) = res2 {
        println!("value: {value}")
    }
}