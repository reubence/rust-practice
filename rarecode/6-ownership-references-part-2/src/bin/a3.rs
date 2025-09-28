fn main() {
	let k = 5;
	let v = vec![4,5,6,7];
	
	let result = is_in_vector(&v, k);
	println!("{}", result);
	println!("{}", k);
	println!("{:?}", &v);
}

pub fn is_in_vector(v: &Vec<i32>, k: i32) -> bool {

    for i in 0..v.len() {
        if v[i] == k {
            return true;
        }
    }
    
    false
    
} 