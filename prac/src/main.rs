use std::collections::HashMap;
use std::io;
// use std::os::windows::thread;
use std::sync::mpsc;
// use std::thread;
// use std::thread;
use std::thread;

use chrono::Local;
use chrono::Utc;
struct User {
    name: String,
    age: u8,
}

impl User {
    fn display_user(&self) {
        println!("name is :{},age is :{}", self.name, self.age)
    }
}
// #[derive(Debug)]
enum Direction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
}

struct Users {
    name: String,
    age: u32,
}

trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Users {
    fn summarize(&self) -> String {
        return format!("The Name is {} and the age is {}", self.name, self.age);
    }
}

fn vece() {
    let vec: Vec<i32> = vec![10, 20, 31, 412, 31, 352, 355];
    dbg!(&vec);
    let new: Vec<i32> = vec.iter().filter(|n| *n % 2 != 0).map(|n| n * 2).collect();
    dbg!(new);
}

fn main() {
    let user = Users {
        name: String::from("Gagan"),
        age: 18,
    };
    dbg!(user.summarize());
    let t = Utc::now();
    println!("{}", t);
    let time = Local::now();
    println!("{}", time);
    let user = User {
        name: String::from("Gagan"),
        age: 18,
    };
    let direction = Direction::North(123);
    match direction {
        Direction::North(a) => println!("GO to NOrth {a}"),
        Direction::South(b) => println!("GO to South {b}"),
        Direction::East(c) => println!("Go to East {c}"),
        Direction::West(d) => println!("Go to West {d}"),
    }
    user.display_user();
    let a = 10;
    let b = String::from("Hi Hello Namaste");
    dbg!(a, b);
    println!("Hello, world!");
    dbg!(iseven(22));
    dbg!(iseven(13));
    // let c = fib(12);
    // dbg!(c);
    dbg!(getstrlen(String::from("Hi Hello Namaste")));
    dbg!(strlen("Hello Namaste"));
    dbg!(area(user).unwrap());
    // vector();
    // hasmap();
    vece();
    // let a = thread::spawn(|| {
    //     for i in 0..100 {
    //         println!("{i}");
    //     }
    // });
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hello Babai");
        tx.send(val).unwrap();
    });
    let xd = rx.recv().unwrap();
    dbg!(xd);
    // a.join().unwrap();

    string(String::from("Helo My name"));
    // dbg!(fib(2));
    // let aa = String::from("Hello");
    // let bb = String::from("h");
    // let aa = "Hello";
    // let bb = "h";
    // let a = long(&aa, &bb);
    // dbg!(a);
    for i in 0..100 {
        println!("Hello Namaste");
    }
}

fn string(a: String) {
    let b = a.trim().chars();
    for i in b {
        if i == ' ' {
            return;
        }
        print!("{i}");
    }
}
fn vector() {
    println!("Enter How Many ELements You Wanna Add");
    let mut m = String::new();
    io::stdin().read_line(&mut m).expect("Enter Correctly");
    let d: u32 = m.trim().parse().expect("Expected An Number");
    let mut vec = Vec::new();
    for i in 0..d {
        let mut y = String::from("");
        println!("Enter The ELements {i}");
        io::stdin().read_line(&mut y).expect("Expected an Element");
        let c: u32 = y.trim().parse().expect("Expected An Number");
        vec.push(c)
    }
    let mut ev = Vec::new();
    for v in vec.iter() {
        if v % 2 == 0 {
            ev.push(v);
        }
    }

    let eve: Vec<u32> = vec.iter().filter(|&&n| n % 2 == 0).copied().collect();

    println!(
        "The Vector is {:?}The Even Elements in THe Vector is {:?} Normal Method {:?}",
        vec, eve, ev
    );
}

fn iseven(n: u32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

fn fib(n: u32) -> u32 {
    let mut first = 0;
    let mut last = 1;
    if n == 0 {
        return first;
    } else if n == 1 {
        return last;
    }
    for i in 0..(n + 1) {
        let temp = last;
        last = first + last;
        first = temp;
    }
    return last;
}

fn hasmap() {
    let vec: Vec<(String, u32)> = vec![(String::from("Gagan"), 18), (String::from("Naidu"), 24)];
    dbg!(&vec);
    let hm = hashee(vec);
    dbg!(hm);
}

fn hashee(vec: Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}

fn getstrlen(str: String) -> usize {
    str.len()
}

fn strlen(str: &str) -> usize {
    str.chars().count()
}

fn area(user: User) -> Option<i32> {
    if user.name == String::from("Gagan") {
        Some(143)
    } else {
        None
    }
}
