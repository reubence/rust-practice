fn main() {
	let v = vec![3,4,5];
	let result = modify(v);
	println!("{:?}", result);
}

pub fn modify(v: Vec<i32>) -> Vec<i32> {
    let mut my_vec = make_copy(v);
    
    // make the change
    my_vec.push(6);
    my_vec
}

pub fn make_copy(v: Vec<i32>) -> Vec<i32> {
    // your code here
    let mut vec = vec![];

    for i in 0..v.len() {
        vec.push(v[i]);
    }

    vec
} 