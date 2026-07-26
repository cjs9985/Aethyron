// This is a sample Rust file for the CI pipeline example.

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
test mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}