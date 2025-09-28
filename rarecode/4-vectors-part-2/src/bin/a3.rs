fn main() {
    let result = simple_count(6);
    println!("{:?}", result);
  }
  
  pub fn simple_count(n: u32) -> Vec<u32> {
      // your code here
  
      let mut vec = vec![];
  
      if n == 0 {
          return vec;
      }
      
      for i in 0..n {
          vec.push(i+1)
      }
  
      vec
  } 