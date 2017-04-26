fn main() {
	let x: u32 = 20;
	let y = fib_iterative(x);
	println!("The iterative Fibonacci number of {} = {}", x, y);
	let z = fib_recursive(x);
	println!("The recursive Fibonacci number of {} = {}", x, z);
}

fn fib_iterative(fib_n: u32) -> u64 {
	let mut current: u64 = 1;
	let mut prev: u64 = 1;
	let mut prev_prev: u64 = 0;

	for _ in 0..fib_n - 1 {

		current = prev + prev_prev;
		prev_prev = prev;
		prev = current;
	}

	current
}

fn fib_recursive(fib_n: u32) -> u64 {
	if fib_n < 3 {
		1
	} else {
		fib_recursive(fib_n - 1) + fib_recursive(fib_n - 2)
	}
}