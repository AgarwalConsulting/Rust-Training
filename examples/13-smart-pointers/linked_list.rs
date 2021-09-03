struct Node<'a, T> {
    value: T,
    pointer: &'a Node<'a, T>
}

fn main() {
}
