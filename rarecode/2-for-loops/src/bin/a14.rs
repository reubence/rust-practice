fn main() {
    let n = 5;
    let result = factorial(n);
    println!("{}", result);
  }
  
  pub fn factorial(n: u32) -> u32 {
    // your code here
    let mut total = 1;
  
    for i in 2..(n+1) {
      total = total * i;
    }
  
    total
  } 