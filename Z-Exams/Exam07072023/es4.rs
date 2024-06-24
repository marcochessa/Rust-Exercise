// Domanda 4
//
// Una DelayedQueue<T:Send> è un particolare tipo di coda non limitata che offre tre metodi principali,
// oltre alla funzione costruttrice:
// 1. offer(&self, t:T, i: Instant): Inserisce un elemento che non potrà essere estratto prima dell'istante di scadenza i.
// 2. take(&self) -> Option<T>: Cerca l'elemento t con scadenza più ravvicinata: se tale scadenza è già stata oltrepassata, restituisce Some(t);
// se la scadenza non è ancora stata superata, attende senza consumare cicli di CPU, che tale tempo trascorra, per poi restituire Some(t);
// se non è presente nessun elemento in coda, restituisce None. Se, durante l'attesa, avviene un cambiamento qualsiasi
// al contenuto della coda, ripete il procedimento suddetto con il nuovo elemento a scadenza più ravvicinata (ammesso che ci sia ancora).
// 3. size(&self) -> usize: Restituisce il numero di elementi in coda indipendentemente dal fatto che siano scaduti o meno.
//
// Si implementi tale struttura dati nel linguaggio Rust, avendo cura di renderne il comportamento thread-safe.
// Si ricordi che gli oggetti di tipo Condvar offrono un meccanismo di attesa limitata nel tempo,
// offerto dai metodi wait_timeout(...) e wait_timeout_while(...)).


use std::cmp::{Ordering};
use std::collections::BinaryHeap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct Item<T: Send> {
    instant: Instant,
    element: T,
}

impl<T: Send> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        self.instant == other.instant
    }
}

impl<T: Send> Eq for Item<T> {
    //Non è necessario implementarlo perché viene derivata da partial eq
}

impl<T: Send> PartialOrd for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.instant.partial_cmp(&self.instant) //other-self per ottenere l'ordine decrescente
    }
}

impl<T: Send> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.instant.cmp(&self.instant) //other-self per ottenere l'ordine decrescente
    }
}

// Struttura della coda ritardata thread-safe.
pub struct DelayedQueue<T: Send> {
    queue: Mutex<BinaryHeap<Item<T>>>,
    cv: Condvar,
}

impl<T: Send> DelayedQueue<T> {
    // Funzione costruttrice per creare una nuova coda ritardata.
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(BinaryHeap::new()),
            cv: Condvar::new(),
        }
    }

    // Metodo per inserire un nuovo elemento nella coda con un istante di scadenza specificato.
    pub fn offer(&self, t: T, i: Instant) {
        let item = Item { instant: i, element: t };
        let mut queue = self.queue.lock().unwrap();
        queue.push(item);

        // Il contesto è variato quindi notifica tutti i thread in attesa che è stato aggiunto un nuovo elemento.
        self.cv.notify_all()
    }

    // Metodo per estrarre l'elemento con la scadenza più ravvicinata se è già passato.
    // Altrimenti, attende fino a quando l'elemento può essere estratto.
    pub fn take(&self) -> Option<T> {

        // Acquisisce il lock sulla coda.
        let mut queue = self.queue.lock().unwrap();

        // Continua a iterare finché la coda non è vuota.
        while !queue.is_empty() {
            // Se entra la coda contiene almeno un elemento.
            let item = queue.peek().unwrap();
            // Se l'elemento con la scadenza più vicina è già scaduto, lo estrae e lo restituisce.
            if item.instant < Instant::now() {
                return Some(queue.pop().unwrap().element);
            } else {
                // Calcola il tempo rimanente fino alla scadenza dell'elemento in testa alla coda.
                let timeout = item.instant.duration_since(Instant::now());

                // Attende fino a quando l'elemento è pronto per essere estratto o il timeout scade.
                // wait_timeout rilascia il lock e poi lo riacquisisce quando si sveglia.
                let (q, _) = self.cv.wait_timeout(queue, timeout).unwrap();

                // Riaggiorna il riferimento a queue dopo l'attesa.
                queue = q;
            }
        }
        // Se la coda è vuota, restituisce None.
        None
    }


    // Metodo per ottenere il numero di elementi presenti nella coda.
    pub fn size(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
}

fn main() {
    println!("Testing...");
    let queue = Arc::new(DelayedQueue::new());

    // Test offer and take
    {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            let now = Instant::now();
            queue.offer(1, now + Duration::from_secs(2));
            queue.offer(2, now + Duration::from_secs(1));
            queue.offer(3, now + Duration::from_secs(3));
            queue.offer(4, now + Duration::from_secs(0));
        });
        handle.join().unwrap();
    }

    // Test take with waiting
    {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            assert_eq!(queue.take(), Some(4));
            assert_eq!(queue.take(), Some(2));
            thread::sleep(Duration::from_secs(1));
            assert_eq!(queue.take(), Some(1));
            assert_eq!(queue.take(), Some(3));
            assert_eq!(queue.take(), None);
        });
        handle.join().unwrap();
    }

    // Test size
    {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            let now = Instant::now();
            queue.offer(5, now + Duration::from_secs(1));
            queue.offer(6, now + Duration::from_secs(2));
            assert_eq!(queue.size(), 2);
        });
        handle.join().unwrap();
    }

    // Test concurrency with multiple threads
    let queue = Arc::new(DelayedQueue::new());
    let mut handles = vec![];

    for i in 0..10 {
        let queue = Arc::clone(&queue);
        handles.push(thread::spawn(move || {
            let now = Instant::now();
            queue.offer(i, now + Duration::from_millis(100 * (10 - i)));
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut handles = vec![];
    for _ in 0..10 {
        let queue = Arc::clone(&queue);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            let _ = queue.take();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All tests passed!");
}
