use std::thread;

fn main() {
  let numbers = vec![1,2,3];

  thread::spawn( move || {
    for n in &numbers {
      println!("{n}");
    }
  }).join().unwrap();
}


// Without move keyword in closure -> thread::spawn( || {
// Compiling atomic v0.1.0 (/Users/mai/workspace/rust/atomic)
// error[E0373]: closure may outlive the current function, but it borrows `numbers`, which is owned by the current function
//   --> examples/ch1-03-spawn-closure.rs:6:18
//    |
// 6  |   thread::spawn( || {
//    |                  ^^ may outlive borrowed value `numbers`
// 7  |     for n in &numbers {
//    |               ------- `numbers` is borrowed here
//    |