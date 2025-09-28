fn main() {
	let v = vec![1,2,3];
	let k = 3;
	
	let result = find_idx_of(&v, k);
	println!("{}", result);
	println!("{}", k);
	println!("{:?}", &v);
}

// pub fn find_idx_of ... your code here
pub fn find_idx_of(v: &Vec<i32>, k: i32) -> usize {

    if v.len() == 0 {
        return 0;
    }

    for i in 0..v.len() {
        if v[i] == k {
            return i;
        }
    }

    return v.len();
    
}