fn main() {
	let n = 10;
	
	for i in 0..n {
		let backward_i = n - i;
		println!("{}", backward_i);
	}

    // A more direct way to do this is to 
    // use the reverse iterator.
    
    // for i in (0..n).rev() {
    //     println!("{}", i);
    // }
}