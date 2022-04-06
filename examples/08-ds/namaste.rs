fn main() {
    let s = String::from("hello 😀");
    // let s = String::from("नमस्ते"); // Na-Ma-S-Te

    println!("Slice: {}", &s[0..6]);
    // println!("Slice: {}", &s[0..7]);

    let v: Vec<u8> = String::as_bytes(&s).to_vec();

    // println!("Slice: {}", String::as_bytes(s));
    println!("Slice: {:?}", &v[0..7]);


    // let v : Vec<u8> = vec!(b'h', b'e', b'l', b'l', b'o');

    // if s == v {
    //     println!("they are same!");
    // }

    // println!("S: {}", s);
    // // println!("V: {:?}", v);

    // println!("------");

    // println!("Length of bytes: {}", s.bytes().len()); // ?

    // for el in s.bytes() {
    //     println!("\tByte: {}", el);
    // }

    // println!("------");

    // println!("Length of chars: {}", s.bytes().len()); // 7

    // for el in s.chars() {
    //     println!("\tChar: {}", el);
    // }
}
