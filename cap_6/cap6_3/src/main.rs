enum Mony {
    Penny(u8),
    Other(u8)
}
fn main() {
    let config_max = Mony::Penny(6);
    if let Mony::Penny(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("The maximum is not configured");
    }
}
