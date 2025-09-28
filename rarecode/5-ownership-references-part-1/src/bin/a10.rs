fn main() {
	let v1 = vec![1,2,3];
	let v2 = vec![1,1,2];
	
	let result = elementwise_sum(&v1, &v2);
	
	// can print because v1 and v2 are not consumed
	println!("{:?}", v1);
	println!("{:?}", v2);
	println!("{:?}", result);
}

pub fn elementwise_sum(x: &Vec<i32>, y: &Vec<i32>) -> Vec<i32> {

    let mut vec = x.clone();

    if x.len() == 0 {
        return vec;
    }

    for i in 0..vec.len() {
        vec[i] = vec[i] + y[i];
    }

    vec
} 