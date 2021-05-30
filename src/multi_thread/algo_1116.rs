use std::sync::mpsc::{SyncSender, Receiver};
use std::sync::{Mutex, mpsc};

struct ZeroEvenOdd {
    zero_sender: SyncSender<i32>,
    zero_receiver: Mutex<Receiver<i32>>,

    even_sender: SyncSender<i32>,
    even_receiver: Mutex<Receiver<i32>>,

    odd_sender: SyncSender<i32>,
    odd_receiver: Mutex<Receiver<i32>>,

    n: i32,
}

impl ZeroEvenOdd {
    pub fn new(n: i32) -> Self {

        let (zero_sender, zero_receiver) = mpsc::sync_channel(1);
        let (even_sender, even_receiver) = mpsc::sync_channel(1);
        let (odd_sender, odd_receiver) = mpsc::sync_channel(1);


        ZeroEvenOdd{
            zero_sender,
            zero_receiver: Mutex::new(zero_receiver),

            even_sender,
            even_receiver: Mutex::new(even_receiver),

            odd_sender,
            odd_receiver: Mutex::new(odd_receiver),

            n
        }
    }

    pub fn zero(&self) {

        for i in 0..self.n {
            if i == 0 || self.zero_receiver.lock().unwrap().recv().unwrap() > 0 {
                print!("0");
                if (i + 1) % 2 == 0 {
                    self.odd_sender.send(i + 1).unwrap();
                } else {
                    self.even_sender.send(i + 1).unwrap();
                }
            }
        }

    }

    pub fn even(&self) {
        for _i in 0..(self.n / 2) {
            let val = self.even_receiver.lock().unwrap().recv().unwrap();
            print!("{}", val.to_string());
            self.zero_sender.send(1).unwrap();
        }
    }

    pub fn odd(&self) {
        for _i in 0..(self.n - self.n / 2) {
            let val = self.odd_receiver.lock().unwrap().recv().unwrap();
            print!("{}", val.to_string());
            self.zero_sender.send(1).unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::multi_thread::algo_1116::ZeroEvenOdd;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_zero_even_odd() {
        let zero_even_odd_base = Arc::new(ZeroEvenOdd::new(4));
        let zero_even_odd = Arc::clone(&zero_even_odd_base);
        let handle1 = thread::spawn(move|| {
            zero_even_odd.zero();
        });
        let zero_even_odd = Arc::clone(&zero_even_odd_base);
        let handle2 = thread::spawn(move|| {
            zero_even_odd.even();
        });

        let zero_even_odd = Arc::clone(&zero_even_odd_base);
        let handle3 = thread::spawn(move|| {
            zero_even_odd.odd();
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
        handle3.join().unwrap();


    }
}