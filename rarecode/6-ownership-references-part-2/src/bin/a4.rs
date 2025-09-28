fn main() {
	let k = 5;
	let v = vec![4,5,6,7];
	let idx = 1;
	
	let result = k_is_at_idx(&v, k, idx);
	println!("{}", result);
	println!("{}", k);
	println!("{}", idx);
	println!("{:?}", &v);
}

pub fn k_is_at_idx(v: &Vec<i32>, k: i32, idx: usize) -> bool {

    if v.len() == 0 || v.len() <= idx {
        return false;
    }
    else if v[idx] == k {
        return true;
    }

    false

} 