fn main() {
    let x = 42;
    let y = 8;
    let result = x_or_y_less_than_10(x, y);
    println!("{}", result);
}

pub fn x_or_y_less_than_10(x: i32, y:i32) -> bool {
    if x < 10 || y < 10 {
        return true;
    }

    false
}