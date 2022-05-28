pub struct CircularQueue<T> {
    data: Vec<T>,
    offset: usize,
}

impl<T> CircularQueue<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            offset: 0,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    #[inline]
    pub fn next(&mut self) -> &T {
        if self.is_empty() {
            panic!("Circular queue is empty");
        }

        let result = &self.data[self.offset];
        self.offset += 1;
        if self.offset == self.data.len() {
            self.offset = 0;
        }

        result
    }

    #[inline]
    pub fn current(&self) -> &T {
        if self.is_empty() {
            panic!("Circular queue is empty");
        }

        &self.data[self.offset]
    }
}

impl<T> Default for CircularQueue<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            offset: 0,
        }
    }
}