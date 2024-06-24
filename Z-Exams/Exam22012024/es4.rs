// Domanda 4

// La struttura `MultiChannel` implementa il concetto di canale con molti mittenti e molti ricevitori.
// I messaggi inviati a questo tipo di canale sono composti da singoli byte che vengono recapitati
// a tutti i ricevitori attualmente collegati.

// Riferimenti a tipi:
use std::result::Result;
use std::sync::mpsc::{channel, Receiver, Sender, SendError};
use std::sync::Mutex;

// Metodi:
//
// new() -> Self
// // crea un nuovo canale senza alcun ricevitore collegato
//
// subscribe(&self) -> Receiver<u8>
// // collega un nuovo ricevitore al canale: da quando
// // questo metodo viene invocato, gli eventuali byte
// // inviati al canale saranno recapitati al ricevitore.
// // Se il ricevitore viene eliminato, il canale
// // continuerà a funzionare inviando i propri dati
// // ai ricevitori restanti (se presenti), altrimenti
// // ritornerà un errore
//
// send(&self, data: u8) -> Result<(), SendError<u8>>
// // invia a tutti i sottoscrittori un byte
// // se non c'è alcun sottoscrittore, notifica l'errore
// // indicando il byte che non è stato trasmesso

pub struct MultiChannel {
    // Un vettore di sender protetto da un mutex per garantire l'accesso thread-safe
    senders: Mutex<Vec<Sender<u8>>>,
}

impl MultiChannel {
    // Crea un nuovo canale senza alcun ricevitore collegato
    pub fn new() -> Self {
        Self {
            // Inizializza il vettore di sender vuoto protetto da un mutex
            senders: Mutex::new(vec![]),
        }
    }

    // Collega un nuovo ricevitore al canale
    pub fn subscribe(&self) -> Receiver<u8> {
        // Crea un nuovo canale, che restituisce un sender (tx) e un receiver (rx)
        let (tx, rx): (Sender<u8>, Receiver<u8>) = channel();

        // Blocca il mutex e ottiene un guard per il vettore di sender
        let mut guard = self.senders.lock().unwrap();

        // Proteggo il vettore dei trasmettitori con un mutex, perché non voglio che mentre ne aggiungo uno un altro thread possa operarci
        guard.push(tx);

        // Restituisce il nuovo receiver
        rx
    }

    // Invia un byte a tutti i sottoscrittori
    pub fn send(&self, data: u8) -> Result<(), SendError<u8>> {
        // Blocca il mutex e ottiene un guard per il vettore di sender
        let mut guard = self.senders.lock().unwrap();

        // Utilizza retain per inviare il dato a ogni sender nel vettore.
        // retain mantiene solo i sender per i quali s.send(data).is_ok() restituisce true,
        // rimuovendo quelli per cui l'invio fallisce (ad esempio, perché il ricevitore corrispondente è stato eliminato)
        guard.retain(|s| s.send(data).is_ok());


        // Se `guard.is_empty()` significa che il dato non è stato trasmesso a nessun ricevitore,
        // cioè non c'è più alcun sottoscrittore "iscritto" al canale, quindi ritorno un errore di tipo SendError
        if guard.is_empty() {
            return Err(SendError(data));
        } else {
            return Ok(());
        }
    }
}

fn main() {
    // Creazione di un nuovo MultiChannel
    let channel = MultiChannel::new();

    // Aggiunta di un primo ricevitore
    let receiver1 = channel.subscribe();

    // Aggiunta di un secondo ricevitore
    let receiver2 = channel.subscribe();

    // Invio di un byte al canale
    channel.send(42).unwrap();

    // Lettura dei dati dal primo ricevitore
    println!("Receiver 1: {}", receiver1.recv().unwrap());

    // Lettura dei dati dal secondo ricevitore
    println!("Receiver 2: {}", receiver2.recv().unwrap());

    // Elimino esplicitamente i ricevitori utilizzando `drop`
    drop(receiver1);
    drop(receiver2);

    // Prova a inviare un byte dopo che tutti i ricevitori sono stati eliminati
    match channel.send(99) {
        Err(SendError(data)) => {
            println!("SendError: Il dato {} non è stato inviato a nessun ricevitore.", data);
        }
        Ok(()) => {
            println!("Il dato è stato inviato con successo.");
        }
    }

}
