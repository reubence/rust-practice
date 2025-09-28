fn main() {
	let v = vec![4,2,6];
	let result = append_sum(v);
	println!("{:?}", result);
}

pub fn append_sum(v: Vec<i32>) -> Vec<i32> {
    // your code here

    let mut vec = v.clone();

    if v.len() == 0 {
        vec.push(0);
        return vec;
    }

    let mut sum = 0;

    for i in 0..vec.len() {
        sum = vec[i] + sum;
    }

    vec.push(sum);
    
    vec
} 