// Domanda 4
//
// Una cache è una struttura dati, generica, thread-safe che consente di memorizzare coppie chiave/valore
// per un periodo non superiore ad una durata stabilita per ciascuna coppia.
// Nell'intervallo di validità associato alla coppia, richieste di lettura basate sulla chiave restituiscono
// il valore corrispondente, se presente. Trascorso tale periodo, eventuali richieste relative alla
// stessa chiave non restituiscono più il valore.
//
// Poiché il numero di richieste in lettura può essere molto maggiore delle richieste in scrittura,
// è necessario fare in modo che le prime possano sovrapporsi temporalmente tra loro, mentre le seconde
// dovranno necessariamente essere con accesso esclusivo. Per evitare la saturazione della struttura,
// quando si eseguono operazioni di scrittura, si provveda ad eseguire un ciclo di pulizia, eliminando
// le eventuali coppie scadute.
//
// Si implementi in Rust la struct `Cache<K: Eq + Hash, V>` dotata dei seguenti metodi:
//
// ```rust
// pub fn new() -> Self
// // Crea una nuova istanza
//
// pub fn size(&self) -> usize
// // Restituisce il numero di coppie presenti nella mappa
//
// pub fn put(&self, k: K, v: V, d: Duration) -> ()
// // Inserisce la coppia k/v con durata pari a d
//
// pub fn renew(&self, k: &K, d: Duration) -> bool
// // Rinnova la durata dell'elemento rappresentato dalla chiave k; restituisce true se la chiave
// // esiste e non è scaduta, altrimenti restituisce false
//
// pub fn get(&self, k: &K) -> Option<Arc<V>>
// // Restituisce None se la chiave k è scaduta o non è presente nella cache; altrimenti restituisce
// // Some(a), dove a è di tipo Arc<V>
// ```
//
// Si ricordi che `Duration` è una struttura contenuta in `std::time`, che rappresenta una durata non
// negativa. Può essere sommato ad un valore di tipo `std::time::Instant` (che rappresenta un momento
// specifico nel tempo) per dare origine ad un nuovo `Instant`, collocato più avanti nel tempo.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

// Struttura della cache thread-safe che memorizza coppie chiave/valore con una scadenza
struct Cache<K: Eq + Hash, V> {
    cache_map: RwLock<HashMap<K, (Arc<V>, Instant)>>, // Mappa protetta da un RwLock
}

impl<K: Eq + Hash, V> Cache<K, V> {
    // Crea una nuova istanza della cache
    pub fn new() -> Self {
        Self {
            cache_map: RwLock::new(HashMap::new()), // Inizializza la mappa interna
        }
    }

    // Restituisce il numero di coppie presenti nella cache
    pub fn size(&self) -> usize {
        self.cache_map.read().unwrap().len()
    }

    // Inserisce una coppia chiave/valore con una durata specificata
    pub fn put(&self, k: K, v: V, d: Duration) {
        let expiration = Instant::now() + d; // Calcola l'istante di scadenza
        let mut map = self.cache_map.write().unwrap();
        map.insert(k, (Arc::new(v), expiration)); // Inserisce la coppia nella mappa
        map.retain(|_k, (_v, exp)| *exp > Instant::now()); // Rimuove le coppie scadute
    }

    // Rinnova la durata di una chiave esistente; restituisce true se la chiave esiste e non è scaduta, altrimenti false
    pub fn renew(&self, k: &K, d: Duration) -> bool {
        let mut map = self.cache_map.write().unwrap();
        // Pulisco per assicurarmi che non ci siano elementi scaduti, quindi se successivamente ottengo Some non devo più controllare l'istante
        map.retain(|_k, (_v, exp)| *exp > Instant::now());
        match map.get_mut(k) {
            Some((_v, exp)) => {
                *exp = Instant::now() + d; // Aggiorna l'istante di scadenza
                true
            }
            None => false, // La chiave non esiste o è scaduta
        }
    }

    // Restituisce il valore associato a una chiave se non è scaduto; altrimenti None
    pub fn get(&self, k: &K) -> Option<Arc<V>> {
        let map = self.cache_map.read().unwrap();
        // Durante le letture non si effettua la pulizia della lista quindi mi limito a verificare la scadenza e ritornare il valore in caso positivo della verifica
        match map.get(k) {
            Some((v, exp)) => {
                if *exp > Instant::now() {
                    Some(Arc::clone(v)) // Restituisce una copia del valore se non è scaduto
                } else {
                    None // La chiave è scaduta
                }
            }
            None => None // La chiave non esiste
        }
    }
}

fn main() {
    use std::thread;
    use std::time::Duration;

    println!("TEST");
    let cache = Arc::new(Cache::<String, String>::new());

    let cache_clone = Arc::clone(&cache);
    thread::spawn(move || {
        cache_clone.put("key1".to_string(), "value1".to_string(), Duration::from_secs(5));
        println!("Inserted key1 with value 'value1' for 5 seconds");
    }).join().unwrap();

    thread::sleep(Duration::from_secs(2));

    match cache.get(&"key1".to_string()) {
        Some(value) => println!("Retrieved key1 with value: {}", value),
        None => println!("key1 has expired or does not exist"),
    }

    let cache_clone = Arc::clone(&cache);
    thread::spawn(move || {
        cache_clone.renew(&"key1".to_string(), Duration::from_secs(5));
        println!("Renewed key1 for another 5 seconds");
    }).join().unwrap();

    println!("Cache size: {}", cache.size());

    thread::sleep(Duration::from_secs(4));

    match cache.get(&"key1".to_string()) {
        Some(value) => println!("Retrieved key1 with value: {}", value),
        None => println!("key1 has expired or does not exist"),
    }

    thread::sleep(Duration::from_secs(4));

    match cache.get(&"key1".to_string()) {
        Some(value) => println!("Retrieved key1 with value: {}", value),
        None => println!("key1 has expired or does not exist"),
    }
}
