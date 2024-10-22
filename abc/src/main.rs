use std::cmp::Ordering;
use std::f32::consts::PI;
use std::io;
fn main() {
    // comparenm(10, 20);
    problem1(10, 20);
    problem2();
    problem3();
}

// fn comparenm(n: u32, m: u32) {
//     if n > 0 {
//         println!("{} is greater than {}", n, m)
//     } else if n < m {
//         println!("{} is lesser than {}", n, m)
//     } else {
//         println!("{} is equal to {}", n, m)
//     }
// }

fn problem3() {
    let mut a = String::new();
    println!("Enter a Number to check whether it is prime or not");
    io::stdin().read_line(&mut a).expect("Failed to read");
    let a: u32 = a.trim().parse().expect("Failed to convert ");
    if (isprime(a)) {
        println!("It is a Prime Number")
    } else {
        println!("It is not a Prime Number")
    }
}

fn isprime(a: u32) -> bool {
    if a < 2 {
        return false;
    }
    let limit = ((a as f32).sqrt() as u32);
    for i in 2..=limit {
        if i % 2 == 0 {
            return false;
        }
    }
    true
}
fn problem2() {
    let mut a = String::new();
    println!("Enter Your Choice 1)Circle 2)Rectangle");
    io::stdin().read_line(&mut a).expect("Failed to read");
    let a: u8 = a.trim().parse().expect("failed to read");
    match a {
        1 => {
            let mut a = String::new();
            println!("Enter the Radius");
            io::stdin().read_line(&mut a).expect("Failed to read");
            let a: f32 = a.trim().parse().expect("Expected the Numerical Value");
            let area = PI * a * a;
            println!("Area of the circle is:{}", area);
        }
        2 => {
            let mut length = String::new();
            let mut bredth = String::new();
            println!("Enter the length");
            io::stdin()
                .read_line(&mut length)
                .expect("failed to read the data");
            println!("Enter the bredth");
            io::stdin().read_line(&mut bredth).expect("failed to read");
            let length: u32 = length.trim().parse().expect("Failed to read");
            let bredth: u32 = bredth.trim().parse().expect("Failed to read");
            let area = 2 * length * bredth;
            println!("Area of the Rectangle is{}", area)
        }
        _ => {
            println!("Error Enter the Option  1 or 2")
        }
    }
}

fn problem1(a: u32, b: u32) {
    // Program to compare 2 numbers
    match a.cmp(&b) {
        Ordering::Less => println!("{} is lesser than {}", a, b),
        Ordering::Greater => println!("{} is greater than {}", a, b),
        Ordering::Equal => println!("Equal"),
    };
}
