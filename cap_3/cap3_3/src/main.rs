fn main() {
    let x: u8 = 9;
    let func = change_ref;
    func(x);
    with_expression();
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn change_ref(mut x: u8) {
    x = 6;
    println!("{}", x);
    ();
}

fn with_expression() -> () {
    let y: u8 = {
        let x = 9;
        x + 2
    };
    println!("{y}");
}