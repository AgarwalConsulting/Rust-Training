fn split<'a, 'b>(inp: &'a String, sep: &'b char) -> Vec<&'a str> {
    let mut result = vec!();
    let mut start_index = 0;
    let mut curr_index = 0;
    for c in inp.chars() {
        if c == *sep {
            if curr_index != start_index {
                result.push(&inp[start_index..curr_index]); // &str
            }
            start_index = curr_index + 1;
        }
        curr_index += 1;
    }

    result.push(&inp[start_index..curr_index]); // &str
    return result
}

fn main() {
    let sep = ' ';

    let words;

    {
        let input = String::from("Hello world!");
        words = split(&input, &sep);
    }

    for word in words {
        println!("{}", word);
    }
}
