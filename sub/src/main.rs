fn main() {
    println!("Hello, world!");
}

mod tests {
    #[test]
    fn it_does_not_work() {
        eprintln!("pwd: {}", std::env::current_dir().unwrap().display());
        panic!();
    }
}
