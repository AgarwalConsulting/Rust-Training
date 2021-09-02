pub fn greeting(mut name: String) -> String {
    name.insert_str(0, "Hello, ");
    name
}

pub mod people;
