# Domanda 1
**Punteggio 3,0**

#### Attraverso un esempio pratico si illustri l’utilizzo dei Mutex nel linguaggio Rust. Qual è il meccanismo che consente il loro utilizzo in un contesto thread-safe?

## Risposta

### Esempio pratico di utilizzo dei `Mutex`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Creazione di un counter condiviso tra i thread, protetto da un Mutex e gestito da Arc
    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    let N = 10;

    for t in 0..N {
        let thread_counter = Arc::clone(&counter);
        threads.push(
            thread::spawn(move || {
                // Il `mut` è necessario per aggiornare il counter
                let mut counter_guard = thread_counter.lock().unwrap();
                *counter_guard += 1;
                // Il guard viene droppato, rilasciando il lock
            })
        );
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}
```

### Meccanismo che consente il loro utilizzo in un contesto thread-safe

Il meccanismo che permette un utilizzo sicuro dei `Mutex` è composto da più parti:

1. **Arc (Atomic Reference Counted)**:
   - I `Mutex` sono condivisi dai thread tramite `Arc`, che sono degli smart pointer con le seguenti proprietà:
   - Tengono traccia dei riferimenti (strong e weak) al dato a cui si riferiscono.
   - Sono thread safe: gli aggiornamenti dei contatori sono effettuati con operazioni atomiche su tipi atomici.
   - Implementano il paradigma RAII: quanto l'ultimo riferimento è droppato, la memoria viene rilasciata.

2. **Mutex**:
   - Incapsulando un `Mutex` in un `Arc`, possiamo condividere in modo sicuro un `Mutex` tra più thread.
   - Il `Mutex` garantisce che solo un thread alla volta possa accedere alla risorsa incapsulata, prevenendo così le race condition.

In sintesi, utilizzando `Arc` e `Mutex`, Rust consente di gestire in modo sicuro la concorrenza tra thread, garantendo che le risorse condivise siano accessibili in modo sicuro e sincronizzato.

---

# Domanda 2
**Punteggio 3,0**

#### Si descriva il concetto di "ownership" in Rust e come contribuisca a prevenire errori come le race condition e i dangling pointer rispetto ad altri linguaggi come C++.

## Risposta

### Concetto di "ownership" in Rust
Ownership:
1. Si ha un solo proprietario per valore:
   - Ogni valore ha un solo possessore (questo può variare durante la vita del valore).
   - Solo il proprietario può modificare il valore o trasferire la proprietà ad altri.
   - In Rust i dati vengono copiati solamente se implementano il tratto `Copy`, altrimenti ogni assegnazione è uno spostamento. La semantica di spostamento permette di associare ad ogni dato un solo possessore. 
   - Il possessore è anche incaricato di liberare la memoria quando non sarà più richiesta.

2. Borrowing e riferimenti:
   - Un valore può avere più riferimenti non mutabili o un solo riferimento mutabile.
   - Il borrow checker verifica che i riferimenti siano validi e non causino race conditions o dangling pointer.

### Prevenzione di errori

1. **Dangling pointer**:
   - I dangling pointer vengono evitati perché il borrow checker verifica che il dato puntato esista ancora quando viene utilizzato. Se il pointer viene utilizzato fuori dal lifetime del dato puntato, il programma non verrà compilato.

2. **Race condition**:
   - In un contesto concorrente, le race condition possono essere evitate in quanto eventuali primitive di sincronizzazione tra thread potranno essere possedute da un thread alla volta. Rust garantisce che non ci siano accessi simultanei a risorse condivise senza opportune sincronizzazioni, grazie alle regole di borrowing e ownership.

---

# Domanda 3
**Punteggio 3,0**

#### Si dimostri come sia possibile implementare il polimorfismo attraverso i tratti. Si fornisca anche un esempio concreto che faccia riferimento ad almeno due strutture diverse.

## Risposta

### Polimorfismo attraverso i tratti

I tratti in Rust sono utilizzati per indicare che un tipo supporta un certo tipo di metodi definiti dal tratto. Ad esempio, il tratto `Iterator` richiede il metodo `next(&self) -> Option<Self::ItemType>`. Se un tipo scritto dall'utente volesse essere marcato come `Iterator` dovrebbe implementare questo metodo.

Per poter implementare il polimorfismo vero e proprio in Rust, si dovrebbe ricorrere alla keyword `dyn`. `dyn Trait` può essere utilizzato come placeholder al posto del tipo vero e proprio, e sta ad indicare che l'argomento di quella funzione dovrà essere un tipo che implementi quel tratto. I `dyn` sono composti sia da un puntatore al dato che alla sua vtable (puntatori alle implementazioni delle funzioni del tratto per quel dato).

### Esempio concreto

#### Definizione del Tratto

```rust
trait Author {
    fn print(&self, s: String);
}
```

#### Implementazione del Tratto per `BookAuthor`

```rust
struct BookAuthor;

impl Author for BookAuthor {
    fn print(&self, s: String) {
        println!("{} by a BookAuthor", s);
    }
}
```

#### Implementazione del Tratto per `ArticleAuthor`

```rust
struct ArticleAuthor;

impl Author for ArticleAuthor {
    fn print(&self, s: String) {
        println!("{} by an ArticleAuthor", s);
    }
}
```

#### Funzione che utilizza il Tratto

```rust
fn get_author<A: Author>(author: A) -> A {
    author
}

fn main() {
    let book_author = BookAuthor;
    let article_author = ArticleAuthor;

   get_author(book_author).print("Hello".to_string());
   get_author(article_author).print("Hello".to_string());
}
```

# Domanda 4
**Punteggio 6,0**

Una cache è una struttura dati, generica, thread-safe che consente di memorizzare coppie chiave/valore per un periodo non superiore ad una durata stabilita per ciascuna coppia. Nell'intervallo di validità associato alla coppia, richieste di lettura basate sulla chiave restituiscono il valore corrispondente, se presente. Trascorso tale periodo, eventuali richieste relative alla stessa chiave non restituiscono più il valore.

Poiché il numero di richieste in lettura può essere molto maggiore delle richieste in scrittura, è necessario fare in modo che le prime possano sovrapporsi temporalmente tra loro, mentre le seconde dovranno necessariamente essere con accesso esclusivo. Per evitare la saturazione della struttura, quando si eseguono operazioni di scrittura, si provveda ad eseguire un ciclo di pulizia, eliminando le eventuali coppie scadute.

#### Si implementi in Rust la struct `Cache<K: Eq + Hash, V>` dotata dei seguenti metodi:

```rust
pub fn new() -> Self
// Crea una nuova istanza

pub fn size(&self) -> usize
// Restituisce il numero di coppie presenti nella mappa

pub fn put(&self, k: K, v: V, d: Duration) -> ()
// Inserisce la coppia k/v con durata pari a d

pub fn renew(&self, k: &K, d: Duration) -> bool
// Rinnova la durata dell'elemento rappresentato dalla chiave k; restituisce true se la chiave esiste e non è scaduta, altrimenti restituisce false

pub fn get(&self, k: &K) -> Option<Arc<V>>
// Restituisce None se la chiave k è scaduta o non è presente nella cache; altrimenti restituisce Some(a), dove a è di tipo Arc<V>
```

Si ricordi che `Duration` è una struttura contenuta in `std::time`, che rappresenta una durata non negativa. Può essere sommato ad un valore di tipo `std::time::Instant` 
(che rappresenta un momento specifico nel tempo) per dare origine ad un nuovo `Instant`, collocato più avanti nel tempo.

## Risposta
Vedere **es4.rs**