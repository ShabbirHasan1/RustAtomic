use std::thread;

fn main() {
  let numbers = vec![1,2,3];

  thread::scope(|s| {
    s.spawn(|| {
      println!("length: {}", numbers.len());
    });
    s.spawn(|| {
      for n in &numbers {
        println!("{n}");
      }
    });
  });
}

// This pattern guarantees that none of the threads spawned in the scope can outlive the scope. 
// Because of that, this scoped spawn method does not have a 'static bound on its argument type, 
// allowing us to reference anything as long as it outlives the scope, such as numbers.

// In the example above, both of the new threads are concurrently accessing numbers. 
// This is fine, because neither of them (nor the main thread) modifies it. 
// If we were to change the first thread to modify numbers, as shown below, 
// the compiler wouldn’t allow us to spawn another thread that also uses numbers:

// let mut numbers = vec![1, 2, 3];
// thread::scope(|s| {
//     s.spawn(|| {
//         numbers.push(1);
//     });
//     s.spawn(|| {
//         numbers.push(2); // Error!
//     });
// });