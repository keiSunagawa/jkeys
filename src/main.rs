extern crate jkeys;
use jkeys::visit::keys;
use jkeys::visit::v_funcs::*;
use serde_json::Value;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let json = &args[1];
    let v: Value = serde_json::from_str(&json).expect("aa");
    let ks: Vec<keys::KeysPath> = match v {
        Value::Array(xs) => visit_array(xs),
        Value::Object(m) => {
            let mut xs = m.keys();
            println!("{:?}", xs.next());
            Vec::new()
        }
        _ => {
            println!("invalid input: {}", &json);
            Vec::new()
        }
    };
    println!("done");
    let x: Box<keys::Key> = Box::new(keys::Key::Index(1));
    match *x {
        keys::Key::Index(n) => println!("got n {}", n),
        keys::Key::Key(n) => println!("got n {}", n),
    }
}
