// There are some basic operations that allow us to perform different actions on a stack.
//
// Push: Add an element to the top of a stack
// Pop: Remove an element from the top of a stack
// IsEmpty: Check if the stack is empty
// IsFull: Check if the stack is full
// Peek: Get the value of the top element without removing it

fn main() {
    let mut s:Stack<String> = Stack::new();
    s.push(String::from("4"));
    s.push(String::from("1"));
    println!("{:?}", s.is_empty());
    println!("{:?}", s.is_full());
    println!("{:?}", s.peek());
    println!("{:?}", s.pop());
    println!("{:?}", s.peek());
}

struct Stack<T>  {
    items: Vec<T>

}
impl<T> Stack<T> {

    fn new() -> Self {
        return Stack{
            items: Vec::new(),
        }
    }
    fn push(&mut self, item: T) {
        self.items.push(item)
    }
    fn pop(&mut self) -> Option<T>{
        self.items.pop()
    }
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    fn is_full(&self) -> bool {
        self.items.capacity() == self.items.len()
    }
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

}