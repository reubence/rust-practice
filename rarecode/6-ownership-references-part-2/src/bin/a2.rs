// Similarly, Rust allows integers and boolean variables to be
// reassigned without the variable being consumed, since a bool
// is a "fixed-size" type.

fn main() {
	let x = true;
	let y = x;

	println!("{}", y);
	// print x here
    println!("{}", x);
} 