fn main() {
    println!("Hello, world!");
    // let p = Point { x: 1 };
    let a = longest("x", "y3");
    println!("The longest string is {}", a);
    let mut a = Point { x: &Top { y: 1 } };
    {
        let b: &Top = &Top { y: 2 };
        a.x = b;
    }
    lifetime_test(&mut a);
    println!("a is {}, b is", a.x.y);
}

// [#Test]
struct Point<'a, T> {
	x: &'a T
}

struct Top {
    y: i32
}

trait Test {
    fn print(&self) {
        println!("test");
    }
}

fn lifetime_test(a: & mut Point<Top>) {
    let b = &Top { y: 20 };
    a.x = b;
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
