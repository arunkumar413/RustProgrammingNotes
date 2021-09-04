fn main() {
    let test_var = 55;

    if test_var < 100 {
        println!("test var is less than 100")
    }

    loop {
        println!("again!"); // will print this for ever. Press Ctrl+C to stop the execution
    }
}

fn main() {}

fn calculate_length(s: &String) -> usize {
    s.len()
}
