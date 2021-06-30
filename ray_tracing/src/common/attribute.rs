#[derive(Debug, Copy, Clone, Default)]
pub struct Attribute<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T> Attribute<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Attribute { a, b, c }
    }
}
