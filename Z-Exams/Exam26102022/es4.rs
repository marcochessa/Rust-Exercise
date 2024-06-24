// Un componente con funzionalità di cache permette di ottimizzare il comportamento
// di un sistema riducendo il numero di volte in cui una funzione è invocata, tenendo
// traccia dei risultati da essa restituiti per un particolare dato in ingresso.
//
// Per generalità, si assuma che la funzione accetti un dato di tipo generico `K` e
// restituisca un valore di tipo generico `V`.
//
// Il componente offre un unico metodo `get(...)` che prende in ingresso due parametri:
// 1. il valore `k` (di tipo `K`, clonabile) del parametro,
// 2. la funzione `f` (di tipo `K -> V`) responsabile della sua trasformazione,
//    e restituisce uno smart pointer clonabile al relativo valore.
//
// Se, per una determinata chiave `k`, non è ancora stato calcolato il valore
// corrispondente, la funzione viene invocata e ne viene restituito il risultato;
// altrimenti viene restituito il risultato già trovato.
//
// Il componente cache deve essere thread-safe perché due o più thread possono
// richiedere contemporaneamente il valore di una data chiave: quando questo avviene
// e il dato non è ancora presente, la chiamata alla funzione dovrà essere eseguita
// nel contesto di UN SOLO thread, mentre gli altri dovranno aspettare il risultato
// in corso di elaborazione, SENZA CONSUMARE cicli macchina.
//
// Si implementi tale componente in Rust.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Eq, PartialEq)]
enum EntryState<V> {
    Pending,
    Present(Arc<V>),
}
pub struct Cache<K:Clone+Eq+Hash, V> {
    map: Mutex<HashMap<K, EntryState<V>>>,
    cv: Condvar
}

impl <K:Clone+Eq+Hash, V> Cache<K,V>{
    pub fn new()-> Self{
        Self{
            map: Mutex::new(HashMap::new()),
            cv: Condvar::new()
        }
    }

    pub fn get(&self, k: K, f: impl FnOnce(&K) -> V + 'static) -> Arc<V> {
        let mut map = self.map.lock().unwrap();
        match map.get(&k) {
            Some(EntryState::Pending) => {
                // If another thread is computing, wait for it to finish and return the result
                loop {
                    map = self.cv.wait(map).unwrap();
                    match map.get(&k) {
                        Some(EntryState::Pending) => continue,
                        Some(EntryState::Present(arc_v)) => return arc_v.clone(),
                        None => unreachable!(),
                    }
                }
            }
            Some(EntryState::Present(v)) => v.clone(),
            None => {
                // Insert Pending state to indicate computation is ongoing
                map.insert(k.clone(), EntryState::Pending);

                // Release the lock before executing the function
                drop(map);

                // Compute the value
                let v = f(&k.clone());

                // Re-acquire the lock and update the map
                let mut map = self.map.lock().unwrap();
                let v = Arc::new(v);
                map.insert(k.clone(), EntryState::Present(v.clone()));

                // Notify other waiting threads
                self.cv.notify_all();
                v
            }
        }
    }
}

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Eq, PartialEq)]
enum EntryState<V> {
    Pending,
    Present(Arc<V>),
}
pub struct Cache<K:Clone+Eq+Hash, V> {
    map: Mutex<HashMap<K, EntryState<V>>>,
    cv: Condvar
}

impl <K:Clone+Eq+Hash, V> Cache<K,V>{
    pub fn new()-> Self{
        Self{
            map: Mutex::new(HashMap::new()),
            cv: Condvar::new()
        }
    }

    pub fn get(&self, k: K, f: impl FnOnce(&K) -> V + 'static) -> Arc<V> {
        let mut map = self.map.lock().unwrap();
        match map.get(&k) {
            Some(EntryState::Pending) => {
                // If another thread is computing, wait for it to finish and return the result
                loop {
                    map = self.cv.wait(map).unwrap();
                    match map.get(&k) {
                        Some(EntryState::Pending) => continue,
                        Some(EntryState::Present(arc_v)) => return arc_v.clone(),
                        None => unreachable!(),
                    }
                }
            }
            Some(EntryState::Present(v)) => v.clone(),
            None => {
                // Insert Pending state to indicate computation is ongoing
                map.insert(k.clone(), EntryState::Pending);

                // Release the lock before executing the function
                drop(map);

                // Compute the value
                let v = f(&k.clone());

                // Re-acquire the lock and update the map
                let mut map = self.map.lock().unwrap();
                let v = Arc::new(v);
                map.insert(k.clone(), EntryState::Present(v.clone()));

                // Notify other waiting threads
                self.cv.notify_all();
                v
            }
        }
    }
}


fn main() {
    let cache = Arc::new(Cache::new());

    // Test semplice con un singolo thread
    let result = cache.get(1, |&key| key * 2);
    assert_eq!(*result, 2);

    let result2 = cache.get(1, |&key| key * 3);
    assert_eq!(*result2, 2);

    println!("Test single-thread passed!");

    // Test multithread
    let cache_clone = Arc::clone(&cache);
    let handle1 = thread::spawn(move || {
        let result = cache_clone.get(2, |&key| key * 2);
        assert_eq!(*result, 4);
    });

    let cache_clone = Arc::clone(&cache);
    let handle2 = thread::spawn(move || {
        let result = cache_clone.get(2, |&key| key * 2);
        assert_eq!(*result, 4);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Test multi-thread passed!");
}
