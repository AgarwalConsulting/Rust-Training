#[derive(Debug)]
struct Pair<T: Copy>(T, T);

impl<T: Copy> IntoIterator for Pair<T> {
    type Item = T;
    type IntoIter = PairIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        PairIterator{p: self, curr_idx: 0}
    }
}

struct PairIterator<T: Copy> {
    p: Pair<T>,
    curr_idx: u8,
}

impl<T: Copy> Iterator for PairIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_idx > 1 {
            None
        } else {
            let val = if self.curr_idx == 0 { self.p.0 } else { self.p.1 };
            self.curr_idx += 1;
            Some(val)
        }
    }
}

fn main() {
    let p = Pair(30, 40);

    // let pi: Vec<i32> = p.into_iter().map(|el| { el * 2 }).collect();
    let pi: i32 = p.into_iter().sum();

    // println!("{:?}", pi.next());
    // println!("{:?}", pi.next());
    // println!("{:?}", pi.next());

    println!("{:?}", pi);
}
