// Domanda 4
//
// Per garantire che non vengano eseguite contemporaneamente più di N invocazioni di operazioni potenzialmente lente,
// è stata definita la struttura dati `ExecutionLimiter`.
// Questa struttura è thread-safe e offre il metodo pubblico `execute(f)` che accetta una funzione `f`
// senza parametri e con tipo di ritorno generico `R`.
// Il metodo `execute()` restituisce lo stesso tipo `R` ritornato da `f` e si occupa di gestire il conteggio
// delle invocazioni in corso.
// Se il numero di invocazioni attive è già pari al limite N definito durante l'inizializzazione della struttura,
// `execute()` attende passivamente che scenda sotto la soglia prima di invocare `f`.
// Dopo l'esecuzione di `f`, il conteggio viene decrementato correttamente anche in caso di fallimento
// dell'esecuzione di `f`.
// Implementazione realizzata in Rust.

use std::sync::Mutex;

pub struct ExecutionLimiter{
    ex_counter: Mutex<i32>,
    limit: Mutex<i32>,
}

impl ExecutionLimiter{
    pub fn new(){}
}
