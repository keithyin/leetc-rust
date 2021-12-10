use std::sync::mpsc::{self, SyncSender, Receiver};
use std::sync::Mutex;

struct FooBar {

    foo_sender: SyncSender<i32>,
    foo_receiver: Mutex<Receiver<i32>>,

    bar_sender: SyncSender<i32>,
    bar_receiver: Mutex<Receiver<i32>>,

}

impl FooBar {
    pub fn new() -> Self {
        let (foo_sender, foo_receiver) = mpsc::sync_channel(1);
        let (bar_sender, bar_receiver) = mpsc::sync_channel(1);
        FooBar{
            foo_sender,
            foo_receiver: Mutex::new(foo_receiver),

            bar_sender,
            bar_receiver: Mutex::new(bar_receiver),
        }
    }

    pub fn foo(&self, n: i32) {
        for i in 0..n{
            if i == 0 || self.foo_receiver.lock().unwrap().recv().unwrap() > 0 {
                print!("foo");
                self.bar_sender.send(1).unwrap();
            }
        }
    }

    pub fn bar(&self, n: i32) {
        for _i in 0..n{
            if self.bar_receiver.lock().unwrap().recv().unwrap() > 0 {
                print!("bar");
                self.foo_sender.send(1).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc};
    use crate::multi_thread::algo_1115::FooBar;
    use std::thread;

    #[test]
    fn test_algo_1115() {
        let foobar_base = Arc::new(FooBar::new());
        let foobar = Arc::clone(&foobar_base);
        let handle1 = thread::spawn(move|| {
            foobar.foo(10);
        });
        let foobar = Arc::clone(&foobar_base);
        let handle2 = thread::spawn(move || {
            foobar.bar(10);
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }
}