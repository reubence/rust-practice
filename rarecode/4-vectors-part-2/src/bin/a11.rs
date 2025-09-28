fn main() {
	let v = vec![1,2,3,4];
	let result = remove_odd(v);
	println!("{:?}", result);
}

//pub fn remove_odd ... your code here.
pub fn remove_odd(v: Vec<i32>) -> Vec<i32> {
    let mut vec = vec![];

    if v.len() == 0 {
        return v;
    }

    for i in 0..v.len() {
        if v[i] % 2 == 0 {
            vec.push(v[i]);
        }
    }

    vec
}