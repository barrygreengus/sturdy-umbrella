use std::fmt::Display;
struct BoundedQueue<T> {
    queue: Vec<T>,
    max_size: usize,
}

impl<T: Display> BoundedQueue<T> {
    fn new(max_size: usize) -> BoundedQueue<T> {
        BoundedQueue::<T> {
            queue: Vec::new(),
            max_size: max_size,
        }
    }

    fn enqueue(&mut self, elem: T) {
        // insert elem to front of queue
        self.queue.insert(0, elem);
        // if queue past capacity, remove from end
        if self.queue.len() > self.max_size {
            self.queue.pop();
        }
    }

    /*
    fn dequeue(&mut self) -> Option<T> {
        self.queue.pop()
    }
    */

    fn print(&self) {
        println!("BoundedBuffer: max_size={}, len={}", self.max_size, self.queue.len());
        print!("[ ");
        for item in self.queue.iter() {
            print!("{} ", item);
        }
        println!("]");
    }
}
