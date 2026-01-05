#[derive(Debug)]

struct Node {
    data: i32,
    next: Option<Box<Node>>,
}
// Box using heap allocation

#[derive(Debug)]

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_front(&mut self, value: i32) {
        let new_node = Box::new(Node {
            data: value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn print(&self) {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = node.next.as_ref();
        }

        println!("None");
    }

    fn push_back(&mut self, value: i32) {
        let new_node = Box::new( Node {
            data: value,
            next: None,
        });

        match self.head.as_mut(){
            None => self.head = Some(new_node),
            Some(mut current) => {
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(new_node);
            }
        }

    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push_front(3);
    list.push_front(5);

    list.print();
}
