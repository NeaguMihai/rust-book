fn main() {
    parse_int();
}

fn parse_int() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);
}

//All types
//signed is with 2's complement
//signed 8 bit
#[allow(dead_code)]
const A: i8 = 1;
//unsigned 8 bit
#[allow(dead_code)]
const B: u8 = 1;
//signed 16 bit
#[allow(dead_code)]
const C: i16 = 1;
//unsigned 16 bit
#[allow(dead_code)]
const D: u16 = 1;
//signed 32 bit
#[allow(dead_code)]
const E: i32 = 1;
//unsigned 32 bit
#[allow(dead_code)]
const F: u32 = 1;
//signed 64 bit
#[allow(dead_code)]
const G: i64 = 1;
//unsigned 64 bit
#[allow(dead_code)]
const H: u64 = 1;
//signed 128 bit
#[allow(dead_code)]
const I: i128 = 1;
//unsigned 128 bit
#[allow(dead_code)]
const J: u128 = 1;


//Other types
//writing with separatos
//decimal
#[allow(dead_code)]
const K: u32 = 1_000;
//hex
#[allow(dead_code)]
const L: i32 = 0xff;
//octal
#[allow(dead_code)]
const M: i32 = 0o01;
//binary
#[allow(dead_code)]
const N: i32 = 0b11001100;
//byte
#[allow(dead_code)]
const O: u8 = b'A';

//floats
//floating point 64
#[allow(dead_code)]
const P: f32 = 0.00_1;
//floating point 64
#[allow(dead_code)]
const Q: f64 = 0.000_1;

//compound types
//touple -> cannot grow or shrink.
//can have arbitrary types
#[allow(dead_code)]
const TUP: (i16, char, bool) = (12, 'g', true);
//can use destructuring
//let (r,s,t) = tup;
//can access wiyh "." tup.0
#[allow(dead_code)]
const R: i16 = TUP.0;
// () means empty return type

//Array
//fixed size, same types
#[allow(dead_code)]
const S: [i32; 5] =[8, 9, 7, 0, 5];
#[allow(dead_code)]
const T: [[i32; 2]; 2] =[[8, 9], [7, 0]];