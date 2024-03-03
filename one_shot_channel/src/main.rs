use std::thread;
use one_shot_channel::Channel;

fn main() {
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| unsafe {
        s.spawn(|| {
            channel.send("hollow world");
            t.unpark();
        });
        while !channel.is_ready() {
            thread::park();
        }
        assert_eq!(channel.receive(), "hollow world")
     })
}
