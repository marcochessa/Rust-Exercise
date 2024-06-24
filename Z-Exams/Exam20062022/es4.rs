// Domanda 4

// Un paradigma frequentemente usato nei sistemi reattivi è costituito dall'astrazione detta Looper.
// Quando viene creato, un Looper crea una coda di oggetti generici di tipo Message ed un thread. 
// Il thread attende - senza consumare cicli di CPU - che siano presenti messaggi nella coda, 
// li estrae uno a uno nell'ordine di arrivo, e li elabora. Il costruttore di Looper riceve due parametri, 
// entrambi di tipo (puntatore a) funzione: `process(...)` e `cleanup()`. La prima è una funzione responsabile 
// di elaborare i singoli messaggi ricevuti attraverso la coda; tale funzione accetta un unico parametro 
// in ingresso di tipo Message e non ritorna nulla; la seconda è una funzione priva di argomenti e valore di ritorno 
// e verrà invocata dal thread incapsulato nel Looper quando esso starà per terminare.

// Looper offre un unico metodo pubblico, thread safe, oltre a quelli di servizio, necessari per gestirne il ciclo di vita: 
// `send(msg)`, che accetta come parametro un oggetto generico di tipo Message che verrà inserito nella coda 
// e successivamente estratto dal thread ed inoltrato alla funzione di elaborazione. 
// Quando un oggetto Looper viene distrutto, occorre fare in modo che il thread contenuto al suo interno 
// invochi la seconda funzione passata nel costruttore e poi termini.

// Si implementi, utilizzando il linguaggio Rust tale astrazione tenendo conto che i suoi metodi dovranno essere thread-safe.

use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;

struct Looper <Message> {
    sender: Sender<Message>,
    thread_handle: Option<thread::JoinHandle<()>>,
    cleanup: Fn()
}

impl <Message> Lopper <Message>{
    pub fn new(process_fn: Fn(Message), cleanup: Fn()) -> Self{
        let (tx, rx) = channel();
        thread::spawn(move|| {
            loop{
                match rx.recv() {  }
            }
        })
    }

    {
    let (tx, rx) = channel();

    thread::spawn(move || {
    loop {
    match rx.recv() {
    Ok(msg) => process(msg),
    Err(_) => break,
    }
    }
    cleanup();
    });

    Arc::new(Looper { sender: tx })
    }

    pub fn send(&self, msg: Message) {
        self.sender.send(msg).unwrap();
    }
}