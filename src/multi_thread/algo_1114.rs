use std::sync::{Mutex, mpsc};
use std::sync::mpsc::{Sender, Receiver, SyncSender};


pub struct Foo {
    flag: Mutex<i32>,
}

impl Foo {
    pub fn new() -> Self {
        Foo{flag: Mutex::new(0)}
    }
    pub fn first(& self) {
        loop {
            let mut value = self.flag.lock().unwrap();
            if *value == 0 {
                *value = 1;
                println!("first");
                break;
            }
        }

    }

    pub fn second(& self) {
        loop {
            let mut value = self.flag.lock().unwrap();
            if *value == 1 {
                println!("second");
                *value = 2;
                break;
            }
        }
    }

    pub fn third(& self) {
        loop {
            let  value = self.flag.lock().unwrap();
            if *value == 2 {
                println!("third");
                break;
            }
        }

    }
}


pub struct Foo2 {
    second_sender: SyncSender<i32>,
    second_receiver: Mutex<Receiver<i32>>,

    third_sender: SyncSender<i32>,
    third_receiver: Mutex<Receiver<i32>>,
}

impl Foo2 {
    pub fn new() -> Self {
        let (second_producer, second_consumer) =  mpsc::sync_channel(1);
        let (third_producer, third_consumer) =  mpsc::sync_channel(1);
        Foo2{
            second_sender: second_producer,
            second_receiver: Mutex::new(second_consumer),

            third_sender: third_producer,
            third_receiver: Mutex::new(third_consumer)
        }
    }

    pub fn first(& self) {
        println!("first");
        self.second_sender.send(1).unwrap();
    }

    pub fn second(& self) {
        self.second_receiver.lock().unwrap().recv().unwrap();
        println!("second");
        self.third_sender.send(1).unwrap();
    }

    pub fn third(& self) {
        self.third_receiver.lock().unwrap().recv().unwrap();
        println!("third")
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_algo_1114() {
        let base_foo = Arc::new(Foo::new());
        let foo = Arc::clone(&base_foo);
        let handle1 = thread::spawn(move ||{foo.as_ref().third();});

        let foo = Arc::clone(&base_foo);
        let handle2 = thread::spawn(move ||{foo.as_ref().second();});

        let foo = Arc::clone(&base_foo);
        let handle3 = thread::spawn(move ||{foo.as_ref().first();});

        handle1.join().unwrap();
        handle2.join().unwrap();
        handle3.join().unwrap();
    }

    #[test]
    fn test_algo_1114_channel() {
        let base_foo = Arc::new(Foo2::new());
        let foo = Arc::clone(&base_foo);
        let handle1 = thread::spawn(move ||{foo.as_ref().third();});

        let foo = Arc::clone(&base_foo);
        let handle2 = thread::spawn(move ||{foo.as_ref().second();});

        let foo = Arc::clone(&base_foo);
        let handle3 = thread::spawn(move ||{foo.as_ref().first();});

        handle1.join().unwrap();
        handle2.join().unwrap();
        handle3.join().unwrap();
    }
}