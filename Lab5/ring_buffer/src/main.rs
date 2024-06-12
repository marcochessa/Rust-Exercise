use std::{
    sync::{Arc, Condvar, Mutex},
    thread, time::Instant,
};

pub mod circbuf {
    use std::fmt::Debug;
    use std::mem;
    use std::ops::{Deref, DerefMut, Index, IndexMut};

    pub struct CircularBuffer<T> {
        data: Vec<T>,
        tail: usize,
        head: usize,
        len: usize,
    }

    // we improve the FullBuffer error to include the element that we couldn't write
    // therefore we can try to rewrite it without copies
    #[derive(Debug, PartialEq)]
    pub enum Error<T> {
        EmptyBuffer,
        FullBuffer(T),
    }

    impl<T: Default> CircularBuffer<T> {
        pub fn new(capacity: usize) -> Self {
            let mut buf = Vec::with_capacity(capacity);
            for _ in 0..capacity {
                buf.push(T::default());
            }
            CircularBuffer {
                data: buf,
                tail: 0,
                head: 0,
                len: 0,
            }
        }

        pub fn write(&mut self, _element: T) -> Result<(), Error<T>> {
            if self.len == self.data.len() {
                return Err(Error::FullBuffer(_element));
            } else {
                self.data[self.tail] = _element;
                self.tail = (self.tail + 1) % self.data.len();
                self.len += 1;
                return Ok(());
            }
        }

        pub fn read(&mut self) -> Result<T, Error<T>> {
            if self.len == 0 {
                return Err(Error::EmptyBuffer);
            } else {
                let element = mem::take(&mut self.data[self.head]);
                self.head = (self.head + 1) % self.data.len();
                self.len -= 1;
                return Ok(element);
            }
        }

        pub fn clear(&mut self) {
            while self.len > 0 {
                self.read();
            }
        }

        pub fn overwrite(&mut self, _element: T) {
            // if it's full, we need to read one element and discard it
            if self.len == self.data.len() {
                self.read();
            }
            self.write(_element);
        }

        fn make_contiguous(&mut self) {
            // if it's empty, we can just reset the pointers
            if self.len == 0 {
                self.head = 0;
                self.tail = 0;
            } else {
                // otherwise we need to make it contiguos: just rotate it until head is zero
                while self.head != 0 {
                    if let Ok(element) = self.read() {
                        self.write(element);
                    }
                }
            }
        }

        fn real_index(&self, index: usize) -> usize {
            if index >= self.len {
                panic!("out of bounds");
            }
            (self.head + index) % self.data.len()
        }
    }

    impl<T: Default> Index<usize> for CircularBuffer<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[self.real_index(index)]
        }
    }

    impl<T: Default> IndexMut<usize> for CircularBuffer<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            let idx = self.real_index(index);
            &mut self.data[idx]
        }
    }

    impl<T> Deref for CircularBuffer<T> {
        type Target = [T];

        fn deref(&self) -> &Self::Target {
            if self.head > self.tail {
                panic!("not contiguous!!!")
            }
            &self.data[self.head..self.tail]
        }
    }

    pub trait TryDeref {
        type Target: ?Sized;

        fn try_deref(&self) -> Result<&Self::Target, String>;
    }

    impl<T: Default> TryDeref for CircularBuffer<T> {
        type Target = [T];

        fn try_deref(&self) -> Result<&Self::Target, String> {
            if self.head > self.tail {
                return Err("not contiguous".to_string());
            }
            Ok(&self.data[self.head..self.tail])
        }
    }

    impl<T: Default> DerefMut for CircularBuffer<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.make_contiguous();
            if self.head > self.tail {
                panic!("not contiguous!!!")
            }
            &mut self.data[self.head..self.tail]
        }
    }
}

pub struct SyncBuffer<T> {
    buf: Mutex<circbuf::CircularBuffer<T>>,
}

impl<T: Default> SyncBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        SyncBuffer {
            buf: Mutex::new(circbuf::CircularBuffer::new(capacity)),
        }
    }

    pub fn write(&self, element: T) -> Result<(), circbuf::Error<T>> {
        self.buf.lock().unwrap().write(element)
    }

    pub fn read(&self) -> Result<T, circbuf::Error<T>> {
        self.buf.lock().unwrap().read()
    }
}
// not required by the exercise but useful for avoid blocking
struct BlockingSyncBuf<T> {
    buf: Mutex<circbuf::CircularBuffer<T>>,
    cv: Condvar,
}

impl<T: Default> BlockingSyncBuf<T> {
    pub fn new(capacity: usize) -> Self {
        BlockingSyncBuf {
            buf: Mutex::new(circbuf::CircularBuffer::new(capacity)),
            cv: Condvar::new(),
        }
    }

    pub fn read_blocking(&self) -> T {
        let mut buf_guard = self.buf.lock().unwrap();
        loop {
            match buf_guard.read() {
                Ok(element) => {
                    self.cv.notify_one();
                    return element;
                }
                Err(_) => {
                    buf_guard = self.cv.wait(buf_guard).unwrap();
                }
            }
        }
    }

    pub fn write_blocking(&self, element: T) {
        let mut element = element;
        let mut buf_guard = self.buf.lock().unwrap();
        loop {
            match buf_guard.write(element) {
                Ok(_) => {
                    self.cv.notify_one();
                    return;
                }
                Err(circbuf::Error::FullBuffer(el)) => {
                    element = el;
                    buf_guard = self.cv.wait(buf_guard).unwrap();
                }
                Err(_) => {
                    panic!("unexpected error")
                }
            }
        }
    }
}

pub fn test_producer_consumer() {
    let buf = Arc::new(BlockingSyncBuf::new(10));
    let buf1 = buf.clone();
    let consumer = thread::spawn(move || loop {
        let el = buf1.read_blocking();
        println!("read: {}", el);
        thread::sleep(std::time::Duration::from_secs(2));
    });

    let producer = std::thread::spawn(move || {
        let mut count = 0;
        loop {
            count += 1;
            buf.write_blocking(count);
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

pub fn test_speed() {
    let buf = Arc::new(SyncBuffer::new(10));
    let buf1 = buf.clone();

    let consumer = thread::spawn(move || loop {

        let mut last_print = Instant::now();
        let mut count = 0u64;
        loop {
            match buf1.read() {
                Ok(el) => {
                    count += 1;
                    if last_print.elapsed().as_secs() == 1 {
                        println!("msg/s: {} elapsed: {:?}", count/last_print.elapsed().as_secs(), last_print.elapsed());
                        last_print = Instant::now();
                        count = 0;
                    }
                }
                Err(_) => { }
            }
        }
    });

    let mut count: usize = 0;
    loop {
        let el = count;
        // try writing until seccess
        while let Err(el) = buf.write(el) { };
        count += 1;
    }

}

fn main() {
    println!("Hello, world!");
}
