use std::env;
use serde_json::{ Value};

enum Literal {
    Boolean(bool),
    String(String),
    Number(f64),
    Null
}

enum Key {
    Key(String),
    Index(usize)
}
enum KeysPath {
    Next(Key, Box<KeysPath>),
    Value(Literal)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let json = &args[1];
    let v: Value = serde_json::from_str(&json).expect("aa");
    let ks: Vec<KeysPath>= match v {
        Value::Array(xs) => visitArray(xs),
        Value::Object(m) => {
            let mut xs = m.keys();
            println!("{:?}", xs.next());
            Vec::new()
        },
        _ => {
            println!("invalid input: {}", &json);
            Vec::new()
        }
    };
    println!("done");
    let x: Box<Key> = Box::new(Key::Index(1));
    match  *x {
        Key::Index(n) => println!("got n {}", n),
        Key::Key(n) => println!("got n {}", n),
    }
}

fn visit(v: Value) -> KeysPath {
    KeysPath::Value(Literal::Boolean(true))
}

fn visitArray(xs: Vec<Value>) -> Vec<KeysPath> {
    xs.into_iter().enumerate().map(|(i, v)| {
        let child = visit(v);
        KeysPath::Next(Key::Index(i), Box::new(child))
    }).collect()
}
