# Domanda 1
**Punteggio 3,0**

#### Si spieghi il concetto di possesso in relazione agli smart pointers. Come viene gestito il ciclo delle risorse quando si utilizzano smart pointers?
Infine, si espliciti il ciclo di vita delle risorse nel seguente esempio:
```rust
{
    let mut i = 10;
    let bi1 = Box::new(i);
    let mut bi2 = Box::new(*bi1);
    *bi2 = 20;
    i = *bi2;
    println!("{} {:?} {:?}", i, bi1, bi2);
}
```

## Risposta
### Concetto di Possesso negli Smart Pointers

Gli smart pointer sono dei costrutti che aggiungono funzionalità ad un semplice puntatore. La funzionalità intrinseca degli 
smart pointer è di assicurare che un puntatore venga distrutto quando non è più utilizzato (usando il paradigma RAII).

**RAII (Resource Acquisition Is Initialization)**:
- RAII è un paradigma di gestione delle risorse che garantisce che le risorse (come memoria, file, socket di rete) vengano acquisite e rilasciate in modo deterministico.
- In questo paradigma, l'acquisizione delle risorse avviene durante l'inizializzazione dell'oggetto e la loro liberazione avviene quando l'oggetto esce dal suo scope.

Quindi gli smart pointer, quando sono distrutti, distruggono i propri dati contenuti (incluso il puntatore). 
Ciò varia leggermente per quegli smart pointer che supportano più owner (`Rc`, `Arc`), dove invece la struttura è deallocata 
quando tutti i possessori smettono di impiegarla.

### Ciclo delle Risorse con Smart Pointers

Nell'esempio usiamo lo smart pointer `Box`, che alloca sull'heap un dato in ingresso e mantiene il puntatore associato come descritto precedentemente. Questo tipo di puntatore prevede un solo owner, quindi appena `Box` viene droppato, sarà fatto lo stesso con il puntatore e il relativo dato.

#### Analisi dell'Esempio
1. `let mut i = 10;`
   - Viene dichiarata una variabile mutabile `i` con valore 10.

2. `let bi1 = Box::new(i);`
   - Viene creato uno smart pointer `Box` che alloca il valore di `i` (10) nell'heap.
   - `bi1` diventa l'owner di questo `Box`.

3. `let mut bi2 = Box::new(*bi1);`
   - Viene creato un nuovo `Box` che alloca una copia del valore contenuto in `bi1` (10) in una nuova area di memoria nell'heap.
   - `bi2` diventa l'owner di questo nuovo `Box`.

4. `*bi2 = 20;`
   - Il valore puntato da `bi2` viene modificato a 20.

5. `i = *bi2;`
   - `i` viene aggiornato con il valore dereferenziato di `bi2`, quindi `i` diventa 20.

6. `println!("{} {:?} {:?}", i, bi1, bi2);`
   - Viene stampato:
      - `i = 20`
      - `bi1` è un `Box` che punta a 10
      - `bi2` è un `Box` che punta a 20
---

# Domanda 2
**Punteggio 3,0**

#### Si descriva la gestione della memoria in Rust e si spieghi come vengono evitati i problemi di sicurezza comuni come le violazioni di accesso o la presenza di puntatori nulli.

## Risposta

Ecco una risposta più completa che affronta anche i punti sollevati dal commento:

### Gestione della Memoria in Rust

Rust è un linguaggio di programmazione che mette al primo posto la sicurezza e la gestione della memoria senza costi di runtime aggiuntivi. Questo viene ottenuto attraverso un sistema di proprietà (ownership) e il concetto di RAII (Resource Acquisition Is Initialization).

1. **Ownership**:
   - Ogni valore ha un proprietario (owner) unico.
   - Quando il proprietario esce dallo scope, il valore viene automaticamente deallocato.
   - Il sistema di proprietà impedisce accessi simultanei non sicuri, previene dangling pointers e memory leaks.

2. **Borrowing**:
   - Un valore può essere prestato (borrowed) in lettura o scrittura.
   - Regole del borrowing:
      - Si può avere solo un mutabile reference (`&mut`) o molti immutabili references (`&`), ma non entrambi simultaneamente.
   - Queste regole sono verificate a tempo di compilazione dal borrow checker, che impedisce errori comuni come data races e invalid memory access.

