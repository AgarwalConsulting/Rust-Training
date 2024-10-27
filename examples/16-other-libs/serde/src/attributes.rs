use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]  // <-- this is a container attribute
struct S {
    #[serde(default)]  // <-- this is a field attribute
    f: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "e")]  // <-- this is also a container attribute
enum E {
    #[serde(rename = "a")]  // <-- this is a variant attribute
    A(String),
}

// Implementing a Serialize: https://serde.rs/impl-serializer.html
// Implementing a Deserializer: https://serde.rs/impl-deserializer.html
