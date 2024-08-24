#[derive(Debug)]
struct Info<'a> {
    name: &'a str,
}
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("{r}");
    }
    let nameu = "gagam";
    let info = Info { name: &nameu };
    println!("{info:?}");
    let str = String::from("Hello My name is Gagan");
    {
        let str2 = "Hello My name is Gagan from Vijayawada";
        let long = longes(&str2, &str.as_str());

        println!("Hello, world!{long:?}");
    }
}

fn longest<'a>(v: &'a str, r: &'a str) -> &'a str {
    if v.len() < r.len() {
        v
    } else {
        r
    }
}

// fn longest<'a>(v: &'a str, r: &'a str) -> &'a str {
//     if v.len() < r.len() {
//         v
//     } else {
//         r
//     }
// }

fn longes<'a, 'b>(v: &'a str, r: &'b str) -> &'a str {
    &v[0..4]
}
