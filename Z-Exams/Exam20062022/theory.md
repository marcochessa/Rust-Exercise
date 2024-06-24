# Domanda 1
Punteggio 3,00

#### Si definisca il concetto di Smart Pointer, quindi si fornisca un esempio (Rust) che ne evidenzi il ciclo di vita.
## Risposta

### Smart Pointer: Definizione ed Esempio in Rust

Uno **smart pointer** è un tipo di dato che, dal punto di vista sintattico, "sembra" un puntatore, ma possiede caratteristiche aggiuntive rispetto ai puntatori "nativi". Queste caratteristiche includono:

* **Garanzia di inizializzazione e rilascio:** Ciò significa che gli smart pointer si occupano automaticamente di allocare e deallocare la memoria, evitando errori comuni come memory leak.
* **Conteggio dei riferimenti:** Alcuni smart pointer tengono traccia di quanti riferimenti puntano a un blocco di memoria, rilasciando automaticamente la memoria quando non ci sono più riferimenti attivi.
* **Accesso esclusivo con atesa:** Alcuni smart pointer garantiscono che solo un thread alla volta possa accedere a un determinato blocco di memoria, evitando problemi di concorrenza.

In Rust, l'uso degli smart pointer è fondamentale per la creazione di strutture dati dinamiche come grafi, alberi e liste. Grazie agli smart pointer come `Rc<T>` e `Arc<T>`, è possibile avere più "proprietari" di uno stesso valore.

Ecco un esempio di smart pointer in Rust, `std::Box<T>`, e il suo ciclo di vita:

**Definizione:**

* `std::Box<T>` è una struttura che incapsula un puntatore a un blocco di memoria allocato dinamicamente sullo heap al momento della sua creazione tramite il metodo `Box::new(t)`.
* Il dato puntato è posseduto da `Box`: quando la struttura esce dal proprio scope sintattico, il blocco sullo heap viene rilasciato automaticamente grazie all'implementazione del tratto `Drop`.

**Ciclo di vita:**

1. **Creazione:** Un `Box<T>` viene creato usando `Box::new(t)`, allocando memoria sullo heap e spostando il valore `t` al suo interno.
2. **Utilizzo:** Si può accedere al valore contenuto nel `Box` usando l'operatore di dereferenziazione `*` o il metodo `borrow()`.
3. **Movimento:**  Il `Box` può essere spostato in un'altra variabile, trasferendo la proprietà del dato puntato.
4. **Rilascio:** Quando il `Box` esce dallo scope, viene eseguito il suo metodo `drop()`, che dealloca la memoria allocata sullo heap.

**Esempio:**

```rust
fn produce(odd: bool) -> Box<i32> {
    let mut b = Box::new(0); // Creazione del Box
    if odd { *b = 5; } 
    return b; // Movimento del Box
}

fn main() {
    let b1 = produce(false); // b1 possiede il Box
    println!("b1: {}", b1);
    let b2 = produce(true); // b2 possiede il Box
    drop(b1); // Rilascio anticipato del Box posseduto da b1
    println!("b2: {}", b2); 
} // b2 esce dallo scope e il Box viene rilasciato
```

In questo esempio, la funzione `produce()` crea un `Box<i32>` e lo restituisce. Il `Box` viene quindi spostato alle variabili `b1` e `b2` nel `main()`. Quando `b1` esce dallo scope, il `Box` viene rilasciato anticipatamente da `drop(b1)`. Alla fine del `main()`, anche `b2` esce dallo scope, rilasciando l'ultimo riferimento al `Box` e deallocando la memoria sullo heap.

Oltre a `Box<T>`, Rust offre una varietà di smart pointer per coprire ulteriori casi d'uso, come `Rc<T>`, `Arc<T>`, `Cell<T>`, `RefCell<T>`, `Cow<T>`, `Mutex<T>` e `RwLock<T>`.

# Domanda 2
Punteggio 3,00

#### Si illustrino le differenze nel linguaggio Rust tra std::channel() e std::sync_channel(), indicando quali tipi di sincronizzazione i due meccanismi permettono.

## Risposta

In Rust, `std::sync::mpsc::channel()` e `std::sync::mpsc::sync_channel()` sono due funzioni che creano canali di comunicazione tra thread, ma differiscono nel modo in cui gestiscono la sincronizzazione e la capacità dei canali. 

Ecco un'illustrazione delle loro differenze e i tipi di sincronizzazione che offrono:

### `std::sync::mpsc::channel()`

`std::sync::mpsc::channel()` crea un canale asincrono (o senza blocchi) illimitato.

- **Tipo di canale**: Asincrono
- **Capacità**: Illimitata, può contenere un numero arbitrario di messaggi.
- **Sincronizzazione**: I messaggi inviati non bloccano il thread mittente fintanto che c'è memoria disponibile per allocare nuovi messaggi. Il ricevitore può leggere i messaggi a suo piacimento.
- La funzione `std::sync::mpsc::channel<T>()` resituisce una coppia ordinata formata da
una struct `Sender<T>` ed una struct `Receiver<T>` 
  - Tutti i dati inviati tramite il metodo send(...) possono essere consumati attraverso il
  metodo recv(), nello stesso ordine in cui sono stai inviati 
  - Il metodo send(...) offre la garanzia che chi lo invoca non sarà bloccato (ovvero il canale di
  comunicazione ha una capacità ininita di memorizzazione temporanea dei messaggi) 
  - Il metodo recv() si blocca senza consumare cicli macchina in atesa di un messaggio o della
  terminazione dell’oggeto Sender e di tutti i suoi eventuali cloni
