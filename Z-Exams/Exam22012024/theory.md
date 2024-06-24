# Domanda 1
**Punteggio 3,0**

Sia data la struttura LinkedList<T> definita come:
```rust
pub struct LinkedList<'a, T> {
    pub val: Option<T>,
    pub next: Option<&'a Box<LinkedList<'a, T>>>,
}
```

#### Si definisca l’occupazione di memoria di un elemento della lista e si indichi come sia possibile definire il fine lista.

## Risposta
### Analisi dell'occupazione di memoria

1. **Campo `val: Option<T>`**
    - `Option<T>` contiene il valore `T` e un discriminante.
    - **Esempio con `T = i32`:**
        - `size_of::<i32>() = 4` byte.
        - `Option<i32>` occupa `4 (i32) + 1 (discriminante) = 5` byte.

2. **Campo `next: Option<&'a Box<LinkedList<'a, T>>>`**
    - `Option<&'a Box<LinkedList<'a, T>>>` è un'opzione che contiene un riferimento a un `Box`.
    - **Nota:** Un riferimento `&T` è solitamente 8 byte su una architettura a 64 bit.
      - **Extra** `Box<LinkedList<'a, T>>` è un puntatore a un heap-allocated `LinkedList` che è sized, quindi occupa anch'esso 8 byte.
    - **Quindi:** `Option<&Box<LinkedList<T>>>` in effetti contiene solo un puntatore (8 byte) e non aggiunge ulteriore spazio.

### Sommario

- **`val`:** `1` byte (discriminante o tag) + `4` byte (`i32`) = `5` byte.
- **`next`:** `8` byte (riferimento a `Box<LinkedList<T>>`).

Quindi, per un elemento della lista:

- **Totale:** `5 (val) + 8 (next) = 13 byte`.
- A 64 bit, i puntatori sono allineati a 8 byte, quindi gli oggetti potrebbero avere padding per allinearsi correttamente.
- Quindi con **allineamento**: `16 byte`

---

# Domanda 2
**Punteggio 3,0**

#### Si definisca un esempio in cui, data la necessità di creare N thread, si possano evitare race-conditions nel momento in cui i thread debbano accedere in scrittura alla stessa risorsa. Si distingua il caso in cui tale risorsa sia uno scalare e quella in cui sia una struttura più articolata.

## Risposta

In un programma multi-thread, in assenza di costrutti di sincronizzazione, si possono verificare delle race-conditions quando più thread cercano di accedere ad una stessa risorsa modificandone il valore. Nel caso di due thread, ad esempio, il valore finale della risorsa condivisa può essere quello scritto dal primo thread, dal secondo oppure un valore completamente arbitrario (questo fenomeno è noto come interferenza), e in ogni caso non è possibile prevederne il contenuto, essendo l'esecuzione non deterministica.

Per questo motivo occorre proteggere le risorse condivise tra più thread con opportuni costrutti di sincronizzazione. Per far ciò si può ricorrere all'uso di `Mutex` (mutual exclusion lock), che sono delle strutture che "incapsulano" la struttura dati condivisa permettendo l'accesso solo ad un thread alla volta: per accedere ad una risorsa occorre prima aver ottenuto il possesso del suo `mutex` tramite il metodo `lock()`. Una volta ottenuto il lock, un altro thread che cerchi di ottenerne il possesso è costretto ad aspettare finché il lock non viene liberato dal possessore attuale: in questo modo vengono evitati una serie di problemi di sincronizzazione tra più thread. E' opportuno che il thread possessore del lock lo rilasci prima della sua terminazione (per evitare situazioni di deadlock), anche in caso di terminazione con errore. Ciò in Rust è derivato dall'adozione del paradigma RAII: quando un thread esce dal suo scope sintattico (o termina con errore), la contrazione dello stack determina il rilascio di tutte le risorse, ed il `Mutex` (essendo di fatto uno smart pointer composto da un puntatore ad un mutex nativo del sistema operativo, un campo poison che indica se il thread sia terminato correttamente o meno, ed il dato "incapsulato" dal mutex stesso) rilascia automaticamente le risorse acquisite, evitando deadlock.

Come detto, per strutture articolate si ricorre all'utilizzo di `mutex`. Per risorse più "semplici" (come uno scalare), Rust, oltre ai già citati `Mutex`, mette a disposizione dei particolari tipi, chiamati `Atomic`. Possono essercene diversi per i tipi più semplici (ad esempio `AtomicBool`, `AtomicUsize`...), e garantiscono che le operazioni che vengono effettuate su questi tipi siano atomiche (cioè indivisibili), appoggiandosi internamente ad istruzioni di tipo `fence` o `barrier` offerte dai processori, garantendo quindi sincronizzazione tra thread diversi.


