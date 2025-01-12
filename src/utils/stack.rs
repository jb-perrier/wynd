pub struct StackRefMut<'a, T> where T: Default {
    data: &'a mut [T],
    next_free: usize,
}

impl <'a, T> StackRefMut<'a, T> where T: Default {
    pub fn new(data: &'a mut [T]) -> StackRefMut<'a, T> {
        StackRefMut {
            data,
            next_free: 0,
        }
    }

    pub fn clear(&mut self) {
        self.next_free = 0;
    }

    pub fn push(&mut self, value: T) {
        self.data[self.next_free] = value;
        self.next_free += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.next_free == 0 {
            None
        } else {
            self.next_free -= 1;
            Some(std::mem::replace(&mut self.data[self.next_free], Default::default()))
        }
    }

    pub fn peek(&self, offset: usize) -> Option<&T> {
        self.data.get(self.next_free - 1 - offset)
    }

    pub fn peek_mut(&mut self, offset: usize) -> Option<&mut T> {
        let index = self.next_free - 1 - offset;
        if index < self.data.len() {
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.next_free
    }

    pub fn is_empty(&self) -> bool {
        self.next_free == 0
    }
}