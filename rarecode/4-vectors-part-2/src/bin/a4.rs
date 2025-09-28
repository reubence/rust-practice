fn main() {
    let result = countdown(6);
    println!("{:?}", result);
  }
  
  pub fn countdown(n: u32) -> Vec<u32> {
      let mut vec = vec![];
  
      if n == 0 {
          return vec;
      }
  
      for i in 0..n {
          vec.push(n-i-1);
      }
  
      vec
  }