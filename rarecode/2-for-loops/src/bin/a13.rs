fn main() {
    let n = 10;
    let result = fibonacci(n);
    println!("{}", result);
}

pub fn fibonacci(n: u32) -> u32 {
    // your code here
    let mut fib = 1;
    let mut fib_n1 = 1;
    let mut fib_n2 = 1;


    for _i in 1..n {
        fib = fib_n1 + fib_n2;
        fib_n1 = fib_n2;
        fib_n2 = fib;
    }

    fib
}