- **multiple producer - single consumer** permette di creare più cloni dell’oggeto sender mentre obbliga ad avere una singola copia dell’oggeto receiver

Questa operazione agisce al tempo stesso da sincronizzazione (la ricezione è necessariamente successiva all’invio) e da comunicazione (il dato passato rappresenta l’unità di messaggio)


#### Utilizzo

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..5 {
            tx.send(i).unwrap();
            println!("Sent {}", i);
        }
    });

    thread::sleep(Duration::from_secs(1));
    for received in rx {
        println!("Received {}", received);
    }
}
```

### `std::sync::mpsc::sync_channel()`

`std::sync::mpsc::sync_channel()` crea un canale sincrono (o bloccante) con una capacità fissa.

- **Tipo di canale**: Sincrono
- **Capacità**: Limitata, specificata dall'utente al momento della creazione del canale.
- **Sincronizzazione**: Il mittente viene bloccato se il canale ha raggiunto la sua capacità e deve aspettare che il ricevitore consuma un messaggio prima di poter inviarne uno nuovo. Questo impone una sincronizzazione stretta tra mittente e ricevitore.

Se viene costruito un canale sincrono di dimensione 0, diventa un canale di ipo
**rendezvous**: ogni operazione di letura deve sovrapporsi temporalmente ad una di
scritura
- Le restani operazioni oferte da SyncSender<T> hanno semanica simile alle corrispondeni oferte
da Sender<T>

#### Utilizzo

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::sync_channel(2); // Capacità di 2 messaggi

    thread::spawn(move || {
        for i in 1..5 {
            tx.send(i).unwrap();
            println!("Sent {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::sleep(Duration::from_secs(1));
    for received in rx {
        println!("Received {}", received);
        thread::sleep(Duration::from_secs(1));
    }
}
```
# Domanda 3
Punteggio 3,00

#### Dato il seguente frammento di codice Rust (ogni linea è preceduta dal suo indice):
```rust
1. struct Point {
2.    x: i16,
3.    y: i16,
4. }
5.
6. enum PathCommand {
7.    Move(Point),
8.    Line(Point),
9.    Close,
10. }
11. let mut v = Vec::<PathCommand>::new();
12. v.push(PathCommand::Move(Point{x:1,y:1}));
13. v.push(PathCommand::Line(Point{x:10, y:20}));
14. v.push(PathCommand::Close);
15. let slice = &v[..];
```
Si descriva il contenuto dello stack e dello heap al termine dell'esecuzione della riga 15.

## Risposta
Si assume un'architettura da 64 bit quindi avremo puntatori da 8 Byte ciascuno.

#### Stack

- **Variabile `v` (`Vec<PathCommand>`)**:
    - Puntatore al primo elemento nell'heap: `0x...` (8 byte)
    - Size: `3` (8 byte)
    - Capacity: `3` (8 byte)
    - **Totale**: 24 byte

- **Variabile `slice` (&[PathCommand])**:
    - Puntatore al primo elemento nell'heap: stesso puntatore di `v` (8 byte)
    - Size: `3` (8 byte)
    - **Totale**: 16 byte

#### Heap

Il heap contiene gli elementi del vettore `v`:

- **3 enum `PathCommand`**:
    - Ogni enum ha una dimensione di `(1 byte + 32 bit)` = 5 byte
    - **Totale per i 3 enum**: `5 * 3` = 15 byte

  Descrizione dei singoli enum:
    - **`PathCommand::Move(Point { x: 1, y: 1 })`**:
        - Tag: `0` (1 byte)
        - x: `1` (2 byte)
        - y: `1` (2 byte)
    - **`PathCommand::Line(Point { x: 10, y: 20 })`**:
        - Tag: `1` (1 byte)
        - x: `10` (2 byte)
        - y: `20` (2 byte)
    - **`PathCommand::Close`**:
        - Tag: `2` (1 byte)
        - x: Non usato (`0` o irrilevante) (2 byte)
        - y: Non usato (`0` o irrilevante) (2 byte)

### Totale della memoria occupata

- **Stack**: 24 byte (per `v`) + 16 byte (per `slice`) = 40 byte
- **Heap**: 15 byte


# Domanda 4
Punteggio 6,00

#### Un paradigma frequentemente usato nei sistemi reattivi è costituito dall'astrazione detta Looper. Quando viene creato, un Looper crea una coda di oggetti generici di tipo Message ed un thread. Il thread attende - senza consumare cicli di CPU - che siano presenti messaggi nella coda, li estrae uno a uno nell'ordine di arrivo, e li elabora. Il costruttore di Looper riceve due parametri, entrambi di tipo (puntatore a) funzione: `process(...)` e `cleanup()`. La prima è una funzione responsabile di elaborare i singoli messaggi ricevuti attraverso la coda; tale funzione accetta un unico parametro in ingresso di tipo Message e non ritorna nulla; la seconda è una funzione priva di argomenti e valore di ritorno e verrà invocata dal thread incapsulato nel Looper quando esso starà per terminare.

Looper offre un unico metodo pubblico, thread safe, oltre a quelli di servizio, necessari per gestirne il ciclo di vita: `send(msg)`, che accetta come parametro un oggetto generico di tipo Message che verrà inserito nella coda e successivamente estratto dal thread ed inoltrato alla funzione di elaborazione. Quando un oggetto Looper viene distrutto, occorre fare in modo che il thread contenuto al suo interno invochi la seconda funzione passata nel costruttore e poi termini.

Si implementi, utilizzando il linguaggio Rust o C++, tale astrazione tenendo conto che i suoi metodi dovranno essere thread-safe.

