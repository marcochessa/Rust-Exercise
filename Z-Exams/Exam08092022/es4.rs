// Domanda 4
//
// Punteggio: 6,0
//
// In un sistema concorrente, ciascun thread può pubblicare eventi per rendere noto ad altri thread quanto sta facendo.
// Per evitare un accoppiamento stretto tra mittenti e destinatari degli eventi, si utilizza un Dispatcher.
// Questo è un oggetto thread-safe che offre il metodo `dispatch(msg: Msg)` mediante il quale un messaggio di tipo generico `Msg`
// (soggetto al vincolo di essere clonabile) viene reso disponibile a chiunque si sia sottoscritto.
//
// Un thread interessato a ricevere messaggi può invocare il metodo `subscribe()` del Dispatcher:
// otterrà come risultato un oggetto di tipo `Subscription` mediante il quale potrà leggere i messaggi che da ora in poi saranno
// pubblicati attraverso il Dispatcher. Per ogni sottoscrizione attiva, il Dispatcher mantiene internamente l'equivalente di una
// coda ordinata (FIFO) di messaggi non ancora letti. A fronte dell'invocazione del metodo `dispatch(msg: Msg)`,
// il messaggio viene clonato ed inserito in ciascuna delle code esistenti.
//
// L'oggetto `Subscription` offre il metodo bloccante `read() -> Option<Msg>`.
// Se nella coda corrispondente è presente almeno un messaggio, questo viene rimosso e restituito;
// se nella coda non è presente nessun messaggio e il Dispatcher esiste ancora, l'invocazione si blocca fino a che non viene
// inserito un nuovo messaggio; se invece il Dispatcher è stato distrutto, viene restituito il valore corrispondente all'opzione vuota.
//
// Gli oggetti `Dispatcher` e `Subscription` sono in qualche modo collegati, ma devono poter avere cicli di vita indipendenti:
// la distruzione del Dispatcher non deve impedire la consumazione dei messaggi già recapitati ad una `Subscription`, ma non ancora letti;
// parimenti, la distruzione di una `Subscription` non deve impedire al Dispatcher di consegnare ulteriori messaggi alle eventuali altre
// `Subscription` presenti.

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub struct  Dispatcher<Msg: Clone>{
    subscriptions: Mutex<Vec<Option<Sender<Msg>>>>,
}

impl <Msg: Clone> Dispatcher<Msg> {
    pub fn new() -> Self<>{
        Self{
            subscriptions: Mutex::new(Vec::new())
        }
    }

    pub fn subscribe(&self)-> Subscription<Msg>{
        let mut subs = self.subscriptions.lock().unwrap();
        let (tx,rx) = channel();
        subs.push(Some(tx));
        Subscription{
            receiver: rx
        }
    }

    pub fn dispatch(&self, msg:Msg){
        let mut subs = self.subscriptions.lock().unwrap();
        for sub in subs.iter_mut() {
            if let Some(sender) = sub.as_mut() {
                if sender.send(msg.clone()).is_err() {
                    // Se la spedizione fallisce, rimuove il sender
                    *sub = None;
                }
            }
        }
    }
}

pub struct  Subscription <Msg: Clone>{
    receiver: Receiver<Msg>
}

impl <Msg: Clone> Subscription<Msg> {
    pub fn new(receiver: Receiver<Msg>) -> Self{
        Self{
            receiver
        }
    }

    pub fn read(&self) -> Option<Msg>{
        match self.receiver.recv() {
            Ok(msg) => Some(msg),
            Err(_) => None
        }
    }

}

fn main() {
    let mut dispatcher = Arc::new(Dispatcher::new());

    let dispatcher_clone1 = dispatcher.clone();
    let handle1 = std::thread::spawn(move || {
        let subscription1 = dispatcher_clone1.subscribe();
        for _ in 0..5 {
            if let Some(msg) = subscription1.read() {
                println!("Subscriber 1 received: {}", msg);
            }
        }
    });

    let dispatcher_clone2 = dispatcher.clone();
    let handle2 = std::thread::spawn(move || {
        let subscription2 = dispatcher_clone2.subscribe();
        for i in 0..5 {
            if let Some(msg) = subscription2.read() {
                println!("Subscriber 2 received: {}", msg);
            }
        }
    });

    for i in 0..5 {
        dispatcher.dispatch(i);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
}