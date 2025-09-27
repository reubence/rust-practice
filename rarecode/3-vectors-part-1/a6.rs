// When using a number to index an array, 
// the type of that number cannot be i32 or u32 
// but it must be usize. The reason Rust has a 
// separate datatype for the length of the array 
// is to adjust for different CPUs and runtimes 
// having different word sizes (e.g. 32 bits or 64 bits).

fn main() {
    let k = 8;
    let v = vec![1, 2, 8, 9, 3];    
    let result = find_k(v, k);
    println!("{}", result);
}

pub fn find_k(v: Vec<i32>, k: i32) -> usize {
    for i in 0..v.len() {
        if v[i] == k {
            return i
        }
    }
    0
}