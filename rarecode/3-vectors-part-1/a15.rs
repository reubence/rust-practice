fn main() {
	let v = vec![1,2,1,4,5];
	let k = 1;
	
	let result = contains_k_twice(v, k);
	println!("{}", result);
}

pub fn contains_k_twice(v: Vec<i32>, k: i32) -> bool {
    // TODO
    let mut n = 0;

    for i in 0..v.len() {
        if v[i] == k {
            n = n + 1;
        };

        if n == 2 {
            return true;
        };
    }
    
    false
} 