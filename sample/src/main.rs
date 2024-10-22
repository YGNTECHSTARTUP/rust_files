use std::{cell::RefCell, env, fmt::Debug, rc::Rc};

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}
#[derive(Debug)]
struct Size {
    byte: u64,
    kilobyte: u64,
    megabyte: u64,
    gigabyte: u64,
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

fn main() {
    let xc = |x| x;
    let y = xc(2);
    dbg!(y);
    ShirtFactory();
    let x: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut d = x.into_iter();
    println!("{:?}", d.next().unwrap());

    println!("{:?}", d.next().unwrap());
    println!("{:?}", d.next().unwrap());
    println!("{:?}", d.next().unwrap());
    println!("{:?}", d.next().unwrap());
    let a = Box::new(String::from("Alpha Beta Gamma"));
    println!("{}", *a);
    let b = Rc::new(String::from("ABC"));
    let vc = Rc::clone(&b);
    let kc = Rc::clone(&b);
    println!("{}{}{}", b, vc, kc);
    let aa = RefCell::new(4);
    *aa.borrow_mut() += 23;
    println!("{aa:?}");

    let aa = RefCell::new(String::from("ASCDS"));
    *aa.borrow_mut() += "Sdf";
    println!("{aa:?}");
}

fn structured(ab: u64) -> Size {
    let (byte, kilobyte, megabyte, gigabyte) = (ab, ab / 1000, ab / 1_000_000, ab / 1_000_000_000);
    Size {
        byte,
        kilobyte,
        megabyte,
        gigabyte,
    }
}

fn ShirtFactory() {
    #[derive(Debug, Clone, Copy)]
    enum ShirtColor {
        Red,
        Blue,
    }
    struct Inventery {
        shirts: Vec<ShirtColor>,
    }
    impl Inventery {
        fn giveaway(&self, color: Option<ShirtColor>) -> ShirtColor {
            color.unwrap_or_else(|| self.moststocked())
        }
        fn moststocked(&self) -> ShirtColor {
            let mut red = 0;
            let mut blue = 0;
            for i in &self.shirts {
                println!("{:?}", i);
                match i {
                    ShirtColor::Red => red += 1,
                    ShirtColor::Blue => blue += 1,
                };
            }
            println!("{}{}", red, blue);
            if red > blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }
    let store = Inventery {
        shirts: vec![
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Red,
        ],
    };
    let prefercolor = Some(ShirtColor::Blue);
    let giveawayitem = store.giveaway(prefercolor);
    println!("giveaway item is{:?}", giveawayitem);

    let prefercolor = None;
    let giveawayitem = store.giveaway(prefercolor);
    println!("giveaway item is{:?}", giveawayitem)
}
