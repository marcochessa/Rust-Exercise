# Domanda 1
**Punteggio 3,0**

#### Si definiscano le principali aree di memoria associate ad un eseguibile e si mostri, attraverso opportuni esempi di codice, in quale situazione ciascuna di esse viene utilizzata.

## Risposta
Le principali aree di memoria associate a un eseguibile sono:

1. **Stack**: Utilizzata per la gestione delle chiamate di funzione, variabili locali e contesto delle funzioni.
2. **Heap**: Utilizzata per l'allocazione dinamica della memoria durante l'esecuzione del programma. 
3. **Segmento delle costanti**: a seconda del sistema operativo, potrebbe essere accorpato a quello di codice, contiene le costanti. Nel caso di Rust sono presenti le costanti standard e tutte le variabili con lifetime `'static` (es. `let MAX: &'static i32 = &10;`).
4. **Segmento delle variabili globali**: contiene le variabili globali.
5. **Text Segment**: Memoria che contiene il codice eseguibile del programma.

##### Esempio - Stack e Heap
```rust
fn main() {
    let local_var = 10; // Variabile locale nello stack
    println!("local_var: {}", local_var);
    let heap_var = Box::new(20); // Allocazione dinamica nell'heap
    println!("Heap value: {}", heap_var);
}
```

# Domanda 2
**Punteggio 3,0**

#### Sia dato un primo valore di tipo `std::cell::Cell<T>` ed un secondo valore di tipo `std::cell::RefCell<T>` (dove T fa riferimento alla medesima entità). Si indichino le differenze tra i due e le modalità di occupazione della memoria (quantità, zone di memoria, ecc.).

## Risposta
Gli smart pointer `Cell<T>` e `RefCell<T>` sono due smart pointer che Rust rende disponibili che implementano un meccanismo di mutabilità interna, cioè fanno in modo che le stringenti regole del borrow checker siano rispettate solo a runtime, e non a compile time. La differenza tra i due è semplicemente il fatto che `Cell<T>` non permette di creare riferimenti (e dunque prestiti) al valore contenuto nello smart pointer.

`Cell<T>` è un tipo generico di Rust che va ad inquadrare un determinato dato T. Non contiene nient'altro che il dato.
In termini di memoria quindi, `Cell` occupa esattamente quanto il valore `T`, ed è allocato sullo stack.

Il borrow checker garanisce, in fase di compilazione, che  dato un valore di ipo T in
ogni momento valgano i segueni invariani, mutuamente esclusivi
- Non esista alcun riferimento al valore al di là del suo possessore 
- Esistano uno o più riferimeni immutabili (&T) - aliasing 
- Esista un solo riferimento mutabile (&mut T) - mutabilità

Cell non permette di ottenere riferimenti all'oggetto contenuto. Qui interviene l'oggetto `RefCell<T>` che è un contenitore simile a Cell ma contiene anche un flag che indica se sono stati creati dei riferimenti a quel dato.

- Il flag chiamato `borrow` è costituito da un numero incrementabile nel caso dei riferimenti semplici e da un numero specifico per il riferimento mutabile, 
- `RefCell<T>` occupa quindi lo spazio di `T` più un `Cell<usize>`

Viene creato un panic nel caso in cui si chieda la creazione dei riferimenti, tramite il metodo **borrow**, quando esiste già un riferimento mutabile.
Esiste anche **try_borrow** che non genera panic ma restituisce un errore nel caso in cui esista già un riferimento mutabile.

# Domanda 3
**Punteggio 3,0**

#### In un programma che utilizza una sincronizzazione basata su Condition Variable, è possibile che alcune notifiche vengano perse? Se sì, perché? In entrambi i casi si produca un esempio di codice che giustifichi la risposta.
## Risposta

Sì, è possibile che alcune notifiche vengano perse in un programma che utilizza una sincronizzazione basata su Condition Variable. Questo può accadere se un thread invia una notifica prima che l'altro thread inizi a mettersi in attesa. Vediamo come può succedere e come evitarlo.
```rust
// Primo thread
{
    let mut mutex = mutex.lock().unwrap();
    mutex = cv.wait(mutex).unwrap();
}

// Secondo thread
{
    cv.notify_one();
}
```

Questo comportamento va assolutamente evitato, in quanto porta ad errori casuali (dipendenti dalla politica attuale di scheduling, dal carico), e Rust offre un opportuno costrutto sintattico per risolvere il problema:

```rust
// Primo thread
{
    let mut mutex = mutex.lock().unwrap();
    mutex = cv.wait_while(mutex, |m| condition(m)).unwrap();
}

// Secondo thread
{
    cv.notify_one();
}
```

In questo modo, se l'azione che dovrebbe notificare il risveglio è stata già fatta, il thread non si mette in attesa.

Alternativamente si può utilizzare un ciclo while con una condizione. Questo garantisce che il thread controlli la condizione ogni volta che si risveglia, evitando di rimanere bloccato se la notifica è già stata inviata.

```rust
let mut mutex = mutex.lock().unwrap();
while (condition(*mutex)) {
    mutex = cv.wait(mutex).unwrap();
}
```

**Commento:**
- Ok

# Domanda 4
**Punteggio: 6,0**

#### In un sistema concorrente, ciascun thread può pubblicare eventi per rendere noto ad altri thread quanto sta facendo. Per evitare un accoppiamento stretto tra mittenti e destinatari degli eventi, si utilizza un Dispatcher. Questo è un oggetto thread-safe che offre il metodo `dispatch(msg: Msg)` mediante il quale un messaggio di tipo generico `Msg` (soggetto al vincolo di essere clonabile) viene reso disponibile a chiunque si sia sottoscritto.

Un thread interessato a ricevere messaggi può invocare il metodo `subscribe()` del Dispatcher: otterrà come risultato un oggetto di tipo `Subscription` mediante il quale potrà leggere i messaggi che da ora in poi saranno pubblicati attraverso il Dispatcher. Per ogni sottoscrizione attiva, il Dispatcher mantiene internamente l'equivalente di una coda ordinata (FIFO) di messaggi non ancora letti. A fronte dell'invocazione del metodo `dispatch(msg: Msg)`, il messaggio viene clonato ed inserito in ciascuna delle code esistenti.

L'oggetto `Subscription` offre il metodo bloccante `read() -> Option<Msg>`. Se nella coda corrispondente è presente almeno un messaggio, questo viene rimosso e restituito; se nella coda non è presente nessun messaggio e il Dispatcher esiste ancora, l'invocazione si blocca fino a che non viene inserito un nuovo messaggio; se invece il Dispatcher è stato distrutto, viene restituito il valore corrispondente all'opzione vuota.

Gli oggetti `Dispatcher` e `Subscription` sono in qualche modo collegati, ma devono poter avere cicli di vita indipendenti: la distruzione del Dispatcher non deve impedire la consumazione dei messaggi già recapitati ad una `Subscription`, ma non ancora letti; parimenti, la distruzione di una `Subscription` non deve impedire al Dispatcher di consegnare ulteriori messaggi alle eventuali altre `Subscription` presenti.
