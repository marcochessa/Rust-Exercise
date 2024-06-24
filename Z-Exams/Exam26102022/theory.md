# Domanda 1
**Punteggio 3,0**

#### Si illustrino le differenze tra stack e heap. Insieme alle differenze, indicare per i seguenti costrutti Rust, in modo dettagliato, dove si trovano i dati che li compongono: Box<[T]>, RefCell<T> e &[T].

## Risposta

Lo stack è utilizzato per allocare variabili locali e contiene i valori per i passaggi a funzione e i valori di ritorno delle funzioni. La sua dimensione è nota a compile time, ma le variabili sono allocate a runtime. Ogni thread ha il proprio stack, mentre l'heap è condiviso dal processo. 

Quando si esce da una funzione, lo stack si contrae. La differenza principale tra stack e heap è che lo heap viene utilizzato per allocare dati dinamici a runtime, con una durata che può superare quella del contesto di chiamata, mentre lo stack è adatto solo per dati con durata limitata al contesto delle chiamate di funzione.

**Box<[T]>:**
- Stack: puntatore al primo elemento del vettore + campo `len` (**solo se l'array puntato non ha il tratto Size**)  che indica la lunghezza del vettore.
- Heap: vettore di elementi di tipo T.

**RefCell<T>:**
- Stack: campo `borrow` + dato T. Il campo `borrow` (4 o 8 Byte) è un flag è costituito da un numero incrementabile nel caso dei riferimenti semplici e da un numero specifico per il riferimento mutabile

**&[T]:**
- Stack: puntatore al primo elemento dello slice in memoria + campo `len` con lunghezza dello slice di T.
- I dati contenuti all'interno dello slice possono essere su stack o heap, indifferentemente

# Domanda 2
**Punteggio 3,0**

#### Un sistema concorrente può essere implementato creando più thread nello stesso processo, creando più processi basati su un singolo thread o basati su più thread. Si mettano a confronto i corrispettivi modelli di esecuzione e si evidenzino pregi e difetti in termini di robustezza, prestazioni, scalabilità e semplicità di sviluppo di ciascuno di essi.

## Risposta

### Thread nel singolo processo

#### Pregi:
- **Prestazioni**: condivisione diretta della memoria, riduzione del overhead di comunicazione tra thread rispetto ai processi.
- **Efficienza**: l’uso di thread è generalmente più leggero rispetto alla creazione di processi completi.
- **Comunicazione**: facilitata tra thread poiché condividono lo stesso spazio di indirizzamento.

#### Difetti:
- **Robustezza**: errori in un thread possono influenzare l'intero processo, portando a crash o corruzione della memoria.
- **Semplicità di sviluppo**: la gestione della concorrenza con i thread può essere complessa a causa della necessità di sincronizzare l'accesso ai dati condivisi, prevenire race condition e deadlock.
- **Scalabilità**: limitata dalla capacità di un singolo processo di gestire un numero elevato di thread, potenzialmente causando problemi di prestazioni.

### Processi singolo thread

#### Pregi:
- **Robustezza**: maggiore isolamento tra processi, un crash di un processo non influisce sugli altri.
- **Semplicità di sviluppo**: meno necessità di sincronizzazione e gestione della concorrenza rispetto ai thread all'interno dello stesso processo.
- **Scalabilità**: migliore scalabilità su sistemi multi-core e distribuiti, poiché ogni processo può essere schedulato indipendentemente.

#### Difetti:
- **Prestazioni**: overhead maggiore per la comunicazione tra processi (IPC) rispetto ai thread nello stesso processo.
- **Efficienza**: la creazione e la gestione di processi sono più costose in termini di risorse rispetto ai thread.
- **Comunicazione**: più complessa e meno efficiente rispetto ai thread, richiede meccanismi di IPC come pipe, socket o shared memory.

### Processi multi-thread

#### Pregi:
- **Robustezza**: combinazione di isolamento dei processi e flessibilità dei thread.
- **Prestazioni**: buona combinazione di comunicazione intra-processo efficiente (thread) e isolamento tra processi.
- **Scalabilità**: elevata, poiché può sfruttare meglio le risorse multi-core e distribuite, bilanciando il carico tra più processi e thread.
- **Semplicità di sviluppo**: offre vantaggi di entrambi i mondi, anche se può risultare complesso da gestire.

#### Difetti:
- **Complessità**: gestione più complessa della concorrenza rispetto ai singoli thread o processi, necessità di sincronizzare sia tra thread che tra processi.
- **Efficienza**: mantiene un overhead maggiore rispetto ai singoli thread a causa della gestione di più processi, ma meno rispetto ai singoli processi.

### Confronto riassuntivo

| Modello                              | Robustezza                                          | Prestazioni                                  | Scalabilità                                 | Semplicità di sviluppo                      |
|--------------------------------------|-----------------------------------------------------|----------------------------------------------|---------------------------------------------|---------------------------------------------|
| Thread multipli nel singolo processo | Bassa (condivisione di memoria e possibili crash)   | Elevata (comunicazione efficiente)           | Limitata (dipende dal singolo processo)     | Complessa (sincronizzazione necessaria)     |
| Processi multipli singolo thread     | Elevata (isolamento tra processi)                   | Media (overhead di IPC)                      | Elevata (buona su multi-core e distribuiti) | Più semplice (meno sincronizzazione)        |
| Processi multipli multi-thread       | Media (isolamento dei processi, flessibilità)       | Elevata (buona combinazione)                 | Elevata (ottima su multi-core e distribuiti)| Complessa (combinazione di thread e processi)|

# Domanda 3
**Punteggio 3,0**

#### In riferimento a programmi multi-thread, si indichi se la natura della loro esecuzione sia deterministica o meno a priori. Si produca un esempio che dimostri tale affermazione.

## Risposta
In programmi multi-thread, senza un'adeguata sincronizzazione, l'esecuzione non è deterministica: si possono ottenere comportamenti imprevedibili. Per evitare questi problemi, è necessario utilizzare costrutti di sincronizzazione, altrimenti si rischiano situazioni di deadlock. Un esempio che dimostra l'imprevedibilità di un approccio multi-thread è l'interferenza.

### Esempio di Interferenza

Consideriamo l'istruzione `a=a+1`. Sebbene sembri atomica, in realtà si traduce in due operazioni:
```rust
temp = a;
a = temp + 1;
```

Senza costrutti di sincronizzazione si verifica **race condition**, un thread può leggere `a` e salvarlo in `temp`, poi essere sospeso. Un altro thread potrebbe eseguire lo stesso codice e aggiornare `a` diverse volte. Quando il primo thread riprende, sovrascrive `a` con un valore obsoleto, causando **interferenza** e quindi un risultato inatteso.

#### Esempio in Rust

Un esempio pertinente in Rust è l'uso di `Rc` (Reference Counted), che ha contatori non atomici e non è né `Send` né `Sync`. Per l'uso multi-thread, Rust offre `Arc` (Atomic Reference Counted), che usa contatori atomici per incrementi e decrementi sicuri.

# Domanda 4
**Punteggio 6,0**

#### Un componente con funzionalità di cache permette di ottimizzare il comportamento di un sistema riducendo il numero di volte in cui una funzione è invocata, tenendo traccia dei risultati da essa restituiti per un particolare dato in ingresso.

Per generalità, si assuma che la funzione accetti un dato di tipo generico `K` e restituisca un valore di tipo generico `V`.

Il componente offre un unico metodo `get(...)` che prende in ingresso due parametri:
1. il valore `k` (di tipo `K`, clonabile) del parametro,
2. la funzione `f` (di tipo `K -> V`) responsabile della sua trasformazione, e restituisce uno smart pointer clonabile al relativo valore.

Se, per una determinata chiave `k`, non è ancora stato calcolato il valore corrispondente, la funzione viene invocata e ne viene restituito il risultato; altrimenti viene restituito il risultato già trovato.

Il componente cache deve essere thread-safe perché due o più thread possono richiedere contemporaneamente il valore di una data chiave: quando questo avviene e il dato non è ancora presente, la chiamata alla funzione dovrà essere eseguita nel contesto di UN SOLO thread, mentre gli altri dovranno aspettare il risultato in corso di elaborazione, SENZA CONSUMARE cicli macchina.

#### Si implementi tale componente in Rust.