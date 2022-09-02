use std::collections::VecDeque;

pub struct MyQueue<T> {
    pub vec: VecDeque<T>,
}

impl<T> Queue<T> for MyQueue<T> {
    fn new() -> Self {
        MyQueue {
            vec: VecDeque::new(),
        }
    }

    fn size(&self) -> usize {
        self.vec.len()
    }

    fn empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn enqueue(&mut self, e: T) {
        self.vec.push_back(e)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.vec.pop_front()
    }

    fn front(&self) -> Option<&T> {
        self.vec.front()
    }
}

pub trait Queue<T> {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn empty(&self) -> bool;
    fn enqueue(&mut self, e: T);
    fn dequeue(&mut self) -> Option<T>;
    fn front(&self) -> Option<&T>;
}
