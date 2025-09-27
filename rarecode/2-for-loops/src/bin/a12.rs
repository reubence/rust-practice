fn main() {
    let x = 48;
    let y = 18;
    let result = gcd(x, y);
    println!("{}", result);
}

pub fn gcd(x: u32, y: u32) -> u32 {
    // your code here
    // gcd is defined as the greatest common divisor of two numbers
    // it is the largest number that divides both x and y without leaving a remainder
    // e.g. gcd(48, 18) = 6
    // e.g. gcd(10, 15) = 5
    // e.g. gcd(10, 20) = 10
    // e.g. gcd(10, 10) = 10
    // e.g. gcd(10, 1) = 1

    // assume x and y are positive integers greater than 0
    let mut gcs = 1;
    let mut n = y;
    if x < y {
        n = x;
    }

    for i in 2..(n+1) {
        if x % i == 0 && y % i == 0 {
            gcs = i;
        }
    }
    gcs
} 