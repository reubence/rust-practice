fn main() {
	let v = vec![1,2,1,3,4];
	let result = first_unsorted(v);
	println!("{}", result);
}

// pub fn first_unsorted   your code here

pub fn first_unsorted(v:Vec<i32>) -> usize {
    for i in 0..(v.len()-1) {
        if v[i] > v[i+1] {
            return i+1;
        }
    }

    0
}