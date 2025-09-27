
// When a variable is declared, 
// its value cannot change later, 
// unless it is a mut variable.

fn main() {
    let mut x = 3;
    x = x + 1;
    println!("{}", x);
} 