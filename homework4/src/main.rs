struct MyVector<T> {
    v: Vec<T>
}

impl<T: Clone> MyVector<T> {
    fn new() -> MyVector<T>{
        MyVector {
            v: Vec::new()
        }
    }
    
    fn with_capacity(capacity: usize) -> MyVector<T>{
        MyVector {
            v: Vec::with_capacity(capacity)
        }
    }

    fn push(&mut self, value: T) {
        self.v.push(value)
    }

    fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }
     
     fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.v.len() {
            None
        } else {
            Some(self.v.remove(index))
        }
     }

     fn get(&self, index: usize) -> Option<&T> {
        self.v.get(index)
     }

     fn resize(&mut self, new_size: usize, value: T) {
        self.v.resize(new_size, value)
     }

}
fn main() {
    let mut vec: MyVector<i32> = MyVector::new();
    let mut vector: MyVector<i32> = MyVector::with_capacity(10);

    vec.push(5);
    vec.push(3);
    vec.push(6);
    vec.push(1);
    vec.push(9);
    vec.push(7);

    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{:?}", vec.remove(1));
    println!("{:?}", vec.get(1));
    println!("{:?}", vec.get(5));  // None

    vector.resize(5, 8);
    println!("{:?}", vector.pop());
}