fn main() {
    let x = 42;
    let y = 43;
    let result = x_and_y_greater_than_20(x, y);
    println!("{}", result);
}

// Your code here
pub fn x_and_y_greater_than_20(x: i32, y: i32) -> bool {
    if x > 20 && y > 20 {
        return true;
    }

    false
}