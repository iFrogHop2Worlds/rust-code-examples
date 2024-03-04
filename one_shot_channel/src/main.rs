use std::thread;
use one_shot_channel::Channel;
fn main() {
  let mut channel = Channel::new();
  thread::scope(|s| {
      let (sender, receiver) = channel.split(); // returns 2 shared references to an exclusive borrow
      let t = thread::current();
      s.spawn(move || {
          sender.send("hello worlds");
      });
      assert_eq!(receiver.receive(), "hello worlds");
  });
}
