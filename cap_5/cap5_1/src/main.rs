fn main() {
    let x = Imaginary {
        x: 54,
        z: 22,
    };
    println!("{}", x.x);


}

struct Imaginary {
    x: u64,
    #[allow(dead_code)]
    z: u64,
}

#[allow(dead_code)]
fn to_real(i: Imaginary) -> Imaginary {
    Imaginary {
        x: i.x,
        ..i //destructuring
    }
}

#[allow(dead_code)]
struct UnitStruct;

#[allow(dead_code)]
struct ToupleStrunct(u32, u32);