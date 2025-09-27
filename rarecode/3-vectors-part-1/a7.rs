// Since we passed the index through a function argument, 
// we must be explicit about the type. Since the number is 
// used to access an array, it must be of type usize.

fn main() {
    let index_at = 2;
    let v = vec![2, 4, 6, 8];    
    let result = get_index_at(v, index_at);
    println!("{}", result);
}

pub fn get_index_at(v: Vec<i32>, i: usize) -> i32 {
    v[i]
}

