struct MyVec<T> {
    v: Vec<T>,
}

impl<T: std::fmt::Display> MyVec<T>{
    fn new() -> Self {
        MyVec {v: Vec::new()}
    }

    fn  with_capacity(size: usize) -> Self {
        MyVec {v: Vec::with_capacity(size)}
    }

    fn push(&mut self, value: T) {
        self.v.push(value);
    }

    fn pop(&mut self) {
        self.v.pop();
    }

    fn len(&self) -> usize {
        self.v.len()
    }

    fn is_empty(&self) {
        self.v.is_empty();
    }

    fn get(&self, index: usize) -> &T {
        self.v.get(index).unwrap()
    }

    fn capacity(&self) -> usize{
        self.v.capacity()
    }

    fn remove(&mut self, index: usize) {
        self.v.remove(index);
    }

    fn print(&self) {
        for i in &self.v {
            print!("{} ", i);
        }
        print!("\n");
    }
}

fn main() {
    println!("Hello, world!");

    let mut vec1: MyVec<i32> = MyVec::with_capacity(7);
    let mut vec2: MyVec<i32> = MyVec::new();

    println!("Filling a vector with elements:");
    vec1.push(123);
    vec1.push(12);
    vec1.push(1);
    vec1.push(33);
    vec1.push(312);
    vec1.push(3111);
    vec1.print();

    println!("Deleting an element at an index 2");
    vec1.remove(2);
    vec1.print();

    println!("Removing the last element");
    vec1.pop();
    vec1.print();

    println!("Removing the last element");
    vec1.pop();
    vec1.print();

    println!("The length of the vector - {:?}", vec1.len());
    println!("The capacity of the vector - {:?}", vec1.capacity());
}
