#[derive(Debug)]

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();

    println!("is stack empty? {}", stack.is_empty());
    println!("Adding elements to stack");

    stack.push(10);
    stack.push(22);
    stack.push(35);

    println!("Stack size: {}", stack.size());

    if let Some(top_item) = stack.peek() {
        println!("Top item: {}", top_item);
    } else {
        println!("Stack is empty");
    }

    println!("{:?}", stack);

    stack.pop(); 
    println!("Stack after using pop() : {:?}", stack);

}

