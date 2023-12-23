struct Node<T> {
    value: T,
    next_node: Option<Node<T>>
}

// enum Option<T> {
//     Some(T), // Tuple struct
//     None // Unit struct
// }

struct Person {
    name: String,
    age: i32
}

fn main() {
    let (s1, s2) = ("hello", "world");

    let linked_list = Node{value: s1, next_node: None};

    println!("{:?}", linked_list);
}
