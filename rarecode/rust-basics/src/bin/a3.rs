fn main() {
    let x = 42;
    let result = increment(x);
    println!("{}", result);
}

pub fn increment(x: i32) -> i32 {
    // your code here
    x + 1
    // Note that if the last line of code in a function doesn't have a semicolon, that means that value will be returned.
}