### Sicurezza della Memoria

Rust evita molti problemi di sicurezza della memoria comuni attraverso diversi meccanismi:

1. **Violazioni di Accesso**:
   - **Boundary Checker**: Rust include boundary checks per gli accessi agli array, prevenendo buffer overflows. Ad esempio, tentare di accedere a un indice fuori dai limiti di un array causerà un panic a runtime, evitando scritture o letture non sicure.
   - **Controlli di Tipo**: Il sistema di tipi di Rust impedisce l'uso di tipi in modi non previsti, riducendo ulteriormente i rischi di violazioni di accesso.

2. **Puntatori Nulli**:
   - Rust non ha puntatori nulli tradizionali. Al loro posto, utilizza il tipo `Option<T>`, che rappresenta un valore che può essere presente (`Some(T)`) o assente (`None`).
   - Questo approccio elimina il rischio di dereferenziazione di puntatori nulli, poiché il compilatore forza la gestione esplicita dei casi in cui un valore può essere assente.

3. **Puntatori Sospesi (Dangling Pointers)**:
   - Il sistema di proprietà e il borrow checker impediscono che i riferimenti a memoria deallocata vengano utilizzati.
   - Quando il proprietario di un valore esce dallo scope, tutti i riferimenti al valore diventano invalidi, impedendo accessi non sicuri.

4. **Double Free**:
   - Poiché Rust gestisce automaticamente la deallocazione delle risorse, non c'è rischio di deallocare la stessa risorsa più volte, eliminando il problema dei double free.
---

# Domanda 3
**Punteggio 3,0**

#### Si illustri come sia possibile gestire correttamente le situazioni di errore in Rust, distinguendo tra `Option` e `Result`.

## Risposta
### Gestione degli Errori in Rust

Rust gestisce gli errori in maniera diversa rispetto a linguaggi come il C++. In quest'ultimo si usa un approccio dove 
ogni funzione alloca una sezione dello stack che contiene il contesto delle eccezioni e si controlla il risultato di una funzione invocata ispezionando questo contesto mantenuto al ritorno della funzione.

In Rust, il ritorno stesso della funzione trasmette potenzialmente l'errore senza la necessità di una struttura extra nello stack.

### `Option` e `Result`

- `Result<R, E>` è un enum con varianti `Ok(R)` e `Err(E)`, che contengono rispettivamente il valore di successo o l'errore.
- `Option<R>` è un enum con varianti `Some(R)` e `None`, usato quando l'operazione non fallisce ma può avere un risultato vuoto.

Questo approccio obbliga alla gestione puntuale degli errori, evitando valori nulli (non supportati in Rust) e promuovendo la sicurezza attraverso il controllo esplicito dei risultati delle funzioni.

Ecco la risposta seguendo il template richiesto:

# Domanda 4
**Punteggio 6,00**

Una `DelayedQueue<T: Send>` è un particolare tipo di coda non limitata che offre tre metodi principali, oltre alla funzione costruttrice:
1. `offer(&self, t: T, i: Instant)`: Inserisce un elemento che non potrà essere estratto prima dell'istante di scadenza `i`.
2. `take(&self) -> Option<T>`: Cerca l'elemento `t` con scadenza più ravvicinata: se tale scadenza è già stata oltrepassata, restituisce `Some(t)`; se la scadenza non è ancora stata superata, attende senza consumare cicli di CPU, che tale tempo trascorra, per poi restituire `Some(t)`; se non è presente nessun elemento in coda, restituisce `None`. Se, durante l'attesa, avviene un cambiamento qualsiasi al contenuto della coda, ripete il procedimento suddetto con il nuovo elemento a scadenza più ravvicinata (ammesso che ci sia ancora).
3. `size(&self) -> usize`: Restituisce il numero di elementi in coda indipendentemente dal fatto che siano scaduti o meno.

#### Si implementi tale struttura dati nel linguaggio Rust, avendo cura di renderne il comportamento thread-safe. Si ricordi che gli oggetti di tipo `Condvar` offrono un meccanismo di attesa limitata nel tempo, offerto dai metodi `wait_timeout(...)` e `wait_timeout_while(...)`.

## Risposta
Vedere **es4.rs**