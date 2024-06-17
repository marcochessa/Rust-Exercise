mod lab5sol;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::Mutex;

struct Waiter {
    senders: Vec<Sender<()>>,
    receiver: Receiver<()>,
}

impl Waiter {
    fn wait(&self) {
        // Invia un messaggio su ciascuno degli ingressi
        for sender in &self.senders {
            sender.send(()).unwrap();
        }

        // Attende di ricevere n-1 messaggi
        for _ in 0..self.senders.len() {
            self.receiver.recv().unwrap();
        }
    }
}

pub struct CyclicBarrier {
    waiters: Mutex<Vec<Waiter>>,
}


impl CyclicBarrier {
    pub fn new(n: usize) -> CyclicBarrier {
        let mut waiters = Vec::<Waiter>::with_capacity(n);
        let mut senders = Vec::with_capacity(n);
        let mut receivers = Vec::with_capacity(n);

        //Creo i canali
        for _ in 0..n {
            let (tx, rx) = channel();
            senders.push(tx);
            receivers.push(rx);
        }

        //Uso rev per ciclare in maniera inversa ed effettuare la pop dal vettore e mantenere lo stesso ordine iniziale
        for i in (0..n).rev() {
            let mut waiter_senders = Vec::with_capacity(n - 1);
            for j in 0..n {
                if i != j {
                    waiter_senders.push(senders[j].clone());
                }
            }
            waiters.push(Waiter {
                senders: waiter_senders,
                //Receiver non implementa clone per motivi di sicurezza quindi è necessario consumare il valore
                receiver: receivers.pop().unwrap()
            });
        }

        Self {
            waiters: Mutex::new(waiters)
        }
    }

    fn get_waiter(&self) -> Waiter {
        self.lock().unwrap().waiters.pop().unwrap()
    }
}

fn main() {
    let cbarrrier = CyclicBarrier::new(3);
    let mut vt = Vec::new();
    for i in 0..3 {
        let waiter = cbarrrier.get_waiter();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                waiter.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
}