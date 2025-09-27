fn main() {
    let x = 42;
    let y = 43;
    let result = max(x, y);
    println!("{}", result);
}

// your code here
pub fn max(x: i32, y:i32) -> i32 {
    if x < y {
        return y;
    }

    x
}