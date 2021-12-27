pub struct SerialNumberIterator<const D: usize> {
    digits: [i32; D],
}

impl<const D: usize> Default for SerialNumberIterator<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const D: usize> SerialNumberIterator<D> {
    pub fn new() -> Self {
        let digits = [1; D];
        Self { digits }
    }

    fn inc(&mut self) {
        let mut i = D - 1;
        loop {
            self.digits[i] += 1;
            if self.digits[i] > 9 && i > 0 {
                self.digits[i] = 1;
                i -= 1;
                continue;
            }

            break;
        }
    }

    fn is_done(&self) -> bool {
        self.digits[0] > 9
    }
}

impl<const D: usize> Iterator for SerialNumberIterator<D> {
    type Item = [i32; D];

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }

        let r = self.digits;
        self.inc();
        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_number_iterator() {
        let mut serial_number_iterator: SerialNumberIterator<3> = SerialNumberIterator::new();

        assert_eq!(serial_number_iterator.next(), Some([1, 1, 1]));
        assert_eq!(serial_number_iterator.next(), Some([1, 1, 2]));
        assert_eq!(serial_number_iterator.next(), Some([1, 1, 3]));
        for c in 4..9 {
            assert_eq!(serial_number_iterator.next(), Some([1, 1, c]));
        }
        assert_eq!(serial_number_iterator.next(), Some([1, 1, 9]));
        assert_eq!(serial_number_iterator.next(), Some([1, 2, 1]));

        let mut serial_number_iterator: SerialNumberIterator<3> = SerialNumberIterator::new();
        for a in 1..=9 {
            for b in 1..=9 {
                for c in 1..=9 {
                    assert_eq!(serial_number_iterator.next(), Some([a, b, c]));
                }
            }
        }
        assert_eq!(serial_number_iterator.next(), None);
    }
}
