pub enum Literal {
    Boolean(bool),
    String(String),
    Number(f64),
    Null,
}

pub enum Key {
    Key(String),
    Index(usize),
}
pub enum KeysPath {
    Next(Key, Box<KeysPath>),
    Value(Literal),
}
