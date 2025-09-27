fn main() {
    let x = 42;
    let result = less_than_100(x);
    println!("{}", result);
}

pub fn less_than_100(x: i32) -> bool {
    if x < 100 {
        // You cannot always omit the return statement in Rust. 
        // If you want to do an “early return” you need to use 
        //the return statement.
        return true;   
    }
    false
}