## Domanda 1
**Punteggio 3,0**

#### Si definisca il problema delle referenze cicliche nell’uso degli smart pointers. Si fornisca quindi un esempio in cui tale problema sia presente.

## Risposta
In Rust, i riferimenti classici implicano il possesso del valore referenziato e ne determinano il ciclo di vita. Ciò significa che finché esiste un riferimento ad un valore, questo non viene distrutto. Questo comporta un problema qualora si desideri creare delle strutture con riferimenti ciclici, ovvero una struttura dati che è puntata da una seconda struttura di cui la prima possiede un riferimento. Questo implica che tali strutture non possano essere correttamente deallocate, in quanto la distruzione della prima è vietata dall'esistenza di un riferimento ad essa da parte della seconda, e viceversa.

Per ovviare a questo problema, Rust offre lo smart pointer "Reference Counter" (o `Rc<T>`), il quale è così strutturato:

- La variabile `Rc` sullo stack consiste in un semplice puntatore, che punta ad una struttura sullo heap, composta da due contatori, chiamati rispettivamente Strong e Weak, e dal valore di tipo `T` tramite `Rc` allocato.
- `Rc` permette di condividere tale valore sullo heap, in quanto la variabile `Rc` può essere clonata senza problemi, e la creazione di un nuovo puntatore al valore porta all'aumento del contatore Strong.
- La distruzione di riferimenti causa un decremento del contatore Strong fino a 0, e quando giunge a 0, la struttura viene deallocata.

Il problema delle referenze cicliche viene risolto grazie al secondo contatore, Weak, che identifica il numero di Smart Pointer di tipo Weak, al dato allocato. Questi ultimi Smart Pointer si creano a partire da un `Rc` tramite il metodo `Rc::downgrade(&Rc)`, e danno origine ad un puntatore alla struttura allocata, senza però implicarne l'accesso: infatti, tramite un Weak non è possibile accedere al valore, se non tramite il metodo `upgrade()`, il quale può fallire se il valore referenziato è già stato deallocato.

In questo modo è possibile creare una referenza ciclica, a patto che un riferimento sia di tipo Strong e uno di tipo Weak. Sta al programmatore implementare tale logica correttamente, considerando i puntatori quando un oggetto viene deallocato.

### Esempio di tale problema
# DA RIVEDERE
```rust
use std::rc::{Rc, Weak};

struct Node {
    val: i32,
    pointer: Option<Rc<Node>>,
}

fn main() {
    let a = Rc::new(Node { val: 1, pointer: None });
    let b = Rc::new(Node { val: 2, pointer: Some(Rc::clone(&a)) });
    
    // Creiamo una referenza ciclica
    if let Some(ref pointer) = a.pointer {
        *Rc::get_mut(pointer).unwrap() = Some(Rc::downgrade(&b));
    }

    println!("a reads index {} from b", a.pointer.as_ref().unwrap().val); // reading 2
    println!("b reads index {} from a", b.pointer.as_ref().unwrap().upgrade().unwrap().val); // reading 1

    drop(a);
    println!("b reads index {} from a", b.pointer.as_ref().unwrap().upgrade().unwrap().val); // panics here
}
```
---

## Domanda 2
**Punteggio 3,0**

#### Si identifichino i tratti fondamentali della concorrenza. Successivamente, in riferimento alla mutabilità/immutabilità delle risorse, si delinei come questa affligga la gestione della sincronizzazione a livello thread.

## Risposta
I tratti fondamentali della concorrenza in Rust sono `Send` e `Sync`. 
#### Send
- **Descrizione**: Il tratto `std::marker::Send` è applicato automaticamente a tutti i tipi che possono essere trasferiti in sicurezza da un thread a un altro. Questo garantisce che non ci siano accessi concorrenti al loro contenuto.
- **Uso**:
    - Un tipo con il tratto `Send` può essere passato per valore ad altri thread.
    - La movimentazione o la copia dei dati garantisce che non ci siano accessi simultanei.
    - I tipi composti (come struct, tuple, enum, array) implementano `Send` se tutti i loro campi lo implementano.
    - È possibile forzare l'assegnazione o la rimozione del tratto `Send` solo all'interno di un blocco `unsafe`, rendendo il programmatore responsabile della sicurezza.
- **Nota**: Puntatori e riferimenti non implementano `Send`, poiché l'esecuzione indipendente dei thread non consente al borrow checker di garantire la correttezza.

#### Sync
- **Descrizione**: Il tratto `std::marker::Sync` è applicato automaticamente a tutti i tipi `T` tali che `&T` implementa `Send`. Questo indica che i riferimenti non mutabili a questi tipi possono essere condivisi in sicurezza tra thread diversi.
- **Uso**:
    - Un tipo con il tratto `Sync` può essere passato come riferimento non mutabile ad altri thread, permettendo accessi concorrenti sicuri.
    - I tipi che implementano una mutabilità interna (come `Cell` e `RefCell`) non implementano `Sync`, poiché ciò potrebbe portare a comport