### Risorsa: Scalare
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let counter = AtomicUsize::new(0);
    let handles: Vec<_> = (0..10).map(|_| {
        let counter = &counter;
        thread::spawn(move || {
            for _ in 0..10 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value is {}", counter.load(Ordering::SeqCst));
}
```

### Risorsa: Struttura
```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

fn main() {
// Un HashMap condiviso tra i thread
let shared_map = Arc::new(Mutex::new(HashMap::new()));
let mut handles = vec![];

    for i in 0..10 {
        let map = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            let mut map = map.lock().unwrap();
            map.insert(i, i * 10);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let map = shared_map.lock().unwrap();
    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }
}
```

---

# Domanda 3
**Punteggio 3,0**

#### Si usi un esempio concreto per dimostrare la capacità di introdurre modularità attraverso i Tratti.

## Risposta

Un tratto in Rust può essere visto come un'interfaccia descritta nel linguaggio Java. Un tratto cioè definisce una serie di metodi che gli oggetti che lo implementano dovranno implementare (possono anche adottare l'implementazione di default a patto che sia definita). A differenza di altri linguaggi, normalmente l'utilizzo di un tratto non comporta costi aggiuntivi: se il compilatore conosce il valore del tipo che implementa il tratto non è necessario il passaggio attraverso la vtable (che contiene tutte le implementazioni dei metodi del tratto), con conseguente costo in termini di memoria e tempo. Se invece si possiede un riferimento ad un valore che implementa un tratto, è necessario ricorrere agli oggetti-tratto, che sono formati dal puntatore al valore del tipo e da un puntatore alla vtable (in questo caso vi è quindi il costo aggiuntivo della vtable).

I tratti di per sè non possono ereditare da altri tratti (in Rust infatti non vi è un supporto specifico all'ereditarietà), ma si possono creare relazioni di "parentela" tra i diversi tratti. Ad esempio, tutti i tipi che implementano il tratto `Copy` (che vengono definiti quindi copiabili, come i tipi elementari: scalari, booleani...) devono per forza implementare anche il tratto `Clone`. Inoltre, vi possono essere dei tratti che sono vicendevolmente esclusivi: ad esempio, un tipo che implementa il tratto `Copy` (che quindi non rappresenta una "risorsa" che richiede ulteriori azioni al momento del rilascio) è mutuamente esclusivo con il tratto `Drop`, che invece definisce il comportamento di una risorsa quando viene rilasciata.

Vi sono poi dei particolari tratti, chiamati tratti marker, che non contengono metodi, ma descrivono il comportamento assunto da un tipo in particolari situazioni. Sono un esempio i tratti della concorrenza, `Send` e `Sync`. Se un tipo che implementa un tratto `Send` è possibile trasferirlo in maniera sicura da un thread all'altro (cioè garantisce che non siano possibili accessi contemporanei al suo valore), mentre se un tipo implementa un tratto `Sync` è possibile condividere riferimenti non mutabili tra thread diversi. Ci sono tipi, come `Cell` e `Refcell`, che, implementando un pattern di interior mutability, non godono del tratto `Sync`, pur implementando il tratto `Send`. I tipo che implementano il tratto `Sync` sono dunque un sottoinsieme di quelli che implementano il tratto `Send`.

E' possibile, infine, che un dato tratto "derivi" da un altro tratto (subtrait o supertrait), estendendone il comportamento o ridefinendo l'implementazione di alcuni metodi (diversi tipi, a seconda che implementino il tratto o il sotto-tratto, avranno quindi comportamenti diversi).

### Esempio concreto definizione del Tratto

Prima, definiamo un tratto `Shape` che dichiara un metodo `area`.

```rust
trait Shape {
    fn area(&self) -> f64;
}
```

### Implementazione del Tratto per Cerchi

Ora implementiamo il tratto `Shape` per un tipo `Circle`.

```rust
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
```

### Implementazione del Tratto per Rettangoli

Implementiamo anche il tratto `Shape` per un tipo `Rectangle`.

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

# Domanda 4

#### La struttura `MultiChannel` implementa il concetto di canale con molti mittenti e molti ricevitori.
#### I messaggi inviati a questo tipo di canale sono composti da singoli byte che vengono recapitati a tutti i ricevitori attualmente collegati.

Riferimenti a tipi:
```rust
use std::result::Result;
use std::sync::mpsc::{Receiver, SendError};
```

**Metodi:**
```rust
new() -> Self
// crea un nuovo canale senza alcun ricevitore collegato
subscribe(&self) -> Receiver<u8>
// collega un nuovo ricevitore al canale: da quando
// questo metodo viene invocato, gli eventuali byte // inviati al canale saranno recapitati al ricevitore. // Se il ricevitore viene eliminato, il canale
// continuerà a funzionare inviando i propri dati
// ai ricevitori restanti (se presenti), altrimenti // ritornerà un errore
send(&self, data: u8) -> Result<(), SendError<u8>> // invia a tutti i sottoscrittori un byte
// se non c'è alcun sottoscrittore, notifica l'errore // indicando il byte che non è stato trasmesso
```

## Risposta 
Vedere **es4.rs**