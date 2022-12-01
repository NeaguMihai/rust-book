fn main() {
    println!("Hello, world!");
}

fn internal_fn() {
    println!("internal_fn");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    use super::internal_fn;
    #[test]
    fn it_fails() {
        internal_fn();
    }
}