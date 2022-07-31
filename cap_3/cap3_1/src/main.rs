fn main() {
    //must add mut in order to change variable value
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    shadow_variable();
}


fn shadow_variable() {
    let x: i8 = 5;

    println!("The value of x is: {x}");

    let x = x + 5;

    println!("The value of x is: {x}");

    {
        let x = x + 2;

        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");

}