use ::circular_buffer::CircularBuffer;
use complex_number::solution::ComplexNumber;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_check_size() {
        let mut buffer = CircularBuffer::new(3);
        assert_eq!(buffer.size(), 0);

        buffer.write(1).unwrap();
        assert_eq!(buffer.size(), 1);

        buffer.write(2).unwrap();
        assert_eq!(buffer.size(), 2);

        buffer.write(3).unwrap();
        assert_eq!(buffer.size(), 3);
    }

    #[test]
    fn test_insert_read() {
        let mut buffer = CircularBuffer::new(3);
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.write(3).unwrap();

        assert_eq!(buffer.read(), Some(1));
        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.read(), Some(3));
    }

    #[test]
    fn test_overflow() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();

        // Prova a scrivere un altro elemento
        assert!(buffer.write(3).is_err());
    }

    #[test]
    fn test_overwrite() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();

        // Sovrascrive il primo elemento
        buffer.overwrite(3);

        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.read(), Some(3));
    }

    #[test]
    fn test_make_contiguous() {
        let mut buffer = CircularBuffer::new(3);
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.read(); // Legge un elemento

        // Fai diventare il buffer contiguo
        buffer.make_contiguous();

        assert_eq!(buffer.size(), 1);
    }

    #[test]
    fn test_error_read_write() {
        // Test precedenti rimossi per brevità

        // Verifica la lettura da un CircularBuffer vuoto
        let mut empty_buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        assert_eq!(empty_buffer.read(), None);

        // Verifica la scrittura in un CircularBuffer pieno
        let mut full_buffer = CircularBuffer::new(2);
        full_buffer.write(1).unwrap();
        full_buffer.write(2).unwrap();
        assert!(full_buffer.write(3).is_err());
    }

    #[test]
    fn test_index_operator() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5);
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        assert_eq!(buffer[0], Some(1));
        assert_eq!(buffer[1], Some(2));
    }

    #[test]
    fn test_index_mut_operator() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5);
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer[0] = Some(5);
        assert_eq!(buffer[0], Some(5));
    }

    #[test]
    fn test_deref_trait() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5);
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        let deref_slice: &[Option<i32>] = &buffer;
        assert_eq!(deref_slice, &[Some(1), Some(2)]);
    }

    #[test]
    fn test_deref_panic_trait() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.write(3).unwrap();
        buffer.overwrite(4);
        buffer.read();

        // Questo test dovrebbe generare un panico perché il buffer non è contiguo!
        let _deref_slice: &[Option<i32>] = &buffer;
        assert_eq!(_deref_slice, &[]);
    }

    #[test]
    fn test_deref_mut_trait() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5);
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        let deref_mut_slice: &mut [Option<i32>] = &mut buffer;
        deref_mut_slice[0] = Some(5);
        assert_eq!(deref_mut_slice, &[Some(5), Some(2)]);
    }

    #[test]
    fn test_write_and_deref_complex() {
        let mut buffer: CircularBuffer<ComplexNumber> = CircularBuffer::new(5);
        buffer.write(ComplexNumber::new(1.0, 2.0)).unwrap();
        buffer.write(ComplexNumber::new(3.0, 4.0)).unwrap();
        let deref_slice: &[Option<ComplexNumber>] = &buffer;
        assert_eq!(
            deref_slice,
            &[Some(ComplexNumber::new(1.0, 2.0)),
                Some(ComplexNumber::new(3.0, 4.0))]
        );
    }

    #[test]
    fn test_write_different_type() {
        #[derive(Clone, PartialEq, Debug)]
        enum Shape {
            Square(u32),
            Point(u8, u8),
            Empty,
        }

        let mut buffer: CircularBuffer<Shape> = CircularBuffer::new(5);
        buffer.write(Shape::Empty).unwrap();
        buffer.write(Shape::Point(1,2)).unwrap();
        buffer.write(Shape::Square(55)).unwrap();

        /*Poiché gli elementi del buffer sono rappresentati come varianti di un
         *enum, ogni elemento occupa lo spazio necessario per il tipo più grande tra
         *le varianti dell'enum. Questo può portare a uno spreco di memoria se alcuni tipi
         *sono significativamente più grandi di altri.*/


        // Creiamo un vettore con i valori attesi nel buffer
        let expected_values = vec![
            Some(Shape::Empty),
            Some(Shape::Point(1, 2)),
            Some(Shape::Square(55))
        ];

        let deref_slice: &[Option<Shape>] = &buffer;

        // Verifichiamo che il contenuto effettivo del buffer sia uguale ai valori attesi
        assert_eq!(deref_slice, &expected_values);

    }
}
