use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    head: usize,
    size: usize,
}

#[derive(Debug)]
pub enum Error {
    FullBuffer
}

impl<T> CircularBuffer<T> where T: Clone {
    pub fn new(capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        vec.resize(capacity, None);
        CircularBuffer {
            data: vec,
            head: 0,
            size: 0,
        }
    }
    pub fn write(&mut self, item: T) -> Result<(), Error> {
        if self.size == self.data.len() {
            return Err(Error::FullBuffer);
        }
        let index = (self.head + self.size) % self.data.len();
        self.data[index] = Some(item);
        self.size += 1;
        Ok(())
    }
    pub fn read(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let item = self.data[self.head].take();
        self.head = (self.head + 1) % self.data.len();
        self.size -= 1;

        item
    }
    pub fn clear(&mut self) {
        let capacity = self.data.len();
        self.data = Vec::with_capacity(capacity);
        self.data.resize(capacity, None);
        self.size = 0;
    }
    pub fn size(&self) -> usize {
        self.size
    }

    // può essere usata quando il buffer è pieno per forzare una
    // scrittura riscrivendo l’elemento più vecchio
    pub fn overwrite(&mut self, item: T) {
        if self.size == self.data.capacity() {
            self.data[self.head] = Some(item);
            self.head = (self.head + 1) % self.data.len();
        } else {
            //Scrittura normale
            let index = (self.head + self.size) % self.data.len();
            self.data[index] = Some(item);
            self.size += 1;
        }
    }
    pub fn make_contiguous(&mut self) {
        // Creiamo un nuovo vettore contiguo con la stessa capacità del buffer attuale
        let mut contiguous_data = Vec::with_capacity(self.data.len());

        // Copiamo gli elementi presenti nel buffer nella nuova struttura dati
        let mut index = self.head;
        for _ in 0..self.size {
            contiguous_data.push(self.data[index].take());
            index = (index + 1) % self.data.len();
        }

        // Aggiorniamo il buffer con la nuova struttura dati contigua
        self.data = contiguous_data;
        self.head = 0;
    }
}

impl<T> Index<usize> for CircularBuffer<T> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.head + index]
    }
}

impl<T> IndexMut<usize> for CircularBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[self.head + index]
    }
}


impl<T> Deref for CircularBuffer<T> {
    type Target = [Option<T>];

    fn deref(&self) -> &Self::Target {
        if self.head + self.size <= self.data.len() {
            // Creiamo uno slice degli elementi validi nel buffer circolare.
            // Utilizziamo self.head come inizio e (self.head + self.size) come fine
           &self.data[self.head..(self.head + self.size)]
        } else {
            //panic!("Circular Buffer not contiguous")
            &[] // Restituiamo un ref a un array vuoto in caso di non contiguità invece di panic
        }
    }
}


impl<T> DerefMut for CircularBuffer<T> where T: Clone{
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.head + self.size > self.data.len() {
            self.make_contiguous();
        }
        &mut self.data[self.head..(self.head + self.size)]
    }
}

