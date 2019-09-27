use super::keys;
use serde_json::{Map, Value};

fn visit(v: Value) -> Vec<keys::KeysPath> {
    // KeysPath::Value(Literal::Boolean(true))
    match v {
        Value::Bool(uv) => {
            let l = keys::Literal::Boolean(uv);
            vec![keys::KeysPath::Value(l)]
        }
        Value::String(uv) => {
            let l = keys::Literal::String(uv.to_string());
            vec![keys::KeysPath::Value(l)]
        }
        Value::Number(uv) => {
            let l = keys::Literal::Number(uv.as_f64().unwrap());
            vec![keys::KeysPath::Value(l)]
        }
        Value::Null => vec![keys::KeysPath::Value(keys::Literal::Null)],
        Value::Array(xs) => visit_array(xs),
        Value::Object(mx) => visit_object(mx),
    }
}

pub fn visit_array(xs: Vec<Value>) -> Vec<keys::KeysPath> {
    xs.into_iter()
        .enumerate()
        .into_iter()
        .flat_map(|(i, v)| {
            visit(v).into_iter().map(move |vtd| {
                let i2 = i;
                keys::KeysPath::Next(keys::Key::Index(i2), Box::new(vtd))
            })
        })
        .collect()
}
pub fn visit_object(m: Map<String, Value>) -> Vec<keys::KeysPath> {
    m.into_iter()
        .flat_map(|(k, v)| {
            visit(v)
                .into_iter()
                .map(move |vtd| keys::KeysPath::Next(keys::Key::Key(k.clone()), Box::new(vtd)))
        })
        .collect()
}
