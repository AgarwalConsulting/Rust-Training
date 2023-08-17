struct Cons<T> {
    head: T,
    tail: Cons<T>
}

fn main() {
    let list = Cons(1, (Cons(2, (Cons(3, (Cons(4, ())))))));

}
