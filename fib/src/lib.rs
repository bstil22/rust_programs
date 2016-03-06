use std::fmt::{self, Formatter, Display};
fn main() {
    let values: Vec<i32> = (1..42).map(|x| fib(x)).collect();
    for x in &values {
      print!("{} ", x);
    }
}

fn fib(x: i32) -> i32 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    } else {
        return fib(x-1) + fib(x-2);
    }
}
