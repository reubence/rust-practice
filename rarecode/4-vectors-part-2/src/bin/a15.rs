fn main() {
	let v = vec![1,2,3,4];
	let idx = 1;
	
	let result = double_at_idx(v, idx);
	println!("{:?}", result);
}

pub fn double_at_idx(v: Vec<i32>, idx: usize) -> Vec<i32> {
    // your code here
    let mut vec = v.clone();

    if vec.len() <= idx {
        return vec;
    }

    vec[idx] = vec[idx] * 2;

    vec
 } 