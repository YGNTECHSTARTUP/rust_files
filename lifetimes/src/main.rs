fn main() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("{r}");
    }
    let str = String::from("Hello My name is Gagan");
    let str2 = "Hello My name is Gagan from Vijayawada";
    let long: str = longest(str.as_str(), str2);

    println!("Hello, world!");
}

fn longest<'a>(v: &'a str, r: &' astr) -> &str {
    
    if v.len() > r.len() {
        v
    } else {
        r
    }
}
