fn main() {
	let v: Vec<i32> = vec![1, 2, 3];
	
	let result = append_sum(&v);
	println!("{:?}", result);
}

// your code here

pub fn append_sum(v: &Vec<i32>) -> Vec<i32>{
    let mut vec = v.clone();
    let mut sum = 0;

    if vec.len() == 0 {
        vec.push(0);
        return vec;
    }

    for i in 0..vec.len() {
        sum = sum + vec[i];
    }

    vec.push(sum);
    
    vec     
}