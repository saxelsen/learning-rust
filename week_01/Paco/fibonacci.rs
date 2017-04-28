use std::io::{self, Write};

fn main() {

    loop {
    	print!("Fibo number? ");
	 io::stdout().flush().unwrap();

    	let mut input = String::new();
    	io::stdin().read_line(&mut input)
		.expect("Failed to read line");

    	let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
    	};

	println!("Result: {}", fibo(input));
	break;
    }
}

/// This is very slow (exponential); iterative is better (0(n))
fn fibo(n: u64) -> u64 {

    if n == 0 {
        0
    } else if n <= 2 {
	1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}