Il tema della mutabilità/immutabilità delle risorse in Rust è molto importante. Se una risorsa è fornita ad una variabile con accesso mutabile, l'accesso ad essa è possibile solo a tale variabile, ed essa non può condividere il proprio lifetime con altre variabili che posseggono tale valore. Se l'accesso è dato in sola lettura invece, ovvero il valore è condiviso in modo immutabile, possono coesistere più variabili allo stesso tempo che ne condividono l'accesso.

Questo problema diventa molto più complesso in un contesto multi-thread, dove l'intersezione dei lifetime non è sempre predicibile e controllabile. Per questo motivo, tutti i metodi che permettono di modificare una variabile in un contesto multi-thread implementano la interior mutability, ovvero chiedono il possesso del valore condiviso in modo immutabile, salvo poi poterlo modificare ugualmente. Tuttavia, com'è naturale, per garantire il corretto accesso a tale valore e per assicurare l'impossibilità dell'insorgere di comportamenti non deterministici, tali valori condivisi, se mutabili, richiedono strutture specifiche che ne garantiscano la sicurezza dell'accesso, come i tipi `Atomic` e i `Mutex`.

---

## Domanda 3
**Punteggio: 3,0**

#### Data la struttura dati definita come
```rust
struct Data { 
    element: AsVector, 
    next: Rc<Data> 
}

enum AsVector {
    AsVector(Box<Rc<i32>>),
    None 
}
```

#### Indicare l’occupazione di memoria di un singolo elemento in termini di:
1. numero di byte complessivi (caso peggiore, architettura a 64 bit)
2. posizionamento dei vari byte in memoria (stack, heap, ecc.).

## Risposta
La richiesta prevede un'architettura da 64 bit quindi avremo puntatori da **8 Byte** ciascuno.

Occupazione nel dettaglio:

- **Struct `Data`**:
  - **Dettagli**:
    - `element`: punta l'ENUM `AsVector` che contiene un puntatore valido da 8 byte oppure None che grazie a delle ottimizzazioni di Rust non occupa memoria. Essendoci quindi una distinzione già netta tra gli elementi non è necessario l'ulteriore byte di tag per l'ENUM. Nel caso quindi di un solo elemento abbiamo 8 Byte occupati.
      - A sua volta **Se puntatore valido**:
        - la Box punta a un `RC` di 8 byte sullo heap che a sua volta punta alla sua struttura costituita da Strong - Weak - _i32_ (8B + 8B + 4B ) per un totale di **20 byte**  (_Si assume Allineamento a **4 Byte**_)
        - element alloca quindi sullo **Heap** un totale di: **28 byte (8 + 20)**
    - `next`: è un `RC` da 8 byte che punta un `RC` sullo Heap che contiene `Data`
      - A sua volta quindi **Se puntatore valido**:
        - punta alla struttura dell'`RC` costituita da Strong - Weak - _Data_ (8B + 8B + 16B) per un totale di **32 byte**
  - **Allocazione `Data`**:
    - Può essere memorizzata su **Stack o Heap**: 16 byte per `Data`

- **Somma Totale**:
  - **Stack o Heap**: 16 byte (struct `Data`)
  - **Heap**: 28 byte (`element`) + 32 byte (`next`)
  - Nel caso di **un solo elemento** abbiamo next che contiene `None (0B)` quindi un **Totale** di : 16 + 28 + 0 = **44 byte**
---

## Domanda 4
**Punteggio 6,0**

#### La struct MpMcChannel<E: Send> è una implementazione di un canale su cui possono scrivere molti produttori e da cui possono attingere valori molti consumatori. Tale struttura offre i seguenti metodi:
```rust
new(n: usize) -> Self
//crea una istanza del canale basato su un buffer circolare di "n" elementi
send(e: E) -> Option<()>
//invia l'elemento "e" sul canale. Se il buffer circolare è pieno, attende senza consumare CPU che si crei almeno un posto libero in cui depositare il valore. Ritorna: Some(()) se è stato possibile inserire il valore nel buffer circolare, None se il canale è stato chiuso (Attenzione: la chiusura può avvenire anche mentre si è in attesa che si liberi spazio) o se si è verificato un errore interno
recv() -> Option<E>
//legge il prossimo elemento presente sul canale. Se il buffer circolare è vuoto, attende senza consumare CPU che venga depositato almeno un valore. Ritorna: Some(e) se è stato possibile prelevare un valore dal buffer, None se il canale è stato chiuso (Attenzione: se, all'atto della chiusura, sono già presenti valori nel buffer, questi devono essere ritornati, prima di indicare che il buffer è stato chiuso; se la chiusura avviene mentre si è in attesa di un valore, l'attesa si sblocca e viene ritornato None) o se si è verificato un errore interno.
shutdown() -> Option<()>
//chiude il canale, impedendo ulteriori invii di valori. Ritorna: Some(()) per indicare la corretta chiusura, None in caso di errore interno all'implementazione
```

#### Si implementi tale struttura dati in linguaggio Rust, senza utilizzare i canali forniti dalla libreria standard né da altre librerie, avendo cura di garantirne la correttezza in presenza di più thread e di non generare la condizione di panico all'interno dei suoi metodi.

## Risposta
Vedere **es4.rs**