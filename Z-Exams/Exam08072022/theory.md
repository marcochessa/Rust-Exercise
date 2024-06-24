# Domanda 1
**Punteggio 3,00**

#### Si definiscano i concetti di Dangling Pointer, Memory Leakage e Wild Pointer, facendo esempi concreti, usando dello pseudocodice, che possono generare questi fenomeni.

## Risposta
### Concetti di Dangling Pointer, Memory Leakage e Wild Pointer

#### Dangling Pointer

Un dangling pointer (puntatore sospeso) si verifica quando un puntatore continua a riferirsi a una memoria che è stata deallocata. Tentare di accedere a tale memoria porta a comportamenti indefiniti.

**Esempio**:

```pseudocode
ptr = allocate_memory(100)  // Allocare 100 byte di memoria
free(ptr)                   // Deallocare la memoria
data = ptr[0]               // Accesso a memoria deallocata (dangling pointer)
```
Rust previene questo problema con la gestione della proprietà e il controllo del ciclo di vita degli oggetti.

#### Memory Leakage

La memory leakage (perdita di memoria) si verifica quando un programma alloca memoria ma non la dealloca correttamente, causando l'esaurimento della memoria disponibile.

**Esempio**:

```pseudocode
while (true) {
    ptr = allocate_memory(100)  // Allocare memoria in un ciclo infinito
    // Nessuna chiamata a free(ptr) - perdita di memoria
}
```
Rust gestisce automaticamente la memoria attraverso il sistema di proprietà
#### Wild Pointer

Un wild pointer (puntatore selvaggio) è un puntatore che non è stato inizializzato correttamente e punta a una posizione di memoria arbitraria. Utilizzare un wild pointer può portare a comportamenti imprevedibili.

**Esempio**:

```pseudocode
ptr           // Dichiarazione di un puntatore senza inizializzazione
data = ptr[0] // Accesso a memoria arbitraria (wild pointer)
```
Rust impedisce l'uso di wild pointers grazie al sistema di proprietà e borrowing

# Domanda 2
**Punteggio 3,00**

#### In relazione al concetto di Atomic, si definisca cosa esso mira a garantire, come tale garanzia possa essere fornite a livello architetturale, e quali siano i suoi limiti.

## Risposta

Il concetto di Atomic in Rust mira a garantire la **sicurezza dei dati** quando questi sono condivisi tra più thread. Questa garanzia viene fornita a livello architetturale attraverso l'utilizzo di **operazioni atomiche**, ovvero operazioni che vengono eseguite in un'unica unità di tempo indivisibile dal processore. Questo significa che, durante l'esecuzione di un'operazione atomica, nessun altro thread può accedere o modificare il dato in questione, evitando così race conditions e altri problemi di concorrenza.

**Come viene fornita la garanzia a livello architetturale:**

*   Le operazioni atomiche sono implementate direttamente a livello hardware dalla CPU.
*   Rust utilizza istruzioni specifiche del processore per garantire l'atomicità delle operazioni.

**Limitazioni:**

*   I tipi atomici in Rust non offrono meccanismi di condivisione esplicita, sono soggetti alla regola del possessore unico come tutti gli altri valori in Rust.
*   Per permettere l'accesso a più thread, è necessario incapsularli in un `Arc` o dichiararli come variabili globali con `static`.
*   Sebbene le operazioni atomiche siano thread-safe, non risolvono tutti i problemi di concorrenza. Ad esempio, è ancora possibile incorrere in deadlock o starvation se non si utilizzano correttamente.


# Domanda 3
**Punteggio 3,00**

#### All'interno di un programma è definita la seguente struttura dati:

```rust
struct Bucket {
    data: Vec<i32>, 
    threshold: Option<i32>
}
```

Usando il debugger si è determinato che, per una istanza di `Bucket`, essa è memorizzata all'indirizzo 0x00006000014ed2c0. Osservando la memoria presente a tale indirizzo, viene mostrato il seguente contenuto (per blocchi di 32 bit):
```
308a6e01 00600000 03000000 00000000 03000000 00000000 01000000 0a000000
```
Cosa è possibile dedurre relativamente ai valori contenuti dei vari campi della singola istanza?

## Risposta
### Interpretazione della Memoria

Dobbiamo interpretare ogni blocco di 32 bit (4 byte) individualmente, considerando l'endianness.

1. `308a6e01` (little-endian) -> `0x016e8a30`
2. `00600000` (little-endian) -> `0x00006000`
3. `03000000` (little-endian) -> `0x00000003`
4. `00000000` (little-endian) -> `0x00000000`
5. `03000000` (little-endian) -> `0x00000003`
6. `00000000` (little-endian) -> `0x00000000`
7. `01000000` (little-endian) -> `0x00000001`
8. `0a000000` (little-endian) -> `0x0000000a`

### Mappatura dei Blocchi

#### `Vec<i32>`

Il `Vec<i32>` è rappresentato in memoria da un puntatore ai dati, una capacità e una lunghezza.

- **Puntatore ai dati**:
    - Parte bassa: `0x016e8a30`
    - Parte alta: `0x00006000`
    - Combinato: `0x00006000016e8a30`

- **Capacità del vettore**:
    - `0x00000003`

- **Lunghezza del vettore**:
    - `0x00000003`

#### `Option<i32>`

L'`Option<i32>` è rappresentato in memoria come un discriminante seguito dal valore.

- **Discriminante**:
    - `0x00000001` (indica `Some`)

- **Valore**:
    - `0x0000000a`

### Deduzione dei Valori

Per la struttura `Bucket`, all'indirizzo specificato, possiamo dedurre:

- **data** (`Vec<i32>`):
    - Puntatore ai dati: `0x00006000016e8a30`
    - Capacità: `3`
    - Lunghezza: `3`

- **threshold** (`Option<i32>`):
    - `Some(10)` (poiché il discriminante è `1` e il valore è `10`)

# Domanda 4
**Punteggio 6,00**

#### All'interno di un programma è necessario garantire che non vengano eseguite CONTEMPORANEAMENTE più di N invocazioni di operazioni potenzialmente lente. A questo scopo, è stata definita la struttura dati `ExecutionLimiter` che viene inizializzata con il valore N del limite. Tale struttura è thread-safe e offre solo il metodo pubblico generico `execute(f)`, che accetta come unico parametro una funzione `f`, priva di parametri che ritorna il tipo generico `R`. Il metodo `execute(...)` ha, come tipo di ritorno, lo stesso tipo `R` restituito da `f` ed ha il compito di mantenere il conteggio di quante invocazioni sono in corso. Se tale numero è già pari al valore N definito all'atto della costruzione della struttura dati, attende, senza provocare consumo di CPU, che scenda sotto soglia, dopodiché invoca la funzione `f` ricevuta come parametro e ne restituisce il valore. Poiché l'esecuzione della funzione `f` potrebbe fallire, in tale caso, si preveda di decrementare il conteggio correttamente. Si implementi, usando i linguaggi Rust.
