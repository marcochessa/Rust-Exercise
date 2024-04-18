// Definizione del modulo 'solution'
pub mod solution {
    use std::cmp::Ordering;
    use std::hash::{Hash, Hasher};
    use std::ops::Add;
    use std::ops::AddAssign;
    // Definizione della struttura 'ComplexNumber'
    #[derive(Copy, Clone, Debug)]
    pub struct ComplexNumber{
        real: f64,
        imag: f64,
    }

    impl ComplexNumber {
        // Costruttore per 'ComplexNumber'
        pub fn new(real: f64, imag: f64) -> Self {
            ComplexNumber { real, imag }
        }

        pub fn from_real(real: f64) -> Self {
            ComplexNumber { real, imag:0.0  }
        }


        // Metodo per ottenere il valore reale
        pub fn real(&self) -> f64 {
            self.real
        }

        // Metodo per ottenere il valore immaginario
        pub fn imag(&self) -> f64 {
            self.imag
        }

        // Metodo per ottenere una tupla (real, imag)
        pub fn to_tuple(&self) -> (f64, f64) {
            (self.real, self.imag)
        }

        // Metodo per ottenere il modulo

        pub fn modolus(&self) ->  f64 {
            (self.real * self.real + self.imag + self.imag).sqrt()
        }

    }

    // Implementiamo il tratto Default per ComplexNumber
    impl Default for ComplexNumber {
        fn default() -> Self {
            ComplexNumber {
                real: 0.0, // Valore di default per la parte reale
                imag: 0.0, // Valore di default per la parte immaginaria
            }
        }
    }

    impl Into<f64> for ComplexNumber{
        fn into(self) -> f64 {
            if self.imag==0.0 {
                self.real
            } else {
                panic!("Imaginary part must be zero.");
            }
        }
    }

    impl Into<ComplexNumber> for f64{
        fn into(self) -> ComplexNumber {
            ComplexNumber {
                real: self, // Valore f64 per la parte reale
                imag: 0.0, // Valore di default per la parte immaginaria
            }
        }
    }

    impl PartialEq for ComplexNumber {
        // impl automatically symmetric & transitive
        fn eq(&self, other: &ComplexNumber) -> bool {
            self.real == other.real && self.imag == other.imag
        }
    }

    impl Eq for ComplexNumber {}

    impl PartialOrd<Self> for ComplexNumber {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
    }

    impl Ord for ComplexNumber {
        fn cmp(&self, other: &Self) -> Ordering {
            self.modolus().total_cmp(&other.modolus())
        }

    }

    impl AsRef<f64> for ComplexNumber{
        fn as_ref(&self) -> &f64 {
            &self.real
        }
    }

    impl AsMut<f64> for ComplexNumber{
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.real
        }
    }

    impl Hash for ComplexNumber {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            hasher.write(&self.real.to_be_bytes());
            hasher.write(&self.imag.to_ne_bytes());
        }
    }


    // Implementazione per addizione con due ComplexNumber
    impl Add for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl Add<&ComplexNumber> for ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: &ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl Add<ComplexNumber> for &ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl Add for &ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: &ComplexNumber) -> ComplexNumber {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    // Implementazione per addizione con un ComplexNumber e uno scalare (f64)
    impl Add<f64> for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: f64) -> Self {
            Self {
                real: self.real + rhs,
                imag: self.imag,
            }
        }
    }

    impl AddAssign for ComplexNumber {
        fn add_assign(&mut self, rhs: Self) {
            self.real += rhs.real;
            self.imag += rhs.imag;
        }
    }

    impl AddAssign<&ComplexNumber> for ComplexNumber {
        fn add_assign(&mut self, rhs: &Self) {
            Self::add_assign(self, *rhs);
        }
    }

}