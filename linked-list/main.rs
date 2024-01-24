#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn insert(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn search(&self, target: i32) -> Option<usize> {
        let mut current = &self.head;
        let mut index = 0;

        while let Some(node) = current {
            if node.data == target {
                return Some(index);
            }

            current = &node.next;
            index += 1;
        }

        None
    }

    fn display(&self) {
        let mut current = &self.head;

        while let Some(node) = current {
            print!("{} ", node.data);
            current = &node.next;
        }

        println!();
    }
}

fn main() {
    let mut linked_list = LinkedList::new();

    linked_list.insert(2);
    linked_list.insert(4);
    linked_list.insert(7);
    linked_list.insert(1);
    linked_list.insert(0);

    println!("Linked List: ");
    linked_list.display();

    let search_value = 7;
    match linked_list.search(search_value) {
        Some(index) => println!("Found {} at index {}", search_value, index),
        None => println!("{} not found in the list", search_value),
    }
}
