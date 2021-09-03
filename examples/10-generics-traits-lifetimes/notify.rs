trait Summarize {
    fn summary(&self) -> String;
}

trait Notify {
    fn notify(&self, text: String);
}

fn notify<T, U>(a: T, b: U)
    where T: Summarize, U: Notify {
    b.notify(a.summary())
}

fn main() {
}
