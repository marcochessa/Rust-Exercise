// Domanda 4
//
// Punteggio 6,0
//
// La struct `MpMcChannel<E: Send>` è un'implementazione di un canale su cui possono scrivere molti produttori e da cui possono attingere valori molti consumatori. Tale struttura offre i seguenti metodi:
//
// `new(n: usize) -> Self`
// - Crea un'istanza del canale basato su un buffer circolare di "n" elementi.
//
// `send(e: E) -> Option<()>`
// - Invia l'elemento "e" sul canale. Se il buffer circolare è pieno, attende senza consumare CPU che si crei almeno un posto libero in cui depositare il valore.
// - Ritorna:
//   - `Some(())` se è stato possibile inserire il valore nel buffer circolare.
//   - `None` se il canale è stato chiuso (Attenzione: la chiusura può avvenire anche mentre si è in attesa che si liberi spazio) o se si è verificato un errore interno.
//
// `recv() -> Option<E>`
// - Legge il prossimo elemento presente sul canale. Se il buffer circolare è vuoto, attende senza consumare CPU che venga depositato almeno un valore.
// - Ritorna:
//   - `Some(e)` se è stato possibile prelevare un valore dal buffer.
//   - `None` se il canale è stato chiuso (Attenzione: se, all'atto della chiusura, sono già presenti valori nel buffer, questi devono essere ritornati, prima di indicare che il buffer è stato chiuso; se la chiusura avviene mentre si è in attesa di un valore, l'attesa si sblocca e viene ritornato `None`) o se si è verificato un errore interno.
//
// `shutdown() -> Option<()>`
// - Chiude il canale, impedendo ulteriori invii di valori.
// - Ritorna:
//   - `Some(())` per indicare la corretta chiusura.
//   - `None` in caso di errore interno all'implementazione.
//
// Si implementi tale struttura dati in linguaggio Rust, senza utilizzare i canali forniti dalla libreria standard né da altre librerie, avendo cura di garantirne la correttezza in presenza di più thread e di non generare la condizione di panico all'interno dei suoi metodi.

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use crate::ChannelState::{Closed, Open};

#[derive(PartialEq)]
enum ChannelState{Open, Closed}
pub struct  MpMcChannel<E:Send>{
    channel: Mutex<(ChannelState, VecDeque<E>)>,  // Stato del canale e buffer circolare protetti da un Mutex
    not_empty: Condvar,
    not_full: Condvar,
}

impl <E:Send> MpMcChannel<E> {
    pub fn new(n:usize) -> Self {
        Self{
            // Crea un'istanza del canale basato su un buffer circolare di "n" elementi
            channel: Mutex::new((Open,VecDeque::with_capacity(n))),
            not_empty: Condvar::new(),
            not_full: Condvar::new()
        }
    }

    // Invia l'elemento "e" sul canale. Se il buffer circolare è pieno, attende senza consumare CPU
    // che si crei almeno un posto libero in cui depositare il valore.
    pub fn send(&self, e:E) -> Option<()>{

        // Acquisisce il lock sul canale
        let mut channel = self.channel.lock().unwrap();

        if channel.0 == Closed{
            return None // Ritorna None se il canale è chiuso
        }
        if channel.1.len() == channel.1.capacity(){
            // Se il buffer è pieno, attende che si liberi spazio (finché il canale è Open)
            channel = self.not_full.wait_while(channel,|(state,_buffer)|*state==Open).unwrap();
        }

        channel.1.push_front(e);// Inserisce l'elemento in testa al buffer
        self.not_empty.notify_all(); // Notifica i "consumatori" che il buffer non è vuoto
        return Some(())
    }

    // Legge il prossimo elemento (il più vecchio) presente sul canale. Se il buffer circolare è vuoto, attende senza
    // consumare CPU che venga depositato almeno un valore.
    pub fn recv(&self) -> Option<E> {
        let mut channel = self.channel.lock().unwrap(); // Acquisisce il lock sul canale

        if channel.1.is_empty() {
            if channel.0 == Closed {
                return None; // Ritorna None se il canale è chiuso e il buffer è vuoto
            }
            // Se il buffer è vuoto, attende che venga depositato un valore
            channel = self.not_empty.wait_while(channel, |(state, _)| *state == Open).unwrap();
        }
        let element = channel.1.pop_back(); // Preleva l'elemento in coda al buffer
        self.not_full.notify_all(); // Notifica i "produttori" che il buffer non è pieno
        element
    }

    // Chiude il canale, impedendo ulteriori invii di valori
    pub fn shutdown(&self) -> Option<()> {
        let try_lock = self.channel.lock(); // Tenta di acquisire il lock sul canale
        if try_lock.is_err() {
            return None; // Ritorna None in caso di errore nell'acquisizione del lock
        }

        let mut channel = try_lock.unwrap(); // Acquisisce il lock sul canale
        channel.0 = Closed; // Imposta lo stato del canale a Chiuso
        Some(())
    }
}

fn main() {

    println!("Testing...");
    // Creazione di un canale con capacità di 3 elementi
    let channel = MpMcChannel::new(3);

    // Test invio e ricezione di un singolo elemento
    assert_eq!(channel.send(1), Some(()));
    assert_eq!(channel.recv(), Some(1));

    // Test invio di più elementi e ricezione di uno
    assert_eq!(channel.send(2), Some(()));
    assert_eq!(channel.send(3), Some(()));
    assert_eq!(channel.send(4), Some(()));
    assert_eq!(channel.recv(), Some(2));

    // Test buffer pieno
    assert_eq!(channel.send(5), Some(())); // Dovrebbe bloccare finché non c'è spazio
    assert_eq!(channel.recv(), Some(3));

    // Test chiusura canale
    assert_eq!(channel.shutdown(), Some(()));
    assert_eq!(channel.send(6), None);
    assert_eq!(channel.recv(), Some(4));
    assert_eq!(channel.recv(), Some(5));
    assert_eq!(channel.recv(), None);

    println!("Tutti i test sono passati!");
}