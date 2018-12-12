#[derive(Debug)]
pub struct Array1<T> {
    inner: Vec<T>,
}
impl<T> Array1<T> {
    pub fn new(inner: Vec<T>) -> Self {
        Self { inner }
    }
}
