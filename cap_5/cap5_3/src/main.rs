use rand::Rng;

fn main() {
    let mut r = Rectangle {
        height: 20,
        width: 22,
    };
    r.area();
    println!("{:#?}", r); //:? specified Debug format :#? pune si structul
    r.random();
    println!("{:#?}", r); //:? specified Debug format :#? pune si structul
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}
impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn random(&mut self) {
        self.height = rand::thread_rng().gen_range(1..100);
        self.width = rand::thread_rng().gen_range(1..100);
    }
}