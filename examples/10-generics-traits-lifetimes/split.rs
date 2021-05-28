fn split<'a, 'b>(inp: &'a str, sep: &'b char) -> Vec<&'a str> {
    let mut result = vec!();
    let mut start_index = 0;
    let mut curr_index = 0;
    for c in inp.chars() {
        if c == *sep {
            if curr_index != start_index {
                result.push(&inp[start_index..curr_index]);
            }
            start_index = curr_index + 1;
        }
        curr_index += 1;
    }

    result.push(&inp[start_index..curr_index]);
    return result
}

fn main() {
    let input = "Hello world!";

    let words;

    {
        let sep = ' ';
        words = split(&input, &sep);
    }

    for word in words {
        println!("{}", word);
    }
}
