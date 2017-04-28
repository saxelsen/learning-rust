fn main() {
    println!("Fibonacci");
    let n = 10;
    println!("n = {}, Fibonacci = {}",n, fib(n)); 
}

fn fib(n:i32) -> i32 {
	if n == 0 {
		0
	} else if n == 1 {
		1
	} else {
		fib(n-1) + fib(n-2)
	}
}