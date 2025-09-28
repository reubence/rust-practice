fn main() {
	let v = vec![1,2,3,4];
	
	let result = reverse(v);
	println!("{:?}", result);
}

pub fn reverse(v: Vec<i32>) -> Vec<i32> {
    // your code here
    let mut vec = vec![];

    for i in 0..v.len() {
        vec.push(v[v.len()-i-1]);
    }

    vec
} 