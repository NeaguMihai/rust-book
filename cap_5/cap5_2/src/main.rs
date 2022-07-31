fn main() {
    let r = Rectangle {
        height: 20,
        width: 22,
    };
    area(&r);
    println!("{:#?}", r); //:? specified Debug format :#? pune si structul
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

fn area(r: &Rectangle) -> u64 {
    r.width * r.height
}