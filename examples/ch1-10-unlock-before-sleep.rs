use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
  let n = Mutex::new(0);
  thread::scope(|s| {
    for _ in 0..10 {
      s.spawn(|| {
        let mut gaurd = n.lock().unwrap();
        for _ in 0..100 {
          *gaurd += 1;
        }
        drop(gaurd); // New: drop the gaurd before sleeping!
        thread::sleep(Duration::from_secs(1));
      });
    }
  });
  assert_eq!(n.into_inner().unwrap(), 1000);
}