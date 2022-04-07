struct Node<'j, T> {
    value: T,
    next: Option<&'j Node<'j, T>>,
}

// struct Node<T> {
//     value: T,
//     next: Option<Box<Node<T>>>,
// }

fn main() {
    let mut list = Node::<i32>{value: 42, next: None};

    {
        let n = Node{value: 10, next: None};

        list.next = Some(&n)
    }

    // list.next = Some(Box::new(Node::<i32>{value: 44, next: Some(Box::new(Node::<i32>{value: 46, next: None}))}));
//
    let mut cursor: Option<&Node<i32>> = Some(&list);
    // let mut cursor: Option<Box<Node<i32>>> = Some(Box::new(list));

    loop {
        match cursor {
            Some(node) => {
                // let v: Box<Node<i32>> = node;

                println!("\tValue: {}", node.value);
                cursor = node.next;
            },
            None => break,
        }
    }
}
