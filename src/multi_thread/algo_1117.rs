use std::sync::{Mutex, mpsc};
use std::sync::mpsc::{SyncSender, Receiver};

struct H2O {
    hydrogen_counter: i32,
    oxygen_counter: i32,

    hydrogen_sender: SyncSender<i32>,
    hydrogen_receiver: Mutex<Receiver<i32>>,

    oxygen_sender: SyncSender<i32>,
    oxygen_receiver: Mutex<Receiver<i32>>,


}

impl H2O {
    pub fn new(inp: &str) -> Self {
        let mut hydrogen_counter = 0;
        let mut oxygen_counter = 0;
        for c in inp.chars().into_iter() {
            match c {
                'H' => hydrogen_counter+=1,
                'O' => oxygen_counter += 1,
                _ => panic!("invalid input"),
            }
        }

        let (hydrogen_sender, hydrogen_receiver) = mpsc::sync_channel(1);
        let (oxygen_sender, oxygen_receiver) = mpsc::sync_channel(1);

        H2O{
            hydrogen_counter,
            oxygen_counter,

            hydrogen_sender,
            hydrogen_receiver: Mutex::new(hydrogen_receiver),

            oxygen_sender,
            oxygen_receiver: Mutex::new(oxygen_receiver),

        }
    }

    pub fn hydrogen(&self) {
        let mut hydrogen_counter = self.hydrogen_counter;
        let mut output_hydrogen_counter = 0;
        while hydrogen_counter >= 0 {
            if output_hydrogen_counter < 2 {
                print!("H");
                output_hydrogen_counter += 1;
                hydrogen_counter -= 1;
            } else {
                self.oxygen_sender.send(hydrogen_counter).unwrap();
                let remain_oxygen = self.hydrogen_receiver.lock().unwrap().recv().unwrap();
                output_hydrogen_counter = 0;
                if hydrogen_counter == 0 {
                    break;
                }
            }
        }
    }

    pub fn oxygen(&self) {
        let mut oxygen_counter = self.oxygen_counter;
        while oxygen_counter > 0 {
            let remain_hydrogen = self.oxygen_receiver.lock().unwrap().recv().unwrap();
            println!("O");
            oxygen_counter -= 1;
            self.hydrogen_sender.send(oxygen_counter);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::multi_thread::algo_1117::H2O;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_h2o() {
        let h2o_base = Arc::new(H2O::new("HHOOHHHHOHOH"));
        let h2o = Arc::clone(&h2o_base);
        let handle1 = thread::spawn(move|| {
            h2o.hydrogen();
        });

        let h2o = Arc::clone(&h2o_base);
        let handle2 = thread::spawn(move|| {
            h2o.oxygen();
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

    }
}