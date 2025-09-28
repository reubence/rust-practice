fn main() {
	let v = vec![1,1,5];
	let cond1 = all_elements_less_than_k(&v, 6); // true
	let cond2 = sum_greater_than_s(&v, 6); // true
	println!("{}", cond1 && cond2);
}

pub fn all_elements_less_than_k(v: &Vec<i32>, k: i32) -> bool {
    if v.len() == 0 {
        return true;
    }

    for i in 0..v.len() {
        if v[i] >= k {
            return false;
        }
    }

    true
}

pub fn sum_greater_than_s(v: &Vec<i32>, s: i32) -> bool {

    if v.len() == 0 {
        return false;
    }

    let mut sum = v[0];

    for i in 1..v.len() {
        sum = sum + v[i];
    }

    if sum > s {
        return true;
    }

    false
    
}    