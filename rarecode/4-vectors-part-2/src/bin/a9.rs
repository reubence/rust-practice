fn main() {
	let v = vec![4,2,6];
	let result = add_1_to_each(v);
	println!("{:?}", result);
}

// pub fn add_1_to_each your code here

pub fn add_1_to_each(v: Vec<i32>) -> Vec<i32> {
    let mut vec = v.clone();

    for i in 0..vec.len() {
        vec[i] = vec[i] + 1;
    }

    vec